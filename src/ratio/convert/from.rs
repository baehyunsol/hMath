use crate::{Ratio, BigInt, UBigInt, ConversionError};

impl<T: Copy> From<&T> for Ratio where Ratio: From<T> {
    fn from(n: &T) -> Self {
        Ratio::from(*n)
    }
}

impl From<bool> for Ratio {
    fn from(b: bool) -> Self {
        if b {
            Ratio::one()
        } else {
            Ratio::zero()
        }
    }
}

/// It returns 0 for NaN, Ratio(f32::MAX) for f32::Inf and Ratio(f32::MIN) for f32::NegInf.
impl From<f32> for Ratio {
    fn from(n: f32) -> Self {
        match Ratio::from_ieee754_f32(n) {
            Ok(n) => n,
            Err(ConversionError::NotANumber) => Ratio::zero(),
            Err(ConversionError::Infinity) => Ratio::from_ieee754_f32(f32::MAX).unwrap(),
            Err(ConversionError::NegInfinity) => Ratio::from_ieee754_f32(f32::MIN).unwrap(),
            _ => unreachable!()
        }
    }
}

/// It returns 0 for NaN, Ratio(f64::MAX) for f64::Inf and Ratio(f64::MIN) for f64::NegInf.
impl From<f64> for Ratio {
    fn from(n: f64) -> Self {
        match Ratio::from_ieee754_f64(n) {
            Ok(n) => n,
            Err(ConversionError::NotANumber) => Ratio::zero(),
            Err(ConversionError::Infinity) => Ratio::from_ieee754_f64(f64::MAX).unwrap(),
            Err(ConversionError::NegInfinity) => Ratio::from_ieee754_f64(f64::MIN).unwrap(),
            _ => unreachable!()
        }
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

/// If it fails to parse the string, it returns 0.
impl From<String> for Ratio {
    fn from(n: String) -> Self {
        Ratio::from_string(&n).unwrap_or(Ratio::zero())
    }
}

/// If it fails to parse the string, it returns 0.
impl From<&str> for Ratio {
    fn from(n: &str) -> Self {
        Ratio::from_string(n).unwrap_or(Ratio::zero())
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