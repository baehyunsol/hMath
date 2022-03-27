use crate::Number;
use std::ops::{Rem, RemAssign};


impl Rem for &Number {
    type Output = Number;

    fn rem(self, other: &Number) -> Number {

        match self {
            Number::Integer(i) => match other {
                Number::Integer(ii) => {
                    let mut result = Number::Integer(i % ii);
                    result.finalize();

                    result
                }
                Number::Ratio(r) => {
                    let mut result = Number::Ratio(i % r);
                    result.finalize();

                    result
                }
            }
            Number::Ratio(r) => match other {
                Number::Integer(i) => {
                    let mut result = Number::Ratio(r % i);
                    result.finalize();

                    result
                }
                Number::Ratio(rr) => {
                    let mut result = Number::Ratio(r % rr);
                    result.finalize();

                    result
                }
            }
        }

    }

}


impl Rem for Number {
    type Output = Number;

    fn rem(self, other: Number) -> Number {
        &self % &other
    }

}


impl Rem<&Number> for Number {
    type Output = Number;

    fn rem(self, other: &Number) -> Number {
        &self % other
    }

}


impl Rem<Number> for &Number {
    type Output = Number;

    fn rem(self, other: Number) -> Number {
        self % &other
    }

}


impl Rem<i32> for &Number {
    type Output = Number;

    fn rem(self, other: i32) -> Number {
        match self {
            Number::Integer(i) => {
                let mut result = Number::Integer(i % other);
                result.finalize();

                result
            }
            Number::Ratio(r) => {
                let mut result = Number::Ratio(r % other);
                result.finalize();

                result
            }
        }
    }

}


impl Rem<i32> for Number {
    type Output = Number;

    fn rem(self, other: i32) -> Number {
        match self {
            Number::Integer(i) => {
                let mut result = Number::Integer(&i % other);
                result.finalize();

                result
            }
            Number::Ratio(r) => {
                let mut result = Number::Ratio(&r % other);
                result.finalize();

                result
            }
        }
    }

}


impl RemAssign for Number {
    fn rem_assign(&mut self, other: Self) {
        *self = &(*self) % &other;
    }
}


impl RemAssign<i32> for Number {
    fn rem_assign(&mut self, other: i32) {
        *self = &(*self) % other;
    }
}
