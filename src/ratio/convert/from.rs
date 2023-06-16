use crate::{Ratio, BigInt, UBigInt, ConversionError};
use crate::{impl_from_for_ref, impl_tryfrom_for_ref};

macro_rules! impl_from_ref_ratio {
    ($t: ty) => (
        impl_from_for_ref!(Ratio, $t);
    );
    ($t: ty, $($u: ty), +) => (
        impl_from_ref_ratio!($t);
        impl_from_ref_ratio!($($u),+);
    )
}

macro_rules! impl_tryfrom_ref_ratio {
    ($t: ty) => (
        impl_tryfrom_for_ref!(Ratio, $t);
    );
    ($t: ty, $($u: ty), +) => (
        impl_tryfrom_ref_ratio!($t);
        impl_tryfrom_ref_ratio!($($u),+);
    )
}

impl_from_ref_ratio!(bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_tryfrom_ref_ratio!(f32, f64);

impl From<bool> for Ratio {
    fn from(b: bool) -> Self {
        if b {
            Ratio::one()
        } else {
            Ratio::zero()
        }
    }
}

impl TryFrom<f32> for Ratio {
    type Error = ConversionError;

    fn try_from(n: f32) -> Result<Self, Self::Error> {
        Ratio::from_ieee754_f32(n)
    }
}

impl TryFrom<f64> for Ratio {
    type Error = ConversionError;

    fn try_from(n: f64) -> Result<Self, Self::Error> {
        Ratio::from_ieee754_f64(n)
    }
}

impl From<i8> for Ratio {
    fn from(n: i8) -> Self {
        Ratio::from_i32(n as i32)
    }
}

impl From<i16> for Ratio {
    fn from(n: i16) -> Self {
        Ratio::from_i32(n as i32)
    }
}

impl From<i32> for Ratio {
    fn from(n: i32) -> Self {
        Ratio::from_i32(n)
    }
}

impl From<i64> for Ratio {
    fn from(n: i64) -> Self {
        Ratio::from_i64(n)
    }
}

impl From<i128> for Ratio {
    fn from(n: i128) -> Self {
        Ratio::from_i128(n)
    }
}

impl From<isize> for Ratio {
    fn from(n: isize) -> Self {
        Ratio::from_i64(n as i64)
    }
}

impl From<u8> for Ratio {
    fn from(n: u8) -> Self {
        Ratio::from_i32(n as i32)
    }
}

impl From<u16> for Ratio {
    fn from(n: u16) -> Self {
        Ratio::from_i32(n as i32)
    }
}

impl From<u32> for Ratio {
    fn from(n: u32) -> Self {
        Ratio::from_i64(n as i64)
    }
}

impl From<u64> for Ratio {
    fn from(n: u64) -> Self {
        Ratio::from_bi(BigInt::from_ubi(UBigInt::from_u64(n), false))
    }
}

impl From<u128> for Ratio {
    fn from(n: u128) -> Self {
        Ratio::from_bi(BigInt::from_ubi(UBigInt::from_u128(n), false))
    }
}

impl From<usize> for Ratio {
    fn from(n: usize) -> Self {
        Ratio::from_bi(BigInt::from_ubi(UBigInt::from_u64(n as u64), false))
    }
}

impl TryFrom<String> for Ratio {
    type Error = ConversionError;

    fn try_from(n: String) -> Result<Self, Self::Error> {
        Ratio::from_string(&n)
    }
}

impl TryFrom<&str> for Ratio {
    type Error = ConversionError;

    fn try_from(n: &str) -> Result<Self, Self::Error> {
        Ratio::from_string(n)
    }
}

impl From<BigInt> for Ratio {
    fn from(n: BigInt) -> Self {
        Ratio::from_bi(n)
    }
}

impl From<UBigInt> for Ratio {
    fn from(n: UBigInt) -> Self {
        Ratio::from_ubi(n)
    }
}