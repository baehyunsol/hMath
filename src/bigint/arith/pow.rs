use crate::{BigInt, UBigInt};

impl BigInt {

    /// 0^0 is 1
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn pow_u32(&self, exp: u32) -> Self {
        BigInt::from_ubi(self.val.pow_u32(exp), self.is_neg() && exp % 2 == 1)
    }

    pub fn pow_u32_mut(&mut self, exp: u32) {
        self.val.pow_u32_mut(exp);
        self._is_neg = self.is_neg() && exp % 2 == 1;
    }

    /// returns 2^exp
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn pow2(exp: u32) -> Self {
        BigInt::from_ubi(UBigInt::pow2(exp), false)
    }
}
