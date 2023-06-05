use crate::{Ratio, BigInt, UBigInt};

impl<T: Copy> From<&T> for BigInt where BigInt: From<T> {
    fn from(n: &T) -> Self {
        BigInt::from(*n)
    }
}

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
impl From<f32> for BigInt {
    fn from(n: f32) -> Self {
        Ratio::from(n).truncate_bi()
    }
}

/// It returns the truncated value of `Ratio::from(n)`.
impl From<f64> for BigInt {
    fn from(n: f64) -> Self {
        Ratio::from(n).truncate_bi()
    }
}

impl From<i8> for BigInt {
    fn from(n: i8) -> Self {
        BigInt::from_i32(n as i32)
    }
}

impl From<i16> for BigInt {
    fn from(n: i16) -> Self {
        BigInt::from_i32(n as i32)
    }
}

impl From<i32> for BigInt {
    fn from(n: i32) -> Self {
        BigInt::from_i32(n)
    }
}

impl From<i64> for BigInt {
    fn from(n: i64) -> Self {
        BigInt::from_i64(n)
    }
}

impl From<i128> for BigInt {
    fn from(n: i128) -> Self {
        BigInt::from_i128(n)
    }
}

impl From<isize> for BigInt {
    fn from(n: isize) -> Self {
        BigInt::from_i64(n as i64)
    }
}

impl From<u8> for BigInt {
    fn from(n: u8) -> Self {
        BigInt::from_i32(n as i32)
    }
}

impl From<u16> for BigInt {
    fn from(n: u16) -> Self {
        BigInt::from_i32(n as i32)
    }
}

impl From<u32> for BigInt {
    fn from(n: u32) -> Self {
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

impl From<String> for BigInt {
    fn from(n: String) -> Self {
        BigInt::from_string(&n).unwrap_or(BigInt::zero())
    }
}

impl From<&str> for BigInt {
    fn from(n: &str) -> Self {
        BigInt::from_string(n).unwrap_or(BigInt::zero())
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