use crate::BigInt;

impl BigInt {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn div(&self, other: &BigInt) -> Self {
        let val = self.val.div(&other.val);
        let is_neg = !val.is_zero() && self.is_neg() != other.is_neg();
 
        let result = BigInt::from_ubi(val, is_neg);

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn div_mut(&mut self, other: &BigInt) {
        self.val.div_mut(&other.val);
        self._is_neg = !self.val.is_zero() && self.is_neg() != other.is_neg();
        #[cfg(test)] assert!(self.is_valid());
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn div_i32(&self, other: i32) -> Self {
        let val = self.val.div_u32(other.abs() as u32);
        let is_neg = !val.is_zero() && self.is_neg() != (other < 0);

        let result = BigInt::from_ubi(val, is_neg);

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn div_i32_mut(&mut self, other: i32) {
        self.val.div_u32_mut(other.abs() as u32);
        self._is_neg = !self.val.is_zero() && self.is_neg() != (other < 0);
        #[cfg(test)] assert!(self.is_valid());
    }
}

#[cfg(test)]
mod tests {
    use crate::BigInt;

    #[test]
    fn sign_test() {
        for x in -7..8 {
            for y in -7..8 {
                if y == 0 {
                    continue;
                }

                let mut x1 = BigInt::from_i32(x);
                let y1 = BigInt::from_i32(y);
                let mut x2 = BigInt::from_i32(x);
                let res1 = x1.div(&y1);
                let res2 = x1.div_i32(y);
                let res3 = BigInt::from_i32(x / y);
                x1.div_mut(&y1);
                x2.div_i32_mut(y);

                assert_eq!(x1, x2);
                assert_eq!(res1, res2);
                assert_eq!(res2, res3);
                assert_eq!(res1, x1);
                assert_eq!(res2, x2);
            }
        }
    }
}
