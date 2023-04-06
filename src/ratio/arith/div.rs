use crate::{BigInt, Ratio, gcd_bi};

impl Ratio {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn div_rat(&self, other: &Ratio) -> Self {
        let result = Ratio::from_denom_and_numer(
            self.denom.mul_bi(&other.numer),
            self.numer.mul_bi(&other.denom)
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn div_rat_mut(&mut self, other: &Ratio) {
        self.denom.mul_bi_mut(&other.numer);
        self.numer.mul_bi_mut(&other.denom);
        self.fit();

        #[cfg(test)] assert!(self.is_valid());
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn div_bi(&self, other: &BigInt) -> Self {
        let mut result = self.clone();
        result.div_bi_mut(other);

        result
    }

    pub fn div_bi_mut(&mut self, other: &BigInt) {

        if other.is_zero() {
            panic!("Attempt to divide by zero: {self:?} / {other:?}");
        }

        let r = gcd_bi(&self.numer, other);

        if r.is_one() {
            self.denom.mul_bi_mut(&other);
        }

        else {
            self.denom.mul_bi_mut(&other.div_bi(&r));
            self.numer.div_bi_mut(&r);
        }

        if self.denom.is_neg() {
            self.denom.neg_mut();
            self.numer.neg_mut();
        }

        #[cfg(test)] assert!(self.is_valid());
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn div_i32(&self, other: i32) -> Self {
        let result = Ratio::from_denom_and_numer(
            self.denom.mul_i32(other),
            self.numer.clone()
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn div_i32_mut(&mut self, other: i32) {
        self.denom.mul_i32_mut(other);
        self.fit();

        #[cfg(test)] assert!(self.is_valid());
    }

}