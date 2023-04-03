use crate::{BigInt, Ratio};

impl Ratio {

    #[must_use]
    pub fn add_rat(&self, other: &Ratio) -> Self {
        Ratio::from_denom_and_numer(
            self.denom.mul_bi(&other.denom),
            self.denom.mul_bi(&other.numer).add_bi(&other.denom.mul_bi(&self.numer))
        )
    }

    pub fn add_rat_mut(&mut self, other: &Ratio) {
        todo!()
    }

    #[must_use]
    pub fn add_bi(&self, other: &BigInt) -> Self {
        todo!()
    }

    pub fn add_bi_mut(&mut self, other: &BigInt) {
        todo!()
    }

    #[must_use]
    pub fn add_i32(&self, other: i32) -> Self {
        todo!()
    }

    pub fn add_i32_mut(&mut self, other: i32) {
        todo!()
    }

}