use crate::big_int::{BigInt, funcs::gcd};

mod arith;
mod comp;
mod conv;

/*
Always in most reduced form
denom is always positive
*/
#[derive(Clone, PartialEq, Debug)]
pub struct Ratio {
    pub denom: BigInt,
    pub numer: BigInt
}


impl Ratio {

    pub fn new(denom: BigInt, numer: BigInt) -> Ratio {
        let mut result = Ratio {denom, numer};
        result.div_gcd();

        result
    }

    #[inline]
    pub fn zero() -> Ratio {
        Ratio {
            denom: BigInt::one(),
            numer: BigInt::zero()
        }
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

    #[inline]
    pub fn floor(&self) -> Ratio {
        Ratio::from_big_int(&self.numer / &self.denom)
    }

    #[inline]
    pub fn frac(&self) -> Ratio {
        self - &self.floor()
    }

    fn div_gcd(&mut self) {

        if self.numer.is_zero() {
            *self = Ratio::zero();
            return;
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


#[cfg(test)]
mod tests {
    #[test]
    fn frac_test() {
        use crate::Ratio;

        assert_eq!(Ratio::from_string("3.1415".to_string()).unwrap().frac(), Ratio::from_string("0.1415".to_string()).unwrap());
    }
}