use crate::{BigInt, Ratio};

impl Ratio {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sub(&self, other: &Ratio) -> Self {
        let result = Ratio::from_denom_and_numer(
            self.denom.mul(&other.denom),
            self.numer.mul(&other.denom).sub(&other.numer.mul(&self.denom)),
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn sub_mut(&mut self, other: &Ratio) {
        self.numer = self.numer.mul(&other.denom).sub(&other.numer.mul(&self.denom));
        self.denom.mul_mut(&other.denom);

        self.fit();
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sub_bi(&self, other: &BigInt) -> Self {

        // Safety: `self.denom` and `self.numer` are already coprime.
        let result = Ratio::from_denom_and_numer_raw(
            self.denom.clone(),
            self.numer.sub(&self.denom.mul(other)),
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn sub_bi_mut(&mut self, other: &BigInt) {
        self.numer.sub_mut(&self.denom.mul(other));
        #[cfg(test)] assert!(self.is_valid());
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sub_i32(&self, other: i32) -> Self {

        // Safety: `self.denom` and `self.numer` are already coprime.
        let result = Ratio::from_denom_and_numer_raw(
            self.denom.clone(),
            self.numer.sub(&self.denom.mul_i32(other)),
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn sub_i32_mut(&mut self, other: i32) {
        self.numer.sub_mut(&self.denom.mul_i32(other));
        #[cfg(test)] assert!(self.is_valid());
    }
}
