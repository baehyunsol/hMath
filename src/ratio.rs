use crate::{BigInt, gcd_bi};

mod arith;
mod convert;
mod funcs;

// denom is always a positive integer
// when numer is 0, denom is 1
// denom and numer are always coprime
#[derive(Clone, Debug, PartialEq)]
pub struct Ratio {
    denom: BigInt,
    numer: BigInt
}

impl Ratio {

    /// This function DOES NOT check whether `denom` and `numer` are coprime.
    /// Avoid using this function except when converting the result of `Ratio::into_raw` to `Ratio`.
    /// In most cases, it's safer to use `Ratio::from_denom_and_numer`.
    pub fn from_raw(denom: Vec<u32>, denom_neg: bool, numer: Vec<u32>, numer_neg: bool) -> Self {
        Ratio { denom: BigInt::from_raw(denom, denom_neg), numer: BigInt::from_raw(numer, numer_neg) }
    }

    pub fn into_raw(self) -> (Vec<u32>, bool, Vec<u32>, bool) {
        let (denom, numer) = (self.denom, self.numer);
        let (denom, denom_neg) = denom.into_raw();
        let (numer, numer_neg) = numer.into_raw();

        (denom, denom_neg, numer, numer_neg)
    }

    pub fn zero() -> Self {
        Ratio {
            denom: BigInt::one(),
            numer: BigInt::zero()
        }
    }

    pub fn one() -> Self {
        Ratio {
            denom: BigInt::one(),
            numer: BigInt::one()
        }
    }

    #[cfg(test)]
    pub fn is_valid(&self) -> bool {
        self.denom.is_valid() && self.numer.is_valid() && !self.denom.is_neg() && (!self.numer.is_zero() || self.denom.is_one()) && gcd_bi(&self.denom, &self.numer).is_one()
    }

    // TODO: better name
    fn fit(&mut self) {

        if self.denom.is_neg() {
            self.denom.neg_mut();
            self.numer.neg_mut();
        }

        let r = gcd_bi(&self.denom, &self.numer);

        if !r.is_one() {
            self.denom.div_bi_mut(&r);
            self.numer.div_bi_mut(&r);
        }

    }

}