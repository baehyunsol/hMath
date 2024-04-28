use crate::{F192, BigInt, Ratio};
use crate::fp192::{EXP_COEFF, SIGN_MASK};

mod string;

impl F192 {
    const fn from_u128(n: u128) -> Self {
        if n == 0 {
            return F192::ZERO;
        }

        let mut digits = n;
        let mut exp = EXP_COEFF;

        let leading_zeros = digits.leading_zeros();
        digits <<= leading_zeros;
        exp -= leading_zeros as u64;

        F192 {
            digits: digits & !SIGN_MASK,
            exp,
        }
    }

    const fn from_i128(n: i128) -> Self {
        if n < 0 {
            let mut result = F192::from_u128(n.abs() as u128);
            result.digits |= SIGN_MASK;

            result
        }

        else {
            F192::from_u128(n as u128)
        }
    }

    const fn to_u128(&self) -> Result<u128, ()> {
        if self.is_zero() {
            return Ok(0);
        }

        if self.is_neg() || self.exp > EXP_COEFF {
            return Err(());
        }

        let digits = self.digits | SIGN_MASK;

        Ok(digits >> (EXP_COEFF - self.exp))
    }

    fn to_i128(&self) -> Result<i128, ()> {
        if self.is_neg() {
            let n = self.neg().to_u128()?;

            if n < (1 << 127) {
                Ok(-(n as i128))
            }

            else {
                Err(())
            }
        }

        else {
            let n = self.to_u128()?;

            if let Ok(n) = TryInto::<i128>::try_into(n) {
                Ok(n)
            }

            else {
                Err(())
            }
        }
    }
}

macro_rules! impl_from_uint {
    ($ty: ty) => {
        impl From<$ty> for F192 {
            fn from(n: $ty) -> Self {
                F192::from_u128(n as u128)
            }
        }
    }
}

impl_from_uint!(u8);
impl_from_uint!(u16);
impl_from_uint!(u32);
impl_from_uint!(u64);
impl_from_uint!(usize);
impl_from_uint!(u128);

macro_rules! impl_from_iint {
    ($ty: ty) => {
        impl From<$ty> for F192 {
            fn from(n: $ty) -> Self {
                F192::from_i128(n as i128)
            }
        }
    }
}

impl_from_iint!(i8);
impl_from_iint!(i16);
impl_from_iint!(i32);
impl_from_iint!(i64);
impl_from_iint!(isize);
impl_from_iint!(i128);

macro_rules! impl_try_from_int {
    ($ty: ty, $meth: ident) => {
        impl TryFrom<F192> for $ty {
            type Error = ();

            fn try_from(n: F192) -> Result<$ty, ()> {
                TryInto::<$ty>::try_into(n.$meth()?).map_err(|_| ())
            }
        }
    }
}

impl_try_from_int!(u8, to_u128);
impl_try_from_int!(u16, to_u128);
impl_try_from_int!(u32, to_u128);
impl_try_from_int!(u64, to_u128);
impl_try_from_int!(usize, to_u128);
impl_try_from_int!(u128, to_u128);

impl_try_from_int!(i8, to_i128);
impl_try_from_int!(i16, to_i128);
impl_try_from_int!(i32, to_i128);
impl_try_from_int!(i64, to_i128);
impl_try_from_int!(isize, to_i128);
impl_try_from_int!(i128, to_i128);

impl TryFrom<f32> for F192 {
    type Error = crate::ConversionError;

    fn try_from(n: f32) -> Result<F192, Self::Error> {
        // n = sign * (1 + frac / 2^23) * 2^exp
        let (sign, exp, frac) = crate::inspect_ieee754_f32(n)?;

        if exp == i32::MIN && frac == 0 && !sign {
            return Ok(F192::ZERO);
        }

        Ok(F192 {
            digits: ((frac as u128) << 104) | ((sign as u128) << 127),
            exp: (exp as i64 + EXP_COEFF as i64) as u64 - 127,
        })
    }
}

impl TryFrom<f64> for F192 {
    type Error = crate::ConversionError;

