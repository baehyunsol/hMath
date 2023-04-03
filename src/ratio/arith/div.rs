use crate::{BigInt, Ratio};

impl Ratio {

    #[must_use]
    pub fn div_rat(&self, other: &Ratio) -> Self {
        let result = Ratio::from_denom_and_numer(
            self.denom.mul_bi(&other.numer),
            self.numer.mul_bi(&other.denom)
        );

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn div_rat_mut(&mut self, other: &Ratio) {
        todo!()
    }

    #[must_use]
    pub fn div_bi(&self, other: &BigInt) -> Self {
        todo!()
    }

    pub fn div_bi_mut(&mut self, other: &BigInt) {
        todo!()
    }

    #[must_use]
    pub fn div_i32(&self, other: i32) -> Self {
        todo!()
    }

    pub fn div_i32_mut(&mut self, other: i32) {
        todo!()
    }

}