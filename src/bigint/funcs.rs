use crate::{BigInt, gcd_ubi};

impl BigInt {

    /// It returns 0 when `self` is 0.
    /// It returns `log2(abs(self))`.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2(&self) -> Self {
        BigInt::from_ubi(self.val.log2(), false)
    }

    /// It returns `truncate(log2(self) * 16777216)`.
    /// Warning: This function is very expensive.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2_accurate(&self) -> Self {
        BigInt::from_ubi(self.val.log2_accurate(), false)
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn neg(&self) -> Self {
        //   self.is_neg()      self.val.is_zero()       result
        //       true                  false             false
        //       false                 true              false
        //       false                 false             true
        BigInt::from_ubi(self.val.clone(), !self.is_neg() & !self.val.is_zero())
    }

    pub fn neg_mut(&mut self) {
        self._is_neg = !self.is_neg() & !self.val.is_zero();
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn abs(&self) -> Self {
        BigInt::from_ubi(self.val.clone(), false)
    }

    pub fn abs_mut(&mut self) {
        self._is_neg = false;
    }

}

pub fn gcd_bi(a: &BigInt, b: &BigInt) -> BigInt {
    BigInt::from_ubi(gcd_ubi(&a.val, &b.val), false)
}