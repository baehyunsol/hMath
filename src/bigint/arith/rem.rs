use crate::BigInt;

impl BigInt {

    // self - self / other * other
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn rem_bi(&self, other: &BigInt) -> Self {
        let mut sdo = self.div_bi(other);
        sdo.mul_bi_mut(other);

        let result = self.sub_bi(&sdo);

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn rem_bi_mut(&mut self, other: &BigInt) {
        let mut sdo = self.div_bi(other);
        sdo.mul_bi_mut(other);
        self.sub_bi_mut(&sdo);
        #[cfg(test)] assert!(self.is_valid());
    }

    // x % y = x % -y
    // x % y = -(-x % y)
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn rem_i32(&self, other: i32) -> Self {
        let new_val = self.val.rem_u32(other.abs() as u32);
        let is_neg = self.is_neg() & !new_val.is_zero();

        let result = BigInt::from_ubi(new_val, is_neg);

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn rem_i32_mut(&mut self, other: i32) {
        self.val.rem_u32_mut(other.abs() as u32);
        self._is_neg = self.is_neg() & !self.val.is_zero();
        #[cfg(test)] assert!(self.is_valid());
    }

    /// `other` must be a power of 2
    // TODO: write a unit test
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn rem_pow2(&self, other: i32) -> Self {
        let new_val = self.val.rem_pow2(other.abs() as u32);
        let is_neg = self.is_neg() & !new_val.is_zero();

        BigInt::from_ubi(new_val, is_neg)
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
                let res1 = x1.rem_bi(&y1);
                let res2 = x1.rem_i32(y);
                let res3 = BigInt::from_i32(x % y);
                x1.rem_bi_mut(&y1);
                x2.rem_i32_mut(y);

                assert_eq!(x1, x2);
                assert_eq!(res1, res2);
                assert_eq!(res2, res3);
                assert_eq!(res1, x1);
                assert_eq!(res2, x2);
            }

        }

    }

}
