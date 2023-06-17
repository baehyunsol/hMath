use crate::{BigInt, Ratio, ConversionError};
use crate::impl_trivial_try_from;

impl From<BigInt> for bool {
    fn from(n: BigInt) -> Self {
        !n.is_zero()
    }
}

impl_trivial_try_from!(Fallible, BigInt, i8, to_i32);
impl_trivial_try_from!(Fallible, BigInt, i16, to_i32);

impl_trivial_try_from!(BigInt, i32, to_i32);
impl_trivial_try_from!(BigInt, i64, to_i64);
impl_trivial_try_from!(Fallible, BigInt, isize, to_i64);
impl_trivial_try_from!(BigInt, i128, to_i128);

impl_trivial_try_from!(Fallible, BigInt, u8, to_i32);
impl_trivial_try_from!(Fallible, BigInt, u16, to_i32);
impl_trivial_try_from!(Fallible, BigInt, u32, to_i32);
impl_trivial_try_from!(Fallible, BigInt, u64, to_i128);
impl_trivial_try_from!(Fallible, BigInt, usize, to_i128);

impl TryFrom<BigInt> for u128 {
    type Error = ConversionError;

    fn try_from(n: BigInt) -> Result<Self, Self::Error> {

        if n.is_neg() {
            Err(ConversionError::NotInRange { permitted: "0~3.4e38".to_string(), error: format!("{n}") })
        }

        else {
            n.val.try_into()
        }

    }
}

impl TryFrom<BigInt> for f32 {
    type Error = ConversionError;

    fn try_from(n: BigInt) -> Result<Self, Self::Error> {
        Ok(f32::try_from(Ratio::from(n))?)
    }
}

impl TryFrom<BigInt> for f64 {
    type Error = ConversionError;

    fn try_from(n: BigInt) -> Result<Self, Self::Error> {
        Ok(f64::try_from(Ratio::from(n))?)
    }
}