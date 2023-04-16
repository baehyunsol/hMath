use crate::UBigInt;
use crate::consts::{U64_32, U128_32, U128_64};
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
            let other_approx = other.0[other.len() - 1] as u128 * U128_32 + other.0[other.len() - 2] as u128 + 1;
            let mut curr = self.0[self.len() - 1] as u128 * U128_64 + self.0[self.len() - 2] as u128 * U128_32 + self.0[self.len() - 3] as u128;
            let mut result = vec![];
            let mut index = self.len() - 4;

            /*
             *           index
             *             |
             *             |
             *   self      V
             *       a a a b b b b c c c
             *   other
             *         a a b b b b
             *
             *   a: approximation
             *   b: other.len() - 2
             *   c: self.len() - other.len() - 1
             *
             *   self / other ~= (self >> b) / other_approx
             *   has to run the below loop ((self.len() - other.len() + 1) / 2) times
             */
            for _ in 0..((self.len() - other.len() + 1) / 2) {
                result.push((curr / other_approx) as u64);
                result.push(0);  // `result` is 32bit-based, but `curr` is 64bit-based.
                curr %= other_approx;
                curr <<= 64;
                curr += self.0[index] as u128 * U128_32 + self.0[index - 1] as u128;
                index -= 2;
            }

            if (self.len() - other.len()) % 2 == 1 {
                result.pop().unwrap();
            }

            result.reverse();
            let approx = UBigInt::from_raw(v64_to_v32(result));

            // self / other = approx + (self - other * approx) / other
            approx.add_ubi(&self.sub_ubi(&other.mul_ubi(&approx)).div_ubi(&other))
        }

        else {
            let self_approx = self.0[self.len() - 1] as u64 * U64_32 + self.0[self.len() - 2] as u64;
            let other_approx = other.0[other.len() - 1] as u64 * U64_32 + other.0[other.len() - 2] as u64;

            if self_approx < other_approx {
                UBigInt::zero()
            }

            else if self_approx > other_approx {
                // TODO: what if `other_approx == U64::MAX`?
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
            carry = curr % other * U64_32;
        }

        remove_suffix_0(&mut self.0);

        #[cfg(test)] assert!(self.is_valid());
    }

}

#[cfg(test)]
mod tests {
    use crate::UBigInt;
    use crate::consts::RUN_ALL_TESTS;

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
            todo!()
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