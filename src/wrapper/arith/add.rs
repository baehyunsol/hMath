use crate::Number;
use std::ops::{Add, AddAssign};


impl Add for &Number {
    type Output = Number;

    fn add(self, other: &Number) -> Number {

        match self {
            Number::Integer(i) => match other {
                Number::Integer(ii) => {
                    let mut result = Number::Integer(i + ii);
                    result.finalize();

                    result
                }
                Number::Ratio(r) => {
                    let mut result = Number::Ratio(r + i);
                    result.finalize();

                    result
                }
            }
            Number::Ratio(r) => match other {
                Number::Integer(i) => {
                    let mut result = Number::Ratio(r + i);
                    result.finalize();

                    result
                }
                Number::Ratio(rr) => {
                    let mut result = Number::Ratio(r + rr);
                    result.finalize();

                    result
                }
            }
        }

    }

}


impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        &self + &other
    }

}


impl Add<&Number> for Number {
    type Output = Number;

    fn add(self, other: &Number) -> Number {
        &self + other
    }

}


impl Add<Number> for &Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        self + &other
    }

}


impl Add<i32> for &Number {
    type Output = Number;

    fn add(self, other: i32) -> Number {
        match self {
            Number::Integer(i) => {
                let mut result = Number::Integer(i + other);
                result.finalize();

                result
            }
            Number::Ratio(r) => {
                let mut result = Number::Ratio(r + other);
                result.finalize();

                result
            }
        }
    }

}


impl Add<i32> for Number {
    type Output = Number;

    fn add(self, other: i32) -> Number {
        match self {
            Number::Integer(i) => {
                let mut result = Number::Integer(&i + other);
                result.finalize();

                result
            }
            Number::Ratio(r) => {
                let mut result = Number::Ratio(&r + other);
                result.finalize();

                result
            }
        }
    }

}


impl AddAssign for Number {
    fn add_assign(&mut self, other: Self) {
        *self = &(*self) + &other;
    }
}


impl AddAssign<i32> for Number {
    fn add_assign(&mut self, other: i32) {
        *self = &(*self) + other;
    }
}
