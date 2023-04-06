use crate::{BigInt, Ratio};

impl Ratio {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sub_rat(&self, other: &Ratio) -> Self {
        let result = Ratio::from_denom_and_numer(
            self.denom.mul_bi(&other.denom),
            self.numer.mul_bi(&other.denom).sub_bi(&other.numer.mul_bi(&self.denom))
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn sub_rat_mut(&mut self, other: &Ratio) {
        todo!()
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sub_bi(&self, other: &BigInt) -> Self {

        // Safety: `self.denom` and `self.numer` are already coprime.
        let result = Ratio::from_denom_and_numer_raw(
            self.denom.clone(),
            self.numer.sub_bi(&self.denom.mul_bi(other))
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn sub_bi_mut(&mut self, other: &BigInt) {
        todo!()
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sub_i32(&self, other: i32) -> Self {

        // Safety: `self.denom` and `self.numer` are already coprime.
        let result = Ratio::from_denom_and_numer_raw(
            self.denom.clone(),
            self.numer.sub_bi(&self.denom.mul_i32(other))
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn sub_i32_mut(&mut self, other: i32) {
        todo!()
    }

}