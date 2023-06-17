use crate::{BigInt, UBigInt, Ratio, ConversionError};
use crate::impl_trivial_try_from;

macro_rules! impl_ref_for_ubigints {
    (From, $t: ty) => (
        impl From<UBigInt> for $t {
            fn from(n: UBigInt) -> Self {
                (&n).into()
            }
        }
    );
    (TryFrom, $t: ty) => (
        impl TryFrom<UBigInt> for $t {
            type Error = ConversionError;

            fn try_from(n: UBigInt) -> Result<Self, Self::Error> {
                (&n).try_into()
            }
        }
    );
    (From, $t: ty, $($u: ty), +) => (
        impl_ref_for_ubigints!(From, $t);
        impl_ref_for_ubigints!(From, $($u),+);
    );
    (TryFrom, $t: ty, $($u: ty), +) => (
        impl_ref_for_ubigints!(TryFrom, $t);
        impl_ref_for_ubigints!(TryFrom, $($u),+);
    )
}

impl From<&UBigInt> for bool {
    fn from(n: &UBigInt) -> Self {
        !n.is_zero()
    }
}

impl TryFrom<&UBigInt> for u8 {
    type Error = ConversionError;

    fn try_from(n: &UBigInt) -> Result<Self, Self::Error> {
        Ok(n.to_u32()?.try_into()?)
    }
}

impl TryFrom<&UBigInt> for u16 {
    type Error = ConversionError;

    fn try_from(n: &UBigInt) -> Result<Self, Self::Error> {
        Ok(n.to_u32()?.try_into()?)
    }
}

impl_trivial_try_from!(&UBigInt, u32, to_u32);
impl_trivial_try_from!(&UBigInt, u64, to_u64);
impl_trivial_try_from!(&UBigInt, u128, to_u128);

impl_trivial_try_from!(Fallible, &UBigInt, usize, to_u32);
impl_trivial_try_from!(Fallible, &UBigInt, i8, to_u32);
impl_trivial_try_from!(Fallible, &UBigInt, i16, to_u32);
impl_trivial_try_from!(Fallible, &UBigInt, i32, to_u32);
impl_trivial_try_from!(Fallible, &UBigInt, i64, to_u64);
impl_trivial_try_from!(Fallible, &UBigInt, isize, to_u64);
impl_trivial_try_from!(Fallible, &UBigInt, i128, to_u128);

impl_ref_for_ubigints!(From, bool, BigInt, Ratio);
impl_ref_for_ubigints!(
    TryFrom,
    i8, i16, i32, i64, isize, i128,
    u8, u16, u32, u64, usize, u128
);

impl TryFrom<&UBigInt> for f32 {
    type Error = ConversionError;

    fn try_from(n: &UBigInt) -> Result<Self, Self::Error> {
        Ok(f32::try_from(&Ratio::from(n))?)
    }
}

impl TryFrom<&UBigInt> for f64 {
    type Error = ConversionError;

    fn try_from(n: &UBigInt) -> Result<Self, Self::Error> {
        Ok(f64::try_from(&Ratio::from(n))?)
    }
}