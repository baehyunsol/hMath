mod add;
mod sub;
mod mul;
mod div;
mod rem;

use crate::BigInt;

impl BigInt {

    #[inline]
    pub fn abs(&self) -> BigInt {

        if self.is_negative {
            -self
        }

        else {
            self.clone()
        }

    }

    pub fn pow(&self, n: u32) -> BigInt {

        if self.is_zero() {
            self.clone()
        }

        else if *self == 1 {
            self.clone()
        }

        else if n < 5 {

            if n == 0 {
                BigInt::one()
            }

            else if n == 1 {
                self.clone()
            }

            else {
                panic!("Not Implemented!")
            }

        }

        else {
            panic!("Not Implemented!")
        }

    }

}