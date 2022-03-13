use crate::{Ratio, BigInt};

impl Ratio {

    #[inline]
    pub fn from_u32s(denom: u32, numer: u32) -> Ratio {
        Ratio::new(BigInt::from_u32(denom), BigInt::from_u32(numer))
    }

    #[inline]
    pub fn from_u32(n: u32) -> Ratio {
        Ratio {denom: BigInt::one(), numer: BigInt::from_u32(n)}
    }

    #[inline]
    pub fn from_i32s(denom: i32, numer: i32) -> Ratio {
        Ratio::new(BigInt::from_i32(denom), BigInt::from_i32(numer))
    }

    #[inline]
    pub fn from_i32(n: i32) -> Ratio {
        Ratio {denom: BigInt::one(), numer: BigInt::from_i32(n)}
    }

    #[inline]
    pub fn from_big_int(n: BigInt) -> Ratio {
        Ratio {denom: BigInt::one(), numer: n}
    }

    #[inline]
    pub fn from_big_ints(denom: BigInt, numer: BigInt) -> Ratio {
        Ratio::new(denom, numer)
    }

}