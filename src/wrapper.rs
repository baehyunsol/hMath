use crate::BigInt;
use crate::Ratio;


mod arith;
mod comp;


#[derive(Clone)]
pub enum Number {
    Integer(BigInt),
    Ratio(Ratio)
}


impl Number {

    pub fn from_string(string: String) -> Result<Number, ()> {

        match BigInt::from_string(string.clone()) {
            Ok(i) => {
                let mut result = Number::Integer(i);
                result.finalize();

                return Ok(result);
            }
            _ => {}
        }

        match Ratio::from_string(string.clone()) {
            Ok(r) => {
                let mut result = Number::Ratio(r);
                result.finalize();

                return Ok(result);
            }
            _ => {}
        }

        return Err(());
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