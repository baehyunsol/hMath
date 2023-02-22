use crate::Number;
use std::ops::{Sub, SubAssign, Neg};

impl Sub for &Number {
    type Output = Number;

    fn sub(self, other: &Number) -> Number {

        match self {
            Number::Integer(i) => match other {
                Number::Integer(ii) => {
                    let mut result = Number::Integer(i - ii);
                    result.finalize();

                    result
                }
                Number::Ratio(r) => {
                    let mut result = Number::Ratio(i - r);
                    result.finalize();

                    result
                }
            }
            Number::Ratio(r) => match other {
                Number::Integer(i) => {
                    let mut result = Number::Ratio(r - i);
                    result.finalize();

                    result
                }
                Number::Ratio(rr) => {
                    let mut result = Number::Ratio(r - rr);
                    result.finalize();

                    result
                }
            }
        }

    }

}

impl Sub for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        &self - &other
    }

}

impl Sub<&Number> for Number {
    type Output = Number;

    fn sub(self, other: &Number) -> Number {
        &self - other
    }

}

impl Sub<Number> for &Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        self - &other
    }

}

impl Sub<i32> for &Number {
    type Output = Number;

    fn sub(self, other: i32) -> Number {
        match self {
            Number::Integer(i) => {
                let mut result = Number::Integer(i - other);
                result.finalize();

                result
            }
            Number::Ratio(r) => {
                let mut result = Number::Ratio(r - other);
                result.finalize();

                result
            }
        }
    }

}

impl Sub<i32> for Number {
    type Output = Number;

    fn sub(self, other: i32) -> Number {
        match self {
            Number::Integer(i) => {
                let mut result = Number::Integer(&i - other);
                result.finalize();

                result
            }
            Number::Ratio(r) => {
                let mut result = Number::Ratio(&r - other);
                result.finalize();

                result
            }
        }
    }

}

impl Neg for &Number {
    type Output = Number;

    fn neg(self) -> Number {
        match self {
            Number::Integer(i) => {
                let mut result = Number::Integer(-i);
                result.finalize();

                result
            }
            Number::Ratio(r) => {
                let mut result = Number::Ratio(-r);
                result.finalize();

                result
            }
        }
    }

}

impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Number {
        -&self
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, other: Self) {
        *self = &(*self) - &other;
    }
}

impl SubAssign<i32> for Number {
    fn sub_assign(&mut self, other: i32) {
        *self = &(*self) - other;
    }
}
