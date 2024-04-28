use crate::{
    BigInt,
    UBigInt,
    gcd_ubi,
};

impl BigInt {

    /// It returns `2^n`
    pub fn exp2(n: u64) -> Self {
        BigInt::from_ubi(UBigInt::exp2(n), false)
    }

    /// It returns 0 when `self` is 0.
    /// It returns `log2(abs(self))`.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2(&self) -> Self {
        BigInt::from_ubi(self.val.log2(), false)
    }

    /// It returns `truncate(log2(abs(self)) * 4294967296)`. It returns 0 when `self` is 0.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2_accurate(&self) -> Self {
        BigInt::from_ubi(self.val.log2_accurate(), false)
    }

    /// It returns (sign * sqrt(abs(self)))
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

    pub fn factorial(n: u32) -> Self {
        BigInt::from_ubi(UBigInt::factorial(n), false)
    }

    pub fn fibonacci(n: u32) -> Self {
        BigInt::from_ubi(UBigInt::fibonacci(n), false)
    }

    /// divide by 2^32
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn shift_right(&self, n: usize) -> Self {
        let new_val = self.val.shift_right(n);
        let is_neg = !new_val.is_zero() && self.is_neg();

        BigInt::from_ubi(new_val, is_neg)
    }

    /// divide by 2^32
    pub fn shift_right_mut(&mut self, n: usize) {
        self.val.shift_right_mut(n);
        self._is_neg = !self.val.is_zero() && self.is_neg();
    }

    /// multiply 2^32
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn shift_left(&self, n: usize) -> Self {
        BigInt::from_ubi(self.val.shift_left(n), self.is_neg())
    }

    /// multiply 2^32
    pub fn shift_left_mut(&mut self, n: usize) {
        self.val.shift_left_mut(n);
    }

    /// modulo 2^32 (it doesn't care about sign)\
    /// It panics when `n` is too big. It returns 0 when `n` is 0.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn slice_right(&self, n: usize) -> Self {
        let new_val = self.val.slice_right(n);
        let is_neg = !new_val.is_zero() && self.is_neg();

        BigInt::from_ubi(new_val, is_neg)
    }

    /// modulo 2^32 (it doesn't care about sign)\
    /// It panics when `n` is too big. It makes `self` 0 when `n` is 0.
    pub fn slice_right_mut(&mut self, n: usize) {
        self.val.slice_right_mut(n);
        self._is_neg = !self.val.is_zero() && self.is_neg();
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
