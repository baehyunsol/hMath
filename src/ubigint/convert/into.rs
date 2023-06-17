use crate::{UBigInt, Ratio, ConversionError};
use crate::impl_trivial_try_from;

impl From<UBigInt> for bool {
    fn from(n: UBigInt) -> Self {
        !n.is_zero()
    }
}

impl TryFrom<UBigInt> for u8 {
    type Error = ConversionError;

    fn try_from(n: UBigInt) -> Result<Self, Self::Error> {
        Ok(n.to_u32()?.try_into()?)
    }
}

impl TryFrom<UBigInt> for u16 {
    type Error = ConversionError;

    fn try_from(n: UBigInt) -> Result<Self, Self::Error> {
        Ok(n.to_u32()?.try_into()?)
    }
}

impl_trivial_try_from!(UBigInt, u32, to_u32);
impl_trivial_try_from!(UBigInt, u64, to_u64);
impl_trivial_try_from!(UBigInt, u128, to_u128);

impl_trivial_try_from!(Fallible, UBigInt, usize, to_u32);
impl_trivial_try_from!(Fallible, UBigInt, i8, to_u32);
impl_trivial_try_from!(Fallible, UBigInt, i16, to_u32);
impl_trivial_try_from!(Fallible, UBigInt, i32, to_u32);
impl_trivial_try_from!(Fallible, UBigInt, i64, to_u64);
impl_trivial_try_from!(Fallible, UBigInt, isize, to_u64);
impl_trivial_try_from!(Fallible, UBigInt, i128, to_u128);

impl TryFrom<UBigInt> for f32 {
    type Error = ConversionError;

    fn try_from(n: UBigInt) -> Result<Self, Self::Error> {
        Ok(f32::try_from(Ratio::from(n))?)
    }
}

impl TryFrom<UBigInt> for f64 {
    type Error = ConversionError;

    fn try_from(n: UBigInt) -> Result<Self, Self::Error> {
        Ok(f64::try_from(Ratio::from(n))?)
    }
}