use crate::{Ratio, BigInt, UBigInt, ConversionError};
use crate::{impl_from_for_ref, impl_tryfrom_for_ref};

macro_rules! impl_from_ref_ubigint {
    ($t: ty) => (
        impl_from_for_ref!(UBigInt, $t);
    );
    ($t: ty, $($u: ty), +) => (
        impl_from_ref_ubigint!($t);
        impl_from_ref_ubigint!($($u),+);
    )
}

macro_rules! impl_tryfrom_ref_ubigint {
    ($t: ty) => (
        impl_tryfrom_for_ref!(UBigInt, $t);
    );
    ($t: ty, $($u: ty), +) => (
        impl_tryfrom_ref_ubigint!($t);
        impl_tryfrom_ref_ubigint!($($u),+);
    )
}

impl_from_ref_ubigint!(bool, u8, u16, u32, u64, u128, usize);
impl_tryfrom_ref_ubigint!(f32, f64, i8, i16, i32, i64, i128, isize);

impl From<bool> for UBigInt {
    fn from(b: bool) -> Self {
        if b {
            UBigInt::one()
        } else {
            UBigInt::zero()
        }
    }
}

/// It returns the truncated value of `Ratio::from(n)`.
impl TryFrom<f32> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: f32) -> Result<Self, Self::Error> {
        Ok(Ratio::try_from(n)?.truncate_bi().try_into()?)
    }
}

/// It returns the truncated value of `Ratio::from(n)`.
impl TryFrom<f64> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: f64) -> Result<Self, Self::Error> {
        Ok(Ratio::try_from(n)?.truncate_bi().try_into()?)
    }
}

impl TryFrom<i8> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: i8) -> Result<Self, Self::Error> {
        Ok(UBigInt::from_u32(u32::try_from(n)?))
    }
}

impl TryFrom<i16> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: i16) -> Result<Self, Self::Error> {
        Ok(UBigInt::from_u32(u32::try_from(n)?))
    }
}

impl TryFrom<i32> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        Ok(UBigInt::from_u32(u32::try_from(n)?))
    }
}

impl TryFrom<i64> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: i64) -> Result<Self, Self::Error> {
        Ok(UBigInt::from_u64(u64::try_from(n)?))
    }
}

impl TryFrom<i128> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: i128) -> Result<Self, Self::Error> {
        Ok(UBigInt::from_u128(u128::try_from(n)?))
    }
}

impl TryFrom<isize> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: isize) -> Result<Self, Self::Error> {
        Ok(UBigInt::from_u64(u64::try_from(n)?))
    }
}

impl From<u8> for UBigInt {
    fn from(n: u8) -> Self {
        UBigInt::from_u32(n as u32)
    }
}

impl From<u16> for UBigInt {
    fn from(n: u16) -> Self {
        UBigInt::from_u32(n as u32)
    }
}

impl From<u32> for UBigInt {
    fn from(n: u32) -> Self {
        UBigInt::from_u32(n)
    }
}

impl From<u64> for UBigInt {
    fn from(n: u64) -> Self {
        UBigInt::from_u64(n)
    }
}

impl From<u128> for UBigInt {
    fn from(n: u128) -> Self {
        UBigInt::from_u128(n)
    }
}

impl From<usize> for UBigInt {
    fn from(n: usize) -> Self {
        UBigInt::from_u64(n as u64)
    }
}

impl TryFrom<String> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: String) -> Result<Self, Self::Error> {
        UBigInt::from_string(&n)
    }
}

impl TryFrom<&str> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: &str) -> Result<Self, Self::Error> {
        UBigInt::from_string(n)
    }
}

impl TryFrom<Ratio> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: Ratio) -> Result<Self, Self::Error> {
        n.truncate_bi().try_into()
    }
}

impl TryFrom<BigInt> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: BigInt) -> Result<Self, Self::Error> {
        n.to_ubi()
    }
}