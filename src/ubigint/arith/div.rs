use crate::UBigInt;
use crate::utils::{remove_suffix_0, v64_to_v32};

impl UBigInt {

    // TODO: the code is too messy
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn div(&self, other: &UBigInt) -> Self {
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
                approx.add(&self.sub(&other.mul(&approx)).div(&other))
            }

            else if other.len() == 2 {
                let approx = approx1.shift_left(self.len() - other.len() - 3);

                // self / other = approx + (self - other * approx) / other
                approx.add(&self.sub(&other.mul(&approx1).shift_left(self.len() - other.len() - 3)).div(&other))
            }

            else {
                // approx1 <= answer <= approx2
                let approx2 = div_approx(self_approx.to_vec(), other_approx);

                if approx2.eq(&approx1) {
                    let approx = approx2.shift_left(self.len() - other.len() - 3);

                    // self / other = approx + (self - other * approx) / other
                    approx.add(&self.sub(&other.mul(&approx2).shift_left(self.len() - other.len() - 3)).div(&other))
                }

                else {
                    // if other[-3] is small enough, the answer is close to approx2
                    // if other[-3] is big enough, the answer is close to approx1
                    let ratio = ((other.0[other.len() - 3] as u64 >> 24) + 1) as u32;  // 1 ~ 256
                    let approx3 = approx1.mul_u32(ratio).add(&approx2.mul_u32(256 - ratio)).div_u32(256);

                    let approx = approx3.shift_left(self.len() - other.len() - 3);

                    // self / other = approx + (self - other * approx) / other
                    let tmp = other.mul(&approx3).shift_left(self.len() - other.len() - 3);

                    if self.geq(&tmp) {
                        let tmp2 = self.sub(&tmp).div(&other);
                        approx.add(&tmp2)
                    }

                    else {
                        let tmp2 = tmp.sub(self).div(&other);

                        // `approx4` can either be `answer` or `answer + 1`
                        let approx4 = approx.sub(&tmp2);

                        if approx4.mul(&other).leq(self) {
                            approx4
                        }

                        else {
                            approx4.sub_u32(1)
                        }
                    }
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
                approx.add(&self.sub(&other.mul(&approx)).div(&other))
            }

            else {
                if other.gt(self) {
                    UBigInt::zero()
                }

                else {
                    UBigInt::one()
                }
            }
        }
    }

    pub fn div_mut(&mut self, other: &UBigInt) {
        let result = self.div(other);
        *self = result;
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn div_u32(&self, other: u32) -> Self {
        let mut result = self.clone();
        result.div_u32_mut(other);

        #[cfg(test)] {
            // infinite recursion
            /*let t = self.div(&UBigInt::from_u32(other));
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

    #[test]
    fn div_test() {

        // some errors I found at version 0.1.2
        let a = UBigInt::from_raw(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2483027968, 3512055093, 4, 2147483648, 3062546468, 155332645, 2941532299, 4114844189, 3767121251, 1544740476, 844338915, 935339935, 3084816902, 203387259, 306982644, 3480067718, 1071893785, 2115388466, 2769176492, 3819576256, 4073585814, 330041009, 1729468812, 2690227286, 1815449054, 4293438911, 3491709174, 2268335433, 3968017720, 3970688095, 3097051230, 2252455378, 3163133863, 1986523563, 3937260367, 3766236303, 3818926048, 3553676019, 2179178614, 3839100814, 2182920117, 892846200, 1325643434, 3036627424, 2915485122, 1492428706, 2153583434, 3300996263, 770654547, 1925732922, 3528307859, 3009917878, 1372067140, 699931523, 2834606653, 2179510738, 665066584, 1275488545, 3200593841, 999893524, 3893413080, 3003081881, 807324679, 580485371, 1406565171, 2726975953, 2829224488, 3538213070, 2524684375, 918555361, 1811995392, 323295904, 3983103947, 4018682879, 411287072, 1774654195, 43272453]);
        let b = UBigInt::from_raw(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1073741824, 1262726521, 10, 0, 2147483648, 1026242066, 4294967247, 4294967295, 3489660927, 380647447, 110, 0, 0, 2415193643, 4294967138, 4294967295, 2286944255, 3817205550, 159, 0, 1501560832, 1992232541, 4294967173, 4294967295, 2954887167, 2208560584, 73, 0, 3323985920, 2804761636, 4294967260, 4294967295, 1261535231, 3570889685, 13, 0, 3487105024, 2373060436, 4294967291, 4294967295, 1307435007, 779244781, 1, 0, 4221042688, 3179582868, 4294967295, 4294967295, 798314559, 202506890, 0, 0, 1582836480, 4264767166, 4294967295, 4294967295, 2011715967, 3663017, 0, 0, 1334120256, 4294611535, 4294967295, 1073741823, 3237333065, 27021, 0, 2147483648, 3864550132, 4294965751, 4294967295, 3489660927, 1449589813, 64]);
        let c = a.div(&b);

        assert!(c.mul(&b).leq(&a));
        assert!(c.add_u32(1).mul(&b).gt(&a));

        let a = UBigInt::from_raw(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2553449472, 4281327939, 4294967295, 4294967295, 1399384063, 3591542347, 159072862, 2545165805, 527440336, 2062341239, 3426184737, 3279348248, 1529467276, 2620848218, 1847692483, 3793275959, 4053734128, 901204618, 3793275959, 562873206, 983541142, 3461436925, 3426184737, 3279348248, 538679980, 972778435, 1847692483, 3793275959, 1910263116, 2626743040, 3793275959, 3314336630, 2624020821, 1865019127, 3426184737, 2339824152, 2771754496, 2243055, 39285201, 3799457039, 4287582387, 2351642286, 955081196, 1591090889, 2476000135, 3641727336, 1642894222, 973093101, 159163715, 3358906358, 2475611678, 1282761196, 2343327230, 2556076412, 2160686051, 2241674809, 783892778, 2536100748, 1944295436, 1544519881, 1881883790, 965046065, 2550961982, 6531043, 2983011493, 1948099110, 1105781803, 321321655, 296766602, 2386747389, 3828053022, 3103626984, 272204766, 3074548430, 1948159554, 3598074923, 788820056, 89518937, 1432299785, 3573936330, 1181485167, 4069560383, 688457045, 3914797399, 414524473, 2421514027, 89518736, 4073668525, 2183531897, 3028180570, 1206248874, 3183401165, 3901154029, 1583000391, 27964542, 855136786, 2861701818, 2873615845, 3430833754, 1148152955, 3551856958, 993722377, 3870294173, 3262911038, 2952827717, 644019, 10304315, 164869040, 2637904640, 3551768576, 9]);
        let b = UBigInt::from_raw(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3654598656, 1269152120, 2454267026, 613566756, 1885016649, 2171030485, 613566756, 1227133513, 3022509202, 1535551975, 1227133513, 2454267026, 813517092, 3592606343, 2454267025, 613566756, 424211433, 1075133960, 613566757, 1227133513, 3171151634, 1520341354, 1227133512, 2454267026, 3706059556, 79861292, 2454267027, 613566756, 186291625, 667821732, 1840700269, 460175067, 831578165, 3370573438, 3681400539, 920350134, 823065394, 2855088467, 3067833782, 1035393901, 4150095564, 3449744855, 1840700269, 3681400539, 3707146307, 1688558869, 3681400539, 2303421878, 338634434, 3733910393, 3067833782, 3996572525, 1942971214, 3052077816, 1840700269, 2177742555, 3402581779, 4118327, 0, 1130364928, 3375176419, 4294028999, 4294967295, 297222143, 851142247, 186209, 0, 3380838400, 732747133, 4294935173, 4294967295, 2398244863, 172967866, 4800, 0, 2402680832, 4091895407, 4294966677, 4294967295, 4081593759, 312866562, 68, 0, 1626142592, 2791083695, 4294967289, 4294967295, 1965846079, 2125933645, 0, 0, 754431008, 4159046948, 4294967295, 3221225471, 2635492477, 6943046, 0, 2147483648, 2844173929, 4294695019, 4294967295, 805306367, 537422958, 7854]);
        let c = a.div(&b);

        assert!(c.mul(&b).leq(&a));
        assert!(c.add_u32(1).mul(&b).gt(&a));

        if !RUN_ALL_TESTS { return; }
        assert_eq!(
            UBigInt::from_string("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap().div(&UBigInt::from_string("1_0000_0000_0000_0000_0000_0000").unwrap()),
            UBigInt::from_string("1_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap().div(&UBigInt::from_string("316227766016837933199889").unwrap()),
            UBigInt::from_string("316227766016837933199889").unwrap(),
        );
        assert_eq!(
            UBigInt::from_u32(8).pow_u32(888).div(&UBigInt::from_u32(8).pow_u32(886)),
            UBigInt::from_u32(64),
        );
        assert_eq!(
            UBigInt::from_u32(7).pow_u32(777).div(&UBigInt::from_u32(7).pow_u32(775)),
            UBigInt::from_u32(49),
        );
        assert_eq!(
            UBigInt::from_u32(6).pow_u32(666).div(&UBigInt::from_u32(6).pow_u32(664)),
            UBigInt::from_u32(36),
        );
        assert_eq!(
            UBigInt::from_raw(vec![1, 2, 3, 4, 5, 5, 5]).div(&UBigInt::from_raw(vec![1, 1, 1, 1, 5, 5, 5])),
            UBigInt::one(),
        );
        assert_eq!(
            UBigInt::from_raw(vec![u32::MAX; 18]).div(&UBigInt::from_raw(vec![u32::MAX; 8])),
            UBigInt::from_string("2135987035920910082395021706169552114602704522356652769947041607822219725780658996767035796488192").unwrap(),
        );

        for a in 6..12 {
            for b in (a + 1)..18 {
                assert_eq!(
                    UBigInt::from_u32(17).pow_u32(b * 300).div(&UBigInt::from_u32(17).pow_u32(a * 300)),
                    UBigInt::from_u32(17).pow_u32((b - a) * 300),
                );
            }
        }
    }
}
