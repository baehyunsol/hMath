use crate::big_int::{BigInt, funcs::gcd};

mod arith;
mod comp;
mod conv;

/*
Always in most reduced form
denom is always positive
*/
#[derive(Clone, PartialEq)]
pub struct Ratio {
    denom: BigInt,
    numer: BigInt
}


impl Ratio {

    fn new(denom: BigInt, numer: BigInt) -> Ratio {
        let mut result = Ratio {denom, numer};
        result.div_gcd();

        result
    }

    #[inline]
    pub fn zero() -> Ratio {
        Ratio::new(BigInt::one(), BigInt::zero())
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.numer.is_zero() && !self.denom.is_zero()
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        self.numer.is_negative
    }

    pub fn abs(&self) -> Ratio {

        if self.numer.is_negative {
            Ratio {
                denom: self.denom.clone(),
                numer: -&self.numer
            }
        }

        else {
            self.clone()
        }

    }

    pub fn reci(&self) -> Ratio {

        if self.numer.is_zero() {
            panic!("Denominator cannot be 0");
        }

        let mut result = Ratio {
            denom: self.numer.clone(),
            numer: self.denom.clone(),
        };

        if result.denom.is_negative {
            result.denom.is_negative = false;
            result.numer.is_negative = !result.numer.is_negative;
        }

        result
    }

    pub fn floor(&self) -> BigInt {
        &self.numer / &self.denom
    }

    fn div_gcd(&mut self) {

        if self.numer.is_zero() {
            *self = Ratio::zero();
        }

        if self.denom.is_zero() {
            panic!("Denominator cannot be 0");
        }

        if self.denom.is_negative {
            self.denom.is_negative = false;
            self.numer.is_negative = !self.numer.is_negative;
        }

        let r = gcd(self.denom.abs(), self.numer.abs());

        if r != 1 {
            self.denom = &self.denom / &r;
            self.numer = &self.numer / &r;
        }

    }
}