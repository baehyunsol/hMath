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

}