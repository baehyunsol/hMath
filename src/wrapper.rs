use crate::BigInt;
use crate::Ratio;


mod arith;
mod comp;
mod conv;


#[derive(Clone, PartialEq, Debug)]
pub enum Number {
    Integer(BigInt),
    Ratio(Ratio)
}


impl Number {

    pub fn zero() -> Number {
        Number::Integer(BigInt::zero())
    }

    fn finalize(&mut self) {
        match self {
            Number::Integer(_) => {},
            Number::Ratio(r) => {

                if r.denom == 1 {
                    *self = Number::Integer(r.numer.clone());
                }

            }
        }
    }

}


impl std::default::Default for Number {

    #[inline]
    fn default() -> Number {
        Number::zero()
    }

}