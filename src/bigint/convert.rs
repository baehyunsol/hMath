use crate::{BigInt, UBigInt};
use crate::err::ConversionError;
use std::fmt;

impl BigInt {

    pub fn from_i32(n: i32) -> Self {
        let _is_neg = n < 0;

        BigInt {
            val: UBigInt::from_u32(n.abs() as u32),
            _is_neg
        }
    }

    pub fn to_i32(&self) -> Result<i32, ConversionError> {

        match self.val.to_u32() {
            Ok(n) => if !self.is_neg() && n <= i32::MAX as u32 {
                Ok(n as i32)
            } else if self.is_neg() && n <= (i32::MIN as i64).abs() as u32 {
                Ok((-(n as i64)) as i32)  // i32::MIN.abs() > i32::MAX.abs()
            } else {
                Err(ConversionError::NotInRange { permitted: "-2.14e-9~2.14e9".to_string(), error: self.to_scientific_notation(5) })
            },
            Err(e) => Err(e)
        }

    }

    pub fn from_i64(n: i64) -> Self {
        let _is_neg = n < 0;

        BigInt {
            val: UBigInt::from_u64(n.abs() as u64),
            _is_neg
        }
    }

    pub fn to_i64(&self) -> Result<i64, ConversionError> {

        match self.val.to_u64() {
            Ok(n) => if !self.is_neg() && n <= i64::MAX as u64 {
                Ok(n as i64)
            } else if self.is_neg() && n <= (i64::MIN as i128).abs() as u64 {
                Ok((-(n as i128)) as i64)  // i64::MIN.abs() > i64::MAX.abs()
            } else {
                Err(ConversionError::NotInRange { permitted: "9.22e-18~9.22e18".to_string(), error: self.to_scientific_notation(5) })
            },
            Err(e) => Err(e)
        }

    }

    pub fn from_i128(n: i128) -> Self {
        let _is_neg = n < 0;

        BigInt {
            val: UBigInt::from_u128(n.abs() as u128),
            _is_neg
        }
    }

    pub fn to_i128(&self) -> Result<i128, ConversionError> {

        match self.val.to_u128() {
            Ok(n) => if !self.is_neg() && n <= i128::MAX as u128 {
                Ok(n as i128)
            } else if self.is_neg() && n < (1 << 127) {
                Ok(-(n as i128))
            } else {
                Err(ConversionError::NotInRange { permitted: "-1.7e38~1.7e38".to_string(), error: self.to_scientific_notation(5) })
            },
            Err(e) => Err(e)
        }

    }

    pub fn from_ubi(n: UBigInt, is_neg: bool) -> Self {
        let is_neg = is_neg & !n.is_zero();
        BigInt::from_raw(n.0, is_neg)
    }

    pub fn to_ubi(&self) -> Result<UBigInt, ConversionError> {

        if self.is_neg() {
            Err(ConversionError::NotInRange { permitted: "0~inf".to_string(), error: self.to_scientific_notation(5) })
        }

        else {
            Ok(self.val.clone())
        }

    }

    /// ('-')? UBigInt
    /// see `UBigInt::from_string`
    pub fn from_string(s: &str) -> Result<Self, ConversionError> {

        if s.len() == 0 {
            Err(ConversionError::NoData)
        }

        else if s.starts_with('-') {

            if let Some(s) = s.get(1..) {

                match UBigInt::from_string(s) {
                    Ok(n) => {
                        // it has to be neg, except '-0'
                        let _is_neg = !n.is_zero();

                        Ok(BigInt {
                            val: n,
                            _is_neg
                        })
                    },
                    Err(e) => Err(e)
                }

            }

            else {
                Err(ConversionError::NoData)
            }

        }

        else {

            match UBigInt::from_string(s) {
                Ok(n) => Ok(BigInt {
                    val: n,
                    _is_neg: false
                }),
                Err(e) => Err(e)
            }

        }

    }

    /// '9.8e5'
    pub fn to_scientific_notation(&self, digits_max_len: usize) -> String {
        format!(
            "{}{}",
            if self.is_neg() { "-" } else { "" },
            self.val.to_scientific_notation(digits_max_len)
        )
    }

    /// see `UBigInt::to_string_dec`
    pub fn to_string_dec(&self) -> String {
        format!(
            "{}{}",
            if self.is_neg() { "-" } else { "" },
            self.val.to_string_dec()
        )
    }

    /// see `UBigInt::to_string_hex`
    pub fn to_string_hex(&self, prefix: bool) -> String {
        format!(
            "{}{}",
            if self.is_neg() { "-" } else { "" },
            self.val.to_string_hex(prefix)
        )
    }

    /// see `UBigInt::to_string_oct`
    pub fn to_string_oct(&self, prefix: bool) -> String {
        format!(
            "{}{}",
            if self.is_neg() { "-" } else { "" },
            self.val.to_string_oct(prefix)
        )
    }

    /// see `UBigInt::to_string_bin`
    pub fn to_string_bin(&self, prefix: bool) -> String {
        format!(
            "{}{}",
            if self.is_neg() { "-" } else { "" },
            self.val.to_string_bin(prefix)
        )
    }

}

impl fmt::Display for BigInt {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string_dec())
    }

}

impl fmt::LowerHex for BigInt {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string_hex(fmt.alternate()))
    }

}

impl fmt::Octal for BigInt {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string_oct(fmt.alternate()))
    }

}

impl fmt::Binary for BigInt {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string_bin(fmt.alternate()))
    }

}

impl fmt::LowerExp for BigInt {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_scientific_notation(5))
    }

}