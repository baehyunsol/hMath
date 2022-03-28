mod add;
mod div;
mod mul;
mod rem;
mod sub;


use crate::{Number, Ratio, BigInt};


impl Number {

    pub fn abs(&self) -> Self {
        let mut result = match self {
            Number::Integer(i) => Number::Integer(i.abs()),
            Number::Ratio(r) => Number::Ratio(r.abs())
        };
        result.finalize();

        result
    }

    pub fn pow(&self, n: i32) -> Self {

        if n < 0 {
            return self.reci().pow(-n);
        }

        let mut result = match self {
            Number::Integer(i) => Number::Integer(i.pow(n as u32)),
            Number::Ratio(r) =>Number::Ratio(r.pow(n as u32))
        };
        result.finalize();

        result
    }

    pub fn reci(&self) -> Self {
        let mut result = match self {
            Number::Integer(i) => Number::Ratio(Ratio::new(i.clone(), BigInt::one())),
            Number::Ratio(r) => Number::Ratio(r.reci())
        };
        result.finalize();

        result
    }

    pub fn floor(&self) -> Self {
        let mut result = match self {
            Number::Integer(_) => self.clone(),
            Number::Ratio(r) => Number::Integer(r.to_big_int())
        };
        result.finalize();

        result
    }

    pub fn frac(&self) -> Self {
        let mut result = match self {
            Number::Integer(_) => Number::Integer(BigInt::zero()),
            Number::Ratio(r) => Number::Ratio(r.frac())
        };
        result.finalize();

        result
    }

    #[inline]
    pub fn is_integer(&self) -> bool {
        match self {
            Number::Integer(_) => true,
            _ => false
        }
    }

}