use crate::{Ratio, BigInt, UBigInt, ConversionError};
use crate::{impl_from_for_ref, impl_tryfrom_for_ref, impl_trait_for_general};

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
        Ratio::try_from(n)?.truncate_bi().try_into()
    }
}

/// It returns the truncated value of `Ratio::from(n)`.
impl TryFrom<f64> for UBigInt {
    type Error = ConversionError;

    fn try_from(n: f64) -> Result<Self, Self::Error> {
        Ratio::try_from(n)?.truncate_bi().try_into()
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

impl_trait_for_general!(From, u8, UBigInt, from_u32);
impl_trait_for_general!(From, u16, UBigInt, from_u32);
impl_trait_for_general!(From, u32, UBigInt, from_u32);
impl_trait_for_general!(From, u64, UBigInt, from_u64);
impl_trait_for_general!(From, u128, UBigInt, from_u128);

impl_trait_for_general!(TryFrom, &str, UBigInt, from_string);

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