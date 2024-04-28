use crate::{BigInt, UBigInt};

impl BigInt {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add(&self, other: &BigInt) -> Self {
        if self.is_neg() == other.is_neg() {
            BigInt::from_ubi(self.val.add(&other.val), self.is_neg())
        }

        else {
            let self_less = self.val.lt(&other.val);
            let abs_diff = if self_less {
                other.val.sub(&self.val)
            } else {
                self.val.sub(&other.val)
            };
            let is_diff_zero = abs_diff.is_zero();

            // self + other   self_less    self.is_neg()   diff.val == 0    result.is_neg()
            // (-3) + 4         true          true             false           false
            // (-4) + 3         false         true             false           true
            // 3    + (-4)      true          false            false           true
            // 4    + (-3)      false         false            false           false
            // (-3) + 3         false         true             true            false
            // 3    + (-3)      false         false            true            false

            let result = BigInt::from_ubi(abs_diff, (self_less ^ self.is_neg()) & !is_diff_zero);

            #[cfg(test)] assert!(result.is_valid());

            result
        }
    }

    pub fn add_mut(&mut self, other: &BigInt) {
        if self.is_neg() == other.is_neg() {
            self.val.add_mut(&other.val);
        }

        else {
            let self_less = self.val.lt(&other.val);
            if self_less {
                self.val = other.val.sub(&self.val);
            } else {
                self.val.sub_mut(&other.val);
            }
            let is_diff_zero = self.val.is_zero();

            self._is_neg = (self_less ^ self.is_neg()) & !is_diff_zero;

            #[cfg(test)] assert!(self.is_valid());
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add_i32(&self, other: i32) -> Self {
        if self.is_neg() == (other < 0) {
            BigInt::from_ubi(self.val.add_u32(other.abs() as u32), self.is_neg())
        }

        else {
            let other_abs = other.abs() as u32;
            let self_less = self.val.lt_u32(other_abs);
            let abs_diff = if self_less {
                UBigInt::from_u32(other_abs - self.val.to_u32().unwrap())
            } else {
                self.val.sub_u32(other_abs)
            };
            let is_diff_zero = abs_diff.is_zero();

            let result = BigInt::from_ubi(abs_diff, (self_less ^ self.is_neg()) & !is_diff_zero);

            #[cfg(test)] assert!(result.is_valid());

            result
        }
    }

    pub fn add_i32_mut(&mut self, other: i32) {
        if self.is_neg() == (other < 0) {
            self.val.add_u32_mut(other.abs() as u32);
        }

        else {
            let other_abs = other.abs() as u32;
            let self_less = self.val.lt_u32(other_abs);
            if self_less {
                self.val = UBigInt::from_u32(other_abs - self.val.to_u32().unwrap());
            } else {
                self.val.sub_u32_mut(other_abs);
            }
            let is_diff_zero = self.val.is_zero();

            self._is_neg = (self_less ^ self.is_neg()) & !is_diff_zero;
            #[cfg(test)] assert!(self.is_valid());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BigInt;

    #[test]
    fn sign_test() {
        for x in -7..8 {
            for y in -7..8 {
                let mut x1 = BigInt::from_i32(x);
                let y1 = BigInt::from_i32(y);
                let mut x2 = BigInt::from_i32(x);
                let res1 = x1.add(&y1);
                let res2 = x1.add_i32(y);
                let res3 = BigInt::from_i32(x + y);
                x1.add_mut(&y1);
                x2.add_i32_mut(y);

                assert_eq!(x1, x2);
                assert_eq!(res1, res2);
                assert_eq!(res2, res3);
                assert_eq!(res1, x1);
                assert_eq!(res2, x2);
            }
        }
    }
}
