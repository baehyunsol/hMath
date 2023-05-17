use crate::UBigInt;
use crate::utils::{remove_suffix_0, v64_to_v32};

impl UBigInt {

    // TODO: the code is too messy
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn div_ubi(&self, other: &UBigInt) -> Self {

        if self.len() < other.len() {
            UBigInt::zero()
        }

        else if self.len() < 5 && other.len() < 5 {
            UBigInt::from_u128(self.to_u128().unwrap() / other.to_u128().unwrap())
        }

        else if other.len() == 1 {
            self.div_u32(other.0[0])
        }

        else if self.len() > other.len() {
            let other_approx = ((other.0[other.len() - 1] as u64) << 32) + other.0[other.len() - 2] as u64;
            let self_approx = &self.0[(self.len() - 5)..];

            let approx1 = if other_approx == u64::MAX {
                UBigInt::from_raw(self_approx[2..].to_vec())
            } else {
                div_approx(self_approx.to_vec(), other_approx + 1)
            };

            if self.len() - other.len() < 3 {
                let approx = approx1.shift_right(3 + other.len() - self.len());

                // self / other = approx + (self - other * approx) / other
                approx.add_ubi(&self.sub_ubi(&other.mul_ubi(&approx)).div_ubi(&other))
            }

            else if other.len() == 2 {
                let approx = approx1.shift_left(self.len() - other.len() - 3);

                // self / other = approx + (self - other * approx) / other
                approx.add_ubi(&self.sub_ubi(&other.mul_ubi(&approx1).shift_left(self.len() - other.len() - 3)).div_ubi(&other))
            }

            else {
                // approx1 <= answer <= approx2
                let approx2 = div_approx(self_approx.to_vec(), other_approx);

                if approx2.eq_ubi(&approx1) {
                    let approx = approx2.shift_left(self.len() - other.len() - 3);

                    // self / other = approx + (self - other * approx) / other
                    approx.add_ubi(&self.sub_ubi(&other.mul_ubi(&approx2).shift_left(self.len() - other.len() - 3)).div_ubi(&other))
                }

                else {
                    // if other[-3] is small enough, the answer is close to approx2
                    // if other[-3] is big enough, the answer is close to approx1
                    let ratio = ((other.0[other.len() - 3] as u64 >> 24) + 1) as u32;  // 1 ~ 256
                    let approx3 = approx1.mul_u32(ratio).add_ubi(&approx2.mul_u32(256 - ratio)).div_u32(256);
                    let approx = approx3.shift_left(self.len() - other.len() - 3);

                    // TODO: `1` is supposed to be greater than `2`, but it's slightly smaller
                    // see `extra_div_test` below
                    // println!("1: {:?}", self);
                    // println!("2: {:?}", other.mul_ubi(&approx3).shift_left(self.len() - other.len() - 3));
                    // println!("3: {:?}", self.sub_ubi(&other.mul_ubi(&approx3).shift_left(self.len() - other.len() - 3)));

                    // self / other = approx + (self - other * approx) / other
                    approx.add_ubi(&self.sub_ubi(&other.mul_ubi(&approx3).shift_left(self.len() - other.len() - 3)).div_ubi(&other))
                }

            }

        }

        else {
            let self_approx = ((self.0[self.len() - 1] as u64) << 32) + self.0[self.len() - 2] as u64;
            let other_approx = ((other.0[other.len() - 1] as u64) << 32) + other.0[other.len() - 2] as u64;

            if self_approx < other_approx {
                UBigInt::zero()
            }

            else if self_approx > other_approx {
                let approx = UBigInt::from_u64(self_approx / (other_approx + 1));

                // self / other = approx + (self - other * approx) / other
                approx.add_ubi(&self.sub_ubi(&other.mul_ubi(&approx)).div_ubi(&other))
            }

            else {

                if other.gt_ubi(self) {
                    UBigInt::zero()
                }

                else {
                    UBigInt::one()
                }

            }

        }

    }

    pub fn div_ubi_mut(&mut self, other: &UBigInt) {
        let result = self.div_ubi(other);
        *self = result;
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn div_u32(&self, other: u32) -> Self {
        let mut result = self.clone();
        result.div_u32_mut(other);

        #[cfg(test)] {
            // infinite recursion
            /*let t = self.div_ubi(&UBigInt::from_u32(other));
            assert_eq!(t, result);*/

            assert!(result.is_valid());
        }

        result
    }

    pub fn div_u32_mut(&mut self, other: u32) {
        let mut carry = 0;
        let other = other as u64;

        for n in self.0.iter_mut().rev() {
            let curr = *n as u64 + carry;
            *n = (curr / other) as u32;
            carry = (curr % other) << 32;
        }

        remove_suffix_0(&mut self.0);

        #[cfg(test)] assert!(self.is_valid());
    }

}

