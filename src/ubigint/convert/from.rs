use crate::{Ratio, BigInt, UBigInt};

impl<T: Copy> From<&T> for UBigInt where UBigInt: From<T> {
    fn from(n: &T) -> Self {
        UBigInt::from(*n)
    }
}

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
impl From<f32> for UBigInt {
    fn from(n: f32) -> Self {
        Ratio::from(n).truncate_bi().into()
    }
}

/// It returns the truncated value of `Ratio::from(n)`.
impl From<f64> for UBigInt {
    fn from(n: f64) -> Self {
        Ratio::from(n).truncate_bi().into()
    }
}

impl From<i8> for UBigInt {
    fn from(n: i8) -> Self {
        UBigInt::from_u32(n as u32)
    }
}

impl From<i16> for UBigInt {
    fn from(n: i16) -> Self {
        UBigInt::from_u32(n as u32)
    }
}

impl From<i32> for UBigInt {
    fn from(n: i32) -> Self {
        UBigInt::from_u32(n as u32)
    }
}

impl From<i64> for UBigInt {
    fn from(n: i64) -> Self {
        UBigInt::from_u64(n as u64)
    }
}

impl From<i128> for UBigInt {
    fn from(n: i128) -> Self {
        UBigInt::from_u128(n as u128)
    }
}

impl From<isize> for UBigInt {
    fn from(n: isize) -> Self {
        UBigInt::from_u64(n as u64)
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

impl From<String> for UBigInt {
    fn from(n: String) -> Self {
        UBigInt::from_string(&n).unwrap_or(UBigInt::zero())
    }
}

impl From<&str> for UBigInt {
    fn from(n: &str) -> Self {
        UBigInt::from_string(n).unwrap_or(UBigInt::zero())
    }
}

impl From<Ratio> for UBigInt {
    fn from(n: Ratio) -> Self {
        n.truncate_bi().into()
    }
}

impl From<BigInt> for UBigInt {
    fn from(n: BigInt) -> Self {
        n.to_ubi().unwrap()
    }
}