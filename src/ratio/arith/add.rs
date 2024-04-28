use crate::{BigInt, Ratio};

impl Ratio {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add_rat(&self, other: &Ratio) -> Self {
        let result = Ratio::from_denom_and_numer(
            self.denom.mul_bi(&other.denom),
            self.denom.mul_bi(&other.numer).add_bi(&other.denom.mul_bi(&self.numer)),
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn add_rat_mut(&mut self, other: &Ratio) {
        self.numer = self.denom.mul_bi(&other.numer).add_bi(&other.denom.mul_bi(&self.numer));
        self.denom.mul_bi_mut(&other.denom);

        self.fit();
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add_bi(&self, other: &BigInt) -> Self {

        // Safety: `self.denom` and `self.numer` are already coprime.
        let result = Ratio::from_denom_and_numer_raw(
            self.denom.clone(),
            self.numer.add_bi(&self.denom.mul_bi(other)),
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn add_bi_mut(&mut self, other: &BigInt) {
        self.numer.add_bi_mut(&self.denom.mul_bi(other));
        #[cfg(test)] assert!(self.is_valid());
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add_i32(&self, other: i32) -> Self {

        // Safety: `self.denom` and `self.numer` are already coprime.
        let result = Ratio::from_denom_and_numer_raw(
            self.denom.clone(),
            self.numer.add_bi(&self.denom.mul_i32(other)),
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn add_i32_mut(&mut self, other: i32) {
        self.numer.add_bi_mut(&self.denom.mul_i32(other));
        #[cfg(test)] assert!(self.is_valid());
    }
}
