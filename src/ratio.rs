use crate::{BigInt, gcd_bi};

mod arith;
mod comp;
mod convert;
pub mod e;
pub mod funcs;
pub mod ln2;
pub mod pi;

pub use convert::{inspect_ieee754_f32, inspect_ieee754_f64};

// denom is always a positive integer
// when numer is 0, denom is 1
// denom and numer are always coprime
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ratio {
    denom: BigInt,
    numer: BigInt,
}

impl Ratio {

    /// This function DOES NOT check whether `denom` and `numer` are coprime.
    /// Avoid using this function except when converting the result of `Ratio::into_raw` to `Ratio`.
    /// In most cases, it's safer to use `Ratio::from_denom_and_numer`.
    pub fn from_raw(denom: Vec<u32>, denom_neg: bool, numer: Vec<u32>, numer_neg: bool) -> Self {
        Ratio { denom: BigInt::from_raw(denom, denom_neg), numer: BigInt::from_raw(numer, numer_neg) }
    }

    /// (denom, denom_neg, numer, numer_neg)
    pub fn into_raw(self) -> (Vec<u32>, bool, Vec<u32>, bool) {
        let (denom, numer) = (self.denom, self.numer);
        let (denom, denom_neg) = denom.into_raw();
        let (numer, numer_neg) = numer.into_raw();

        (denom, denom_neg, numer, numer_neg)
    }

    pub fn get_denom(&self) -> BigInt {
        self.denom.clone()
    }

    pub fn get_numer(&self) -> BigInt {
        self.numer.clone()
    }

    pub fn zero() -> Self {
        Ratio {
            denom: BigInt::one(),
            numer: BigInt::zero(),
        }
    }

    pub fn one() -> Self {
        Ratio {
            denom: BigInt::one(),
            numer: BigInt::one(),
        }
    }

    pub fn is_neg(&self) -> bool {
        self.numer.is_neg()
    }

    pub fn is_one(&self) -> bool {
        self.denom.is_one() && self.numer.is_one()
    }

    pub fn is_zero(&self) -> bool {
        self.denom.is_one() && self.numer.is_zero()
    }

    pub fn is_integer(&self) -> bool {
        self.denom.is_one()
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
            self.denom.div_mut(&r);
            self.numer.div_mut(&r);
        }

        #[cfg(test)] assert!(self.is_valid());
    }

    // TODO: test this function
    /// It shrinks the size of `self.numer` and `self.denom` until they're less than or equal to `2^(limit * 32)`. It may lose accuracy.
    /// If `denom` and `numer` are already small enough, it returns `Ok(0)`.
    /// If it successfully shrinks, it returns `Ok(n)` where `n` is how much numbers it removed.
    /// Sometimes, the shrinked result doesn't satisfy the limit. It returns `Err(n)` in those cases where `n` is how much numbers it removed.
    pub fn shrink(&mut self, limit: usize) -> Result<usize, usize> {
        if limit < 3 {
            return Err(0);
        }

        let numer_shrink = self.numer.len().max(limit) - limit;
        let denom_shrink = self.denom.len().max(limit) - limit;
        let shrink = numer_shrink.max(denom_shrink);

        if shrink == 0 {
            Ok(0)
        }

        else if shrink + 2 < self.numer.len().min(self.denom.len()) {
            self.numer.shift_right_mut(shrink);
            self.denom.shift_right_mut(shrink);
            self.fit();

            Ok(shrink)
        }

        else if self.numer.len() + 4 < self.denom.len() {
            let shrink = self.numer.len();
            self.denom.div_mut(&self.numer);

            if self.denom.is_neg() {
                self.denom.abs_mut();
                self.numer = BigInt::one().neg();
            }

            else {
                self.numer = BigInt::one();
            }

            if self.denom.len() > limit {
                Err(shrink - 1)
            }

            else {
                Ok(shrink - 1)
            }
        }

        else if self.denom.len() + 4 < self.numer.len() {
            let shrink = self.denom.len();
            self.numer.div_mut(&self.denom);
            self.denom = BigInt::one();

            if self.numer.len() > limit {
                Err(shrink - 1)
            }

            else {
                Ok(shrink - 1)
            }
        }

        else {
            Err(0)
        }
    }
}

impl Default for Ratio {
    fn default() -> Self { Ratio::zero() }
}
