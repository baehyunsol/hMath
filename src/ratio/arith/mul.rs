use crate::{BigInt, Ratio, gcd_bi};

impl Ratio {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul(&self, other: &Ratio) -> Self {
        let result = Ratio::from_denom_and_numer(
            self.denom.mul(&other.denom),
            self.numer.mul(&other.numer)
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn mul_mut(&mut self, other: &Ratio) {
        self.denom.mul_mut(&other.denom);
        self.numer.mul_mut(&other.numer);
        self.fit();

        #[cfg(test)] assert!(self.is_valid());
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul_bi(&self, other: &BigInt) -> Self {
        let mut result = self.clone();
        result.mul_bi_mut(other);

        result
    }

    pub fn mul_bi_mut(&mut self, other: &BigInt) {
        let r = gcd_bi(&self.denom, other);

        if r.is_one() {
            self.numer.mul_mut(other);
        }

        else {
            self.numer.mul_mut(&other.div(&r));
            self.denom.div_mut(&r);
        }

        #[cfg(test)] assert!(self.is_valid());
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul_i32(&self, other: i32) -> Self {
        let result = Ratio::from_denom_and_numer(
            self.denom.clone(),
            self.numer.mul_i32(other)
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn mul_i32_mut(&mut self, other: i32) {
        self.numer.mul_i32_mut(other);
        self.fit();

        #[cfg(test)] assert!(self.is_valid());
    }
}