fn div_approx(divend: Vec<u32>, divisor: u64) -> UBigInt {
    let mut curr = ((divend[4] as u128) << 64) + ((divend[3] as u128) << 32) + divend[2] as u128;
    let quotient1 = (curr / divisor as u128) as u64;
    curr %= divisor as u128;
    curr <<= 64;
    curr += ((divend[1] as u128) << 32) + divend[0] as u128;
    let quotient2 = (curr / divisor as u128) as u64;

    UBigInt::from_raw(v64_to_v32(vec![quotient2, 0, quotient1]))
}

#[cfg(test)]
mod tests {
    use crate::UBigInt;
    use crate::consts::RUN_ALL_TESTS;

    // TODO: it panics: see TODO above
    #[test]
    fn extra_div_test() {
        let x = UBigInt::from_raw(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1811939328, 782912202, 4294967291, 4294967295, 3623878655, 586435810, 226050933, 3616814565, 2084789840, 2627350416, 653035911, 1858640818, 1015659298, 2002791228, 2310742712, 2612143852, 1260220104, 3719786676, 577685584, 653035963, 892378034, 2601340122, 502335413, 3742398404, 1661303873, 4056681275, 2579850830, 2622908181, 1711567193, 3780890215, 1718704556, 1729468869, 1308787798, 4056368048, 2935073717, 4011506630, 1549151338, 798435541, 2881252081, 3150360305, 650055962, 958562816, 1794054843, 2935073724, 3911269318, 1713265479, 1880169476, 17940548, 2776115523, 203037491, 470042369, 3225710609, 3359209057, 1162329116, 1191252416, 1880169476, 2737362604, 285334439, 297813104, 470042369, 4073959709, 71928693, 1148195100, 3003191744, 4030333345, 17927901, 287048775, 3384820848, 3292296293, 3225711331, 71762193, 1735397660, 715744561, 1880169446, 17940548, 287048775, 297813104, 470042369, 3225710609, 273]);
        let y = UBigInt::from_raw(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1073741824, 1262726521, 10, 0, 2147483648, 1026242066, 4294967247, 4294967295, 3489660927, 380647447, 110, 0, 0, 2415193643, 4294967138, 4294967295, 2286944255, 3817205550, 159, 0, 1501560832, 1992232541, 4294967173, 4294967295, 2954887167, 2208560584, 73, 0, 3323985920, 2804761636, 4294967260, 4294967295, 1261535231, 3570889685, 13, 0, 3487105024, 2373060436, 4294967291, 4294967295, 1307435007, 779244781, 1, 0, 4221042688, 3179582868, 4294967295, 4294967295, 798314559, 202506890, 0, 0, 1582836480, 4264767166, 4294967295, 4294967295, 2011715967, 3663017, 0, 0, 1334120256, 4294611535, 4294967295, 1073741823, 3237333065, 27021, 0, 2147483648, 3864550132, 4294965751, 4294967295, 3489660927, 1449589813, 64]);

        println!("{}", x.div_ubi(&y));
    }

    #[test]
    fn div_test() {
        if !RUN_ALL_TESTS { return; }
        assert_eq!(
            UBigInt::from_string("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap().div_ubi(&UBigInt::from_string("1_0000_0000_0000_0000_0000_0000").unwrap()),
            UBigInt::from_string("1_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap()
        );
        assert_eq!(
            UBigInt::from_string("1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap().div_ubi(&UBigInt::from_string("316227766016837933199889").unwrap()),
            UBigInt::from_string("316227766016837933199889").unwrap()
        );
        assert_eq!(
            UBigInt::from_u32(8).pow_u32(888).div_ubi(&UBigInt::from_u32(8).pow_u32(886)),
            UBigInt::from_u32(64)
        );
        assert_eq!(
            UBigInt::from_u32(7).pow_u32(777).div_ubi(&UBigInt::from_u32(7).pow_u32(775)),
            UBigInt::from_u32(49)
        );
        assert_eq!(
            UBigInt::from_u32(6).pow_u32(666).div_ubi(&UBigInt::from_u32(6).pow_u32(664)),
            UBigInt::from_u32(36)
        );
        assert_eq!(
            UBigInt::from_raw(vec![1, 2, 3, 4, 5, 5, 5]).div_ubi(&UBigInt::from_raw(vec![1, 1, 1, 1, 5, 5, 5])),
            UBigInt::one()
        );
        assert_eq!(
            UBigInt::from_raw(vec![u32::MAX; 18]).div_ubi(&UBigInt::from_raw(vec![u32::MAX; 8])),
            UBigInt::from_string("2135987035920910082395021706169552114602704522356652769947041607822219725780658996767035796488192").unwrap()
        );

        for a in 6..12 {

            for b in (a + 1)..18 {
                assert_eq!(
                    UBigInt::from_u32(17).pow_u32(b * 300).div_ubi(&UBigInt::from_u32(17).pow_u32(a * 300)),
                    UBigInt::from_u32(17).pow_u32((b - a) * 300)
                );
            }

        }

    }

}
