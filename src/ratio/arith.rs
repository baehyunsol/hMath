use crate::{Ratio, BigInt};
use std::ops::{Add, Sub, Mul, Div, Neg};

/*
Ratio::new(4, 2) -> (1 / 2)
Ratio {4, 2} -> (2 / 4)

use `Ratio {}` only when it's already reduced form
*/


impl Add for &Ratio {
    type Output = Ratio;

    fn add(self, other: &Ratio) -> Ratio {
        Ratio::new(
            &self.denom * &other.denom,
            &(&self.denom * &other.numer) + &(&self.numer * &other.denom)
        )
    }

}


impl Add<&BigInt> for &Ratio {
    type Output = Ratio;

    fn add(self, other: &BigInt) -> Ratio {
        Ratio {
            denom: self.denom.clone(),
            numer: &self.numer + &(other * &self.denom)
        }
    }

}


impl Add<u32> for &Ratio {
    type Output = Ratio;

    fn add(self, other: u32) -> Ratio {
        Ratio {
            denom: self.denom.clone(),
            numer: &self.numer + &(&self.denom * other)
        }
    }

}


impl Sub for &Ratio {
    type Output = Ratio;

    fn sub(self, other: &Ratio) -> Ratio {
        Ratio::new(
            &self.denom * &other.denom,
            &(&self.numer * &other.denom) - &(&self.denom * &other.numer)
        )
    }

}


impl Sub<&BigInt> for &Ratio {
    type Output = Ratio;

    fn sub(self, other: &BigInt) -> Ratio {
        Ratio {
            denom: self.denom.clone(),
            numer: &self.numer - &(other * &self.denom)
        }
    }

}


impl Sub<u32> for &Ratio {
    type Output = Ratio;

    fn sub(self, other: u32) -> Ratio {
        Ratio {
            denom: self.denom.clone(),
            numer: &self.numer - &(&self.denom * other)
        }
    }

}


impl Mul for &Ratio {
    type Output = Ratio;

    fn mul(self, other: &Ratio) -> Ratio {
        Ratio::new(
            &self.denom * &other.denom,
            &self.numer * &other.numer
        )
    }

}


impl Mul<&BigInt> for &Ratio {
    type Output = Ratio;

    fn mul(self, other: &BigInt) -> Ratio {
        Ratio::new (
            self.denom.clone(),
            &self.numer * other
        )
    }

}


impl Mul<u32> for &Ratio {
    type Output = Ratio;

    fn mul(self, other: u32) -> Ratio {
        Ratio::new (
            self.denom.clone(),
            &self.numer * other
        )
    }

}


impl Div for &Ratio {
    type Output = Ratio;

    fn div(self, other: &Ratio) -> Ratio {
        Ratio::new(
            &self.denom * &other.numer,
            &self.numer * &other.denom
        )
    }

}


impl Div<&BigInt> for &Ratio {
    type Output = Ratio;

    fn div(self, other: &BigInt) -> Ratio {
        Ratio::new (
            &self.denom * other,
            self.numer.clone()
        )
    }

}


impl Div<u32> for &Ratio {
    type Output = Ratio;

    fn div(self, other: u32) -> Ratio {
        Ratio::new (
            &self.denom * other,
            self.numer.clone()
        )
    }

}


impl Neg for &Ratio {
    type Output = Ratio;

    fn neg(self) -> Ratio {

        if self.numer.is_zero() {
            self.clone()
        }

        else {
            Ratio {
                denom: self.denom.clone(),
                numer: -&self.numer
            }
        }

    }

}