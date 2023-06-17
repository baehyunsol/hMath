use crate::{Ratio, BigInt, UBigInt, ConversionError};
use crate::{impl_from_for_ref, impl_tryfrom_for_ref, impl_trait_for_general};

macro_rules! impl_from_ref_bigint {
    ($t: ty) => (
        impl_from_for_ref!(BigInt, $t);
    );
    ($t: ty, $($u: ty), +) => (
        impl_from_ref_bigint!($t);
        impl_from_ref_bigint!($($u),+);
    )
}

macro_rules! impl_tryfrom_ref_bigint {
    ($t: ty) => (
        impl_tryfrom_for_ref!(BigInt, $t);
    );
    ($t: ty, $($u: ty), +) => (
        impl_tryfrom_ref_bigint!($t);
        impl_tryfrom_ref_bigint!($($u),+);
    )
}

impl_from_ref_bigint!(bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_tryfrom_ref_bigint!(f32, f64);

impl From<bool> for BigInt {
    fn from(b: bool) -> Self {
        if b {
            BigInt::one()
        } else {
            BigInt::zero()
        }
    }
}

/// It returns the truncated value of `Ratio::from(n)`.
impl TryFrom<f32> for BigInt {
    type Error = ConversionError;

    fn try_from(n: f32) -> Result<Self, Self::Error> {
        Ok(Ratio::try_from(n)?.truncate_bi())
    }
}

/// It returns the truncated value of `Ratio::from(n)`.
impl TryFrom<f64> for BigInt {
    type Error = ConversionError;

    fn try_from(n: f64) -> Result<Self, Self::Error> {
        Ok(Ratio::try_from(n)?.truncate_bi())
    }
}

impl_trait_for_general!(From, i8, BigInt, from_i32);
impl_trait_for_general!(From, i16, BigInt, from_i32);
impl_trait_for_general!(From, i32, BigInt, from_i32);
impl_trait_for_general!(From, i64, BigInt, from_i64);
impl_trait_for_general!(From, i128, BigInt, from_i128);
impl_trait_for_general!(From, u8, BigInt, from_i32);
impl_trait_for_general!(From, u16, BigInt, from_i32);
impl_trait_for_general!(From, u32, BigInt, from_i64);

impl_trait_for_general!(TryFrom, &str, BigInt, from_string);

impl From<isize> for BigInt {
    fn from(n: isize) -> Self {
        BigInt::from_i64(n as i64)
    }
}

impl From<u64> for BigInt {
    fn from(n: u64) -> Self {
        BigInt::from_ubi(UBigInt::from_u64(n), false)
    }
}

impl From<u128> for BigInt {
    fn from(n: u128) -> Self {
        BigInt::from_ubi(UBigInt::from_u128(n), false)
    }
}

impl From<usize> for BigInt {
    fn from(n: usize) -> Self {
        BigInt::from_ubi(UBigInt::from_u64(n as u64), false)
    }
}

impl TryFrom<String> for BigInt {
    type Error = ConversionError;

    fn try_from(n: String) -> Result<Self, Self::Error> {
        BigInt::from_string(&n)
    }
}

impl From<Ratio> for BigInt {
    fn from(n: Ratio) -> Self {
        n.truncate_bi()
    }
}

impl From<UBigInt> for BigInt {
    fn from(n: UBigInt) -> Self {
        BigInt::from_ubi(n, false)
    }
}