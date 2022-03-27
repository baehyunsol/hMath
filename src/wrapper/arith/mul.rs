use crate::Number;
use std::ops::{Mul, MulAssign};


impl Mul for &Number {
    type Output = Number;

    fn mul(self, other: &Number) -> Number {

        match self {
            Number::Integer(i) => match other {
                Number::Integer(ii) => {
                    let mut result = Number::Integer(i * ii);
                    result.finalize();

                    result
                }
                Number::Ratio(r) => {
                    let mut result = Number::Ratio(r * i);
                    result.finalize();

                    result
                }
            }
            Number::Ratio(r) => match other {
                Number::Integer(i) => {
                    let mut result = Number::Ratio(r * i);
                    result.finalize();

                    result
                }
                Number::Ratio(rr) => {
                    let mut result = Number::Ratio(r * rr);
                    result.finalize();

                    result
                }
            }
        }

    }

}


impl Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        &self * &other
    }

}


impl Mul<&Number> for Number {
    type Output = Number;

    fn mul(self, other: &Number) -> Number {
        &self * other
    }

}


impl Mul<Number> for &Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        self * &other
    }

}


impl Mul<i32> for &Number {
    type Output = Number;

    fn mul(self, other: i32) -> Number {
        match self {
            Number::Integer(i) => {
                let mut result = Number::Integer(i * other);
                result.finalize();

                result
            }
            Number::Ratio(r) => {
                let mut result = Number::Ratio(r * other);
                result.finalize();

                result
            }
        }
    }

}


impl Mul<i32> for Number {
    type Output = Number;

    fn mul(self, other: i32) -> Number {
        match self {
            Number::Integer(i) => {
                let mut result = Number::Integer(&i * other);
                result.finalize();

                result
            }
            Number::Ratio(r) => {
                let mut result = Number::Ratio(&r * other);
                result.finalize();

                result
            }
        }
    }

}


impl MulAssign for Number {
    fn mul_assign(&mut self, other: Self) {
        *self = &(*self) * &other;
    }
}


impl MulAssign<i32> for Number {
    fn mul_assign(&mut self, other: i32) {
        *self = &(*self) * other;
    }
}
