use crate::{BigInt, gcd_ubi};

#[cfg(feature = "rand")]
use crate::UBigInt;

impl BigInt {

    /// It returns 0 when `self` is 0.
    /// It returns `log2(abs(self))`.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2(&self) -> Self {
        BigInt::from_ubi(self.val.log2(), false)
    }

    /// It returns `truncate(log2(self.abs()) * 1073741824)`. It returns 0 when `self` is 0.
    /// Warning: This function is very expensive.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2_accurate(&self) -> Self {
        BigInt::from_ubi(self.val.log2_accurate(), false)
    }

    /// It returns (sign * sqrt(self.abs()))
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sqrt(&self) -> Self {
        BigInt::from_ubi(self.val.sqrt(), self.is_neg())
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

    /// Sign of `self` doesn't make any difference
    pub fn is_prime(&self) -> bool {
        self.val.is_prime()
    }

    /// Sign of `self` doesn't make any difference
    pub fn prime_factorial(&self) -> Vec<Self> {
        self.val.prime_factorial().into_iter().map(
            |n| BigInt::from_ubi(n, false)
        ).collect()
    }

    /// It returns a random number whose absolute value is between 1..2^(32 * scale). It returns a negative number by 50%.\
    /// If `scale` is 0, it returns 0.
    #[cfg(feature = "rand")]
    pub fn random(scale: usize) -> Self {

        if scale == 0 {
            BigInt::zero()
        }

        else {
            BigInt::from_ubi(UBigInt::random(scale), rand::random::<bool>())
        }

    }

}

pub fn gcd_bi(a: &BigInt, b: &BigInt) -> BigInt {
    BigInt::from_ubi(gcd_ubi(&a.val, &b.val), false)
}