use crate::{BigInt, Ratio};

impl Ratio {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add(&self, other: &Ratio) -> Self {
        let result = Ratio::from_denom_and_numer(
            self.denom.mul(&other.denom),
            self.denom.mul(&other.numer).add(&other.denom.mul(&self.numer)),
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn add_mut(&mut self, other: &Ratio) {
        self.numer = self.denom.mul(&other.numer).add(&other.denom.mul(&self.numer));
        self.denom.mul_mut(&other.denom);

        self.fit();
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add_bi(&self, other: &BigInt) -> Self {

        // Safety: `self.denom` and `self.numer` are already coprime.
        let result = Ratio::from_denom_and_numer_raw(
            self.denom.clone(),
            self.numer.add(&self.denom.mul(other)),
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn add_bi_mut(&mut self, other: &BigInt) {
        self.numer.add_mut(&self.denom.mul(other));
        #[cfg(test)] assert!(self.is_valid());
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add_i32(&self, other: i32) -> Self {

        // Safety: `self.denom` and `self.numer` are already coprime.
        let result = Ratio::from_denom_and_numer_raw(
            self.denom.clone(),
            self.numer.add(&self.denom.mul_i32(other)),
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn add_i32_mut(&mut self, other: i32) {
        self.numer.add_mut(&self.denom.mul_i32(other));
        #[cfg(test)] assert!(self.is_valid());
    }
}
