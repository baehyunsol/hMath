use crate::{Number, BigInt, Ratio};
use std::ops::{Div, DivAssign};


impl Div for &Number {
    type Output = Number;

    fn div(self, other: &Number) -> Number {

        match self {
            Number::Integer(i) => match other {
                Number::Integer(ii) => {
                    let mut result = Number::Integer(i / ii);
                    result.finalize();

                    result
                }
                Number::Ratio(r) => {
                    let mut result = Number::Ratio(i / r);
                    result.finalize();

                    result
                }
            }
            Number::Ratio(r) => match other {
                Number::Integer(i) => {
                    let mut result = Number::Ratio(r / i);
                    result.finalize();

                    result
                }
                Number::Ratio(rr) => {
                    let mut result = Number::Ratio(r / rr);
                    result.finalize();

                    result
                }
            }
        }

    }

}


impl Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        &self / &other
    }

}


impl Div<&Number> for Number {
    type Output = Number;

    fn div(self, other: &Number) -> Number {
        &self / other
    }

}


impl Div<Number> for &Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        self / &other
    }

}


impl Div<i32> for &Number {
    type Output = Number;

    fn div(self, other: i32) -> Number {
        match self {
            Number::Integer(i) => {
                let mut result = Number::Ratio(Ratio::new(BigInt::from_i32(other), i.clone()));
                result.finalize();

                result
            }
            Number::Ratio(r) => {
                let mut result = Number::Ratio(r / other);
                result.finalize();

                result
            }
        }
    }

}


impl Div<i32> for Number {
    type Output = Number;

    fn div(self, other: i32) -> Number {
        match self {
            Number::Integer(i) => {
                let mut result = Number::Ratio(Ratio::new(BigInt::from_i32(other), i));
                result.finalize();

                result
            }
            Number::Ratio(r) => {
                let mut result = Number::Ratio(&r / other);
                result.finalize();

                result
            }
        }
    }

}


impl DivAssign for Number {
    fn div_assign(&mut self, other: Self) {
        *self = &(*self) / &other;
    }
}


impl DivAssign<i32> for Number {
    fn div_assign(&mut self, other: i32) {
        *self = &(*self) / other;
    }
}
