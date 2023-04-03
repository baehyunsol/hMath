use crate::{Ratio, BigInt, gcd_bi};
use crate::err::ConversionError;

impl Ratio {

    pub fn from_denom_and_numer(mut denom: BigInt, mut numer: BigInt) -> Self {

        if denom.is_neg() {
            denom.neg_mut();
            numer.neg_mut();
        }

        let r = gcd_bi(&denom, &numer);

        if !r.is_one() {
            denom.div_bi_mut(&r);
            numer.div_bi_mut(&r);
        }

        Ratio { denom, numer }
    }

    /// This function does not do any safety checks. Use this function only when you're sure that the properties below are satisfied.
    ///
    /// - `denom` and `numer` are coprime
    /// - `denom` is positive
    /// - `denom` is 1 when `numer` is 0
    pub fn from_denom_and_numer_raw(denom: BigInt, numer: BigInt) -> Self {
        Ratio { denom, numer }
    }

    pub fn from_bi(n: BigInt) -> Self {
        Ratio { denom: BigInt::one(), numer: n }
    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// Though the ieee 754 standard distinguishes negative 0 and positive 0, it doesn't distinguish between them.
    pub fn from_ieee754_f32(n: f32) -> Self {
        todo!()
    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// This function does not return `f32::NAN` or `f32::INFINITY`, but it returns `ConversionError::NotInRange` instead.
    pub fn to_ieee754_f32(&self) -> Result<f32, ConversionError> {
        todo!()
    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// Though the ieee 754 standard distinguishes negative 0 and positive 0, it doesn't distinguish between them.
    pub fn from_ieee754_f64(n: f64) -> Self {
        todo!()
    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// This function does not return `f64::NAN` or `f64::INFINITY`, but it returns `ConversionError::NotInRange` instead.
    pub fn to_ieee754_f64(&self) -> Result<f64, ConversionError> {
        todo!()
    }

    // TODO: support hex/bin/oct representations
    //   - `0x1.23` -> 1 + 2/16 + 3/256
    //   - `0x1.23e12` -> (1 + 2/16 + 3/256) * (16 ^ 0x12)
    //   - in hex case, `e` represents both number and exp -> what should I do?
    // TODO: base 64 number system: 0~9, a~z, A~Z, #, $
    //   - sounds very space-efficient, doesn't it?
    pub fn from_string(s: &str) -> Result<Self, ConversionError> {
        let dot_index = s.find('.');
        let e_index = s.find(|c: char| c.to_ascii_lowercase() == 'e');

        // in case `-33.44e55`, `-33` is integer_part, `.44` is fractional_part and `e55` is exponential_part.
        // todo: find their index and extract each part
        let integer_part = todo!();
        let fractional_part = todo!();
        let exponential_part = todo!();
    }

}

// https://en.wikipedia.org/wiki/IEEE_754
// it returns (false, i32::MIN, 0) when n is 0
fn inspect_f32(n: f32) -> Result<(bool, i32, u32), ConversionError> {  // (neg, exp, frac)
    let n_u32 = {
        let np = &n as *const f32 as *const u32;

        unsafe { *np }
    };

    let neg = n_u32 > (1 << 31);
    let mut exp = ((n_u32 >> 23) % 256) as i32 - 127;
    let mut frac = n_u32 % (1 << 23);

    // denormalized
    // if neg { -1 } else { 1 } * 2^(-126) * (frac / 2^23)
    if exp == -127 {

        if frac == 0 {
            return Ok((false, i32::MIN, 0));
        }

        while frac <= (1 << 23) {
            frac *= 2;
            exp -= 1;
        }

        frac -= 1 << 23;
    }

    else if exp == 128 {

        if frac == 0 {
            return Err(ConversionError::Infinity);
        }

        else {
            return Err(ConversionError::NotANumber);
        }

    }

    // if neg { -1 } else { 1 } * 2^exp * (1 + frac / 2^23)
    Ok((neg, exp, frac))
}

// https://en.wikipedia.org/wiki/IEEE_754
// it returns (false, i32::MIN, 0) when n is 0
fn inspect_f64(n: f64) -> Result<(bool, i32, u64), ConversionError> {  // (neg, exp, frac)
    let n_u64 = {
        let np = &n as *const f64 as *const u64;

        unsafe { *np }
    };

    let neg = n_u64 > (1 << 63);
    let mut exp = ((n_u64 >> 52) % 2048) as i32 - 1023;
    let mut frac = n_u64 % (1 << 52);

    // denormalized
    // if neg { -1 } else { 1 } * 2^(-1022) * (frac / 2^52)
    if exp == -1023 {

        if frac == 0 {
            return Ok((false, i32::MIN, 0));
        }

        while frac <= (1 << 52) {
            frac *= 2;
            exp -= 1;
        }

        frac -= 1 << 52;
    }

    else if exp == 1024 {

        if frac == 0 {
            return Err(ConversionError::Infinity);
        }

        else {
            return Err(ConversionError::NotANumber);
        }

    }

    // if neg { -1 } else { 1 } * 2^exp * (1 + frac / 2^52)
    Ok((neg, exp, frac))
}

#[cfg(test)]
mod tests {
    use crate::Ratio;
    use super::{inspect_f32, inspect_f64};

    #[test]
    fn ieee754_test() {
        let samples = vec![
            "0.0", "-0.0",
            "1.0", "-1.0",
            "2.0", "-2.0",
            "3.0", "-3.0",
            "1.125", "-1.125",
            "17.0", "-17.0",
            "17.5", "-17.5",
            "1048576.0", "-1048576.0",
            "0.0625", "-0.0625",
            "0.01171875", "-0.01171875",
            "15.640625", "-15.640625",
            "625e-3", "-625e-3",
            "15.625e-2", "-15.625e-2",
            "12e3", "-12e3",
            "16.3e2", "-16.3e2",
            // TODO: denormalized numbers (2^n, where n < -126)
        ];

        for n in samples.into_iter() {
            assert_eq!(Ratio::from_ieee754_f32(n.parse::<f32>().unwrap()), Ratio::from_string(n).unwrap());
            assert_eq!(Ratio::from_ieee754_f64(n.parse::<f64>().unwrap()), Ratio::from_string(n).unwrap());
        }

    }

    #[test]
    fn ieee754_inspect_test() {
        assert_eq!((false, 1, 0), inspect_f32(2.0).unwrap());
        assert_eq!((true, 1, 0), inspect_f32(-2.0).unwrap());
        assert_eq!((false, 0, (1 << 22)), inspect_f32(1.5).unwrap());

        // (1 + 1 / 16 + 1 / 32) * 16
        assert_eq!((false, 4, (1 << 19) + (1 << 18)), inspect_f32(17.5).unwrap());

        // (1 + 1 / 2 + 1 / 4 + 1 / 8 + 1 / 16 + 1 / 32 + 1 / 128) * 32
        assert_eq!((false, 5, (1 << 51) + (1 << 50) + (1 << 49) + (1 << 48) + (1 << 47) + (1 << 45)), inspect_f64(63.25).unwrap());

        // (1 + 1 / 512) * 1
        assert_eq!((false, 0, 1 << 43), inspect_f64(1.001953125).unwrap());

        assert_eq!((false, i32::MIN, 0), inspect_f32(0.0).unwrap());
        assert_eq!((false, i32::MIN, 0), inspect_f32(-0.0).unwrap());
        assert_eq!((false, i32::MIN, 0), inspect_f64(0.0).unwrap());
        assert_eq!((false, i32::MIN, 0), inspect_f64(-0.0).unwrap());
    }

}