    fn try_from(n: f64) -> Result<F192, Self::Error> {
        // n = sign * (1 + frac / 2^52) * 2^exp
        let (sign, exp, frac) = crate::inspect_ieee754_f64(n)?;

        if exp == i32::MIN && frac == 0 && !sign {
            return Ok(F192::ZERO);
        }

        Ok(F192 {
            digits: ((frac as u128) << 75) | ((sign as u128) << 127),
            exp: (exp as i64 + EXP_COEFF as i64) as u64 - 127,
        })
    }
}

impl From<F192> for f32 {
    fn from(n: F192) -> f32 {
        let mut mant = (n.digits >> 99) & 268435455;
        let rem = mant & 31;
        mant >>= 5;

        // 254 = 127 + 127 = f32 exp bias + f192 digit bias
        let mut exp = n.exp as i64 - EXP_COEFF as i64 + 254;

        if rem > 15 {
            mant += 1;

            if mant == 8388608 {
                mant = 0;
                exp += 1;
            }
        }

        if exp < 0 {
            0.0
        } else if exp > 255 {
            if n.is_neg() {
                f32::NEG_INFINITY
            } else {
                f32::INFINITY
            }
        } else {
            unsafe {
                std::mem::transmute::<u32, f32>(((n.is_neg() as u32) << 31) | (((exp as u32) & 255) << 23) | mant as u32)
            }
        }
    }
}

impl From<F192> for f64 {
    fn from(n: F192) -> f64 {
        let mut mant = (n.digits >> 70) & 144115188075855871;
        let rem = mant & 31;
        mant >>= 5;

        // 1150 = 1023 + 127 = f64 exp bias + f192 digit bias
        let mut exp = n.exp as i64 - EXP_COEFF as i64 + 1150;

        if rem > 15 {
            mant += 1;

            if mant == 4503599627370496 {
                mant = 0;
                exp += 1;
            }
        }

        if exp < 0 {
            0.0
        } else if exp > 2047 {
            if n.is_neg() {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            }
        } else {
            unsafe {
                std::mem::transmute::<u64, f64>(((n.is_neg() as u64) << 63) | (((exp as u64) & 2047) << 52) | mant as u64)
            }
        }
    }
}

impl From<&F192> for Ratio {
    fn from(n: &F192) -> Ratio {
        if n.is_zero() {
            return Ratio::zero();
        }

        let d = n.digits | SIGN_MASK;
        let e = n.exp as i64 - EXP_COEFF as i64;

        let mut result: Ratio = d.into();
        let e_pow = BigInt::one().mul_pow2(e.abs() as u32);

        if e < 0 {
            result.div_bi_mut(&e_pow);
        }

        else {
            result.mul_bi_mut(&e_pow);
        }

        if n.is_neg() {
            result.neg_mut();
        }

        result
    }
}

impl From<F192> for Ratio {
    fn from(n: F192) -> Ratio {
        (&n).into()
    }
}

impl From<&Ratio> for F192 {
    fn from(n: &Ratio) -> F192 {
        if n.is_zero() {
            return F192::ZERO;
        }

        let lower_bound: Ratio = SIGN_MASK.into();
        let upper_bound = lower_bound.mul_i32(2);
        let mut n_abs = n.abs();
        let mut exp = EXP_COEFF;

        // TODO: optimize
        while n_abs.lt(&lower_bound) {
            n_abs.mul_i32_mut(2);
            exp -= 1;
        }

        while n_abs.geq(&upper_bound) {
            n_abs.div_i32_mut(2);
            exp += 1;
        }

        let n_abs_rem: u32 = n_abs.mul_i32(32).truncate_bi().rem_i32(32).try_into().unwrap();

        n_abs.sub_mut(&lower_bound);
        let mut digits: u128 = n_abs.truncate_bi().try_into().unwrap();

        if n_abs_rem > 15 {
            if digits != u128::MAX {
                digits += 1;
            }

            else {
                digits = 0;
                exp += 1;
            }
        }

        F192 {
            digits: digits | ((n.is_neg() as u128) << 127),
            exp,
        }
    }
}

impl From<Ratio> for F192 {
    fn from(n: Ratio) -> F192 {
        (&n).into()
    }
}
