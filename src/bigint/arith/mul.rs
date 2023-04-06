use crate::BigInt;

impl BigInt {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul_bi(&self, other: &BigInt) -> Self {
        let val = self.val.mul_ubi(&other.val);
        let is_neg = !val.is_zero() && self.is_neg() != other.is_neg();
 
        let result = BigInt::from_ubi(val, is_neg);

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn mul_bi_mut(&mut self, other: &BigInt) {
        self.val.mul_ubi_mut(&other.val);
        self._is_neg = !self.val.is_zero() && self.is_neg() != other.is_neg();
        #[cfg(test)] assert!(self.is_valid());
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul_i32(&self, other: i32) -> Self {
        let val = self.val.mul_u32(other.abs() as u32);
        let is_neg = !val.is_zero() && self.is_neg() != (other < 0);

        let result = BigInt::from_ubi(val, is_neg);

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn mul_i32_mut(&mut self, other: i32) {
        self.val.mul_u32_mut(other.abs() as u32);
        self._is_neg = !self.val.is_zero() && self.is_neg() != (other < 0);
        #[cfg(test)] assert!(self.is_valid());
    }

    /// returns `self * 2^exp`
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul_pow2(&self, exp: u32) -> Self {
        BigInt::from_ubi(self.val.mul_pow2(exp), self.is_neg())
    }

    /// self *= 2^exp
    pub fn mul_pow2_mut(&mut self, exp: u32) {
        self.val.mul_pow2_mut(exp);
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
                let res1 = x1.mul_bi(&y1);
                let res2 = x1.mul_i32(y);
                let res3 = BigInt::from_i32(x * y);
                x1.mul_bi_mut(&y1);
                x2.mul_i32_mut(y);

                assert_eq!(x1, x2);
                assert_eq!(res1, res2);
                assert_eq!(res2, res3);
                assert_eq!(res1, x1);
                assert_eq!(res2, x2);
            }

        }

    }

}