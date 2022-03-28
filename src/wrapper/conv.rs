use crate::{Number, Ratio, BigInt};


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

    pub fn to_string(&self) -> String {

        match self {
            Number::Integer(i) => i.to_string(),
            _ => "Not Implemented".to_string()
        }

    }

    pub fn from_u32(n: u32) -> Number {
        Number::Integer(BigInt::from_u32(n))
    }

    pub fn from_i32(n: i32) -> Number {
        Number::Integer(BigInt::from_i32(n))
    }

    pub fn from_u64(n: u64) -> Number {
        Number::Integer(BigInt::from_u64(n))
    }

    pub fn from_i64(n: i64) -> Number {
        Number::Integer(BigInt::from_i64(n))
    }
}