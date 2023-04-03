use crate::{BigInt, Ratio};

impl Ratio {

    #[must_use]
    pub fn mul_rat(&self, other: &Ratio) -> Self {
        let result = Ratio::from_denom_and_numer(
            self.denom.mul_bi(&other.denom),
            self.numer.mul_bi(&other.numer)
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn mul_rat_mut(&mut self, other: &Ratio) {
        todo!()
    }

    #[must_use]
    pub fn mul_bi(&self, other: &BigInt) -> Self {
        todo!()
    }

    pub fn mul_bi_mut(&mut self, other: &BigInt) {
        todo!()
    }

    #[must_use]
    pub fn mul_i32(&self, other: i32) -> Self {
        todo!()
    }

    pub fn mul_i32_mut(&mut self, other: i32) {
        todo!()
    }

}