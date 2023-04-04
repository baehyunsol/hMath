use crate::{BigInt, gcd_ubi};

impl BigInt {

    /// It returns 0 when `self` is less than or equal to 0.
    #[must_use]
    pub fn log2(&self) -> Self {
        BigInt::from_ubi(self.val.log2(), false)
    }

    #[must_use]
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

    #[must_use]
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