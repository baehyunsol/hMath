use crate::{Ratio, BigInt, gcd_bi};
use crate::err::ConversionError;

impl Ratio {

    pub fn from_denom_and_numer(denom: BigInt, numer: BigInt) -> Self {
        let mut result = Ratio::from_denom_and_numer_raw(denom, numer);
        result.fit();

        result
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
        #[cfg(test)] assert!(n.is_valid());

        // Safety: 1 and another integer are always coprime. 1 is positive. denom is 1 when n is 0.
        Ratio::from_denom_and_numer_raw(BigInt::one(), n)
    }

    pub fn from_i32(n: i32) -> Self {
        // Safety: 1 and another integer are always coprime. 1 is positive. denom is 1 when n is 0.
        Ratio::from_denom_and_numer_raw(BigInt::one(), BigInt::from_i32(n))
    }

    pub fn from_i64(n: i64) -> Self {
        // Safety: 1 and another integer are always coprime. 1 is positive. denom is 1 when n is 0.
        Ratio::from_denom_and_numer_raw(BigInt::one(), BigInt::from_i64(n))
    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// Though the ieee 754 standard distinguishes negative 0 and positive 0, it doesn't distinguish between them.
    /// It returns an error if `n` is NaN or Inf.
    pub fn from_ieee754_f32(n: f32) -> Result<Self, ConversionError> {
        todo!()
    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// This function does not return `f32::NAN` or `f32::INFINITY`, but it returns `ConversionError::NotInRange` instead.
    pub fn to_ieee754_f32(&self) -> Result<f32, ConversionError> {
        todo!()
    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// Though the ieee 754 standard distinguishes negative 0 and positive 0, it doesn't distinguish between them.
    /// It returns an error if `n` is NaN or Inf.
    pub fn from_ieee754_f64(n: f64) -> Result<Self, ConversionError> {
        todo!()
    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// This function does not return `f64::NAN` or `f64::INFINITY`, but it returns `ConversionError::NotInRange` instead.
    pub fn to_ieee754_f64(&self) -> Result<f64, ConversionError> {
        todo!()
    }

    pub fn from_string(s: &str) -> Result<Self, ConversionError> {
        let dot_index = s.find('.');

        let e_index = if let Some(c) = get_non_decimal_char(s) {

            if c.to_ascii_lowercase() != 'x' {
                s.find(|c: char| c.to_ascii_lowercase() == 'e')
            }

            else {
                None
            }

        } else {
            s.find(|c: char| c.to_ascii_lowercase() == 'e')
        };

        if dot_index.is_none() && e_index.is_none() {
            return match BigInt::from_string(s) {
                Ok(n) => Ok(Ratio::from_bi(n)),
                Err(e) => Err(e)
            };
        }

        if let Some(c) = get_non_decimal_char(s) {

            if c.to_ascii_lowercase() != 'e' {
                return Err(ConversionError::InvalidChar(c));
            }

        }

        // `111e22.33` is invalid
        let integer_end_index = if let Some(i) = dot_index {
            i
        } else {
            e_index.unwrap()
        };

        // in case `-33.44e55`, `-33` is integer_part, `.44` is fractional_part and `e55` is exponential_part.
        // todo: find their index and extract each part
        let integer_part = {
            let integer_string = s.get(0..integer_end_index).unwrap();

            match BigInt::from_string(integer_string) {
                Ok(n) => n,
                Err(e) => { return Err(e); }
            }

        };
        let mut fractional_part = match dot_index {
            None => Ratio::zero(),
            Some(i) => {
                let fractional_part_end_index = if let Some(ii) = e_index {
                    ii
                } else {
                    s.len()
                };

                let fraction_string = match s.get((i + 1)..fractional_part_end_index) {
                    Some(i) => i,
                    _ => { return Err(ConversionError::UnexpectedEnd); }
                };
                let mut denom = BigInt::one();
                let mut numer = BigInt::zero();

                for c in fraction_string.chars() {

                    match c {
                        '_' => { continue; }
                        x if x.is_digit(10) => {
                            denom.mul_i32_mut(10);
                            numer.mul_i32_mut(10);
                            numer.add_i32_mut(x.to_digit(10).unwrap() as i32);
                        }
                        _ => {
                            return Err(ConversionError::InvalidChar(c));
                        }
                    }

                }

                Ratio::from_denom_and_numer(denom, numer)
            }
        };
        let exponential_part = match e_index {
            None => 0,
            Some(i) => {
                let exp_string = match s.get((i + 1)..) {
                    Some(i) => i,
                    _ => { return Err(ConversionError::UnexpectedEnd); }
                };

                match BigInt::from_string(&exp_string) {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(n) => match n.to_i32() {
                        Ok(n) => n,
                        Err(e) => { return Err(e); }
                    }
                }

            }
        };

        if integer_part.is_neg() {
            fractional_part.neg_mut();
        }

        let mut result = Ratio::from_bi(integer_part);
        result.add_rat(&fractional_part);

        if exponential_part == 0 {
            Ok(result)
        }

        else {
            // multiply 10^exp
            todo!()
        }

    }

    /// Ratio { 4, 7 } -> "4/7".
    pub fn to_ratio_string(&self) -> String {
        format!("{}/{}", self.numer.to_string_dec(), self.denom.to_string_dec())
    }

    /// Ratio { 4, 7 } -> "1.75"
    /// The length of the returned string is less or equal to `digits`.
    pub fn to_approx_string(&self, digits: usize) -> String {
        todo!()
    }

}

// it returns the first non-decimal character, if exists
// it assumes that `s` is a valid numeric literal
fn get_non_decimal_char(s: &str) -> Option<char> {

    for c in s.chars() {

        match c.to_ascii_lowercase() {
            '-' | '0' | '.' => { continue; }
            x if x.is_digit(10) => { return None; }
            x => { return Some(c); }
        }

    }

    None
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
    use crate::{Ratio, BigInt};
    use super::{inspect_f32, inspect_f64};

    #[test]
    fn string_test() {
        assert_eq!(
            // not 16 * 10^5
            Ratio::from_string("0x16e5").unwrap(),
            Ratio::from_bi(BigInt::from_i32(0x16e5))
        );

        assert_eq!(
            // not -16 * 10^5
            Ratio::from_string("-0x16e5").unwrap(),
            Ratio::from_bi(BigInt::from_i32(-0x16e5))
        );

        assert_eq!(
            Ratio::from_string("16e5").unwrap(),
            Ratio::from_bi(BigInt::from_i32(1600000))
        );

        assert!(Ratio::from_string("0x123.4").is_err());
        assert!(Ratio::from_string("0x123.4e4").is_err());
        assert!(Ratio::from_string("0x123.e44").is_err());
        assert!(Ratio::from_string("0x.3").is_err());
        assert!(Ratio::from_string("-0x123.4").is_err());
        assert!(Ratio::from_string("-0x123.4e4").is_err());
        assert!(Ratio::from_string("-0x123.e44").is_err());
        assert!(Ratio::from_string("-0x.3").is_err());

        // TODO: aren't the below valid in Rust?
        assert!(Ratio::from_string(".3").is_err());
        assert!(Ratio::from_string("-.3").is_err());
        assert!(Ratio::from_string("3.").is_err());
        assert!(Ratio::from_string("-3.").is_err());
    }

    #[test]
    fn ieee754_test() {
        let samples = vec![
            "0.0", "-0.0",
            "1.0", "-1.0",
            "2.0", "-2.0",
            "3.0", "-3.0",
            "1.125", "-1.125",
            // "1._125", "-1._125",  // is this valid in Rust?
            "1.1_25", "-1.1_25",
            "17.0", "-17.0",
            "1_7.0", "-1_7.0",
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
            let rat = Ratio::from_string(n).unwrap();
            assert_eq!(Ratio::from_ieee754_f32(n.parse::<f32>().unwrap()).unwrap(), rat);
            assert_eq!(Ratio::from_ieee754_f64(n.parse::<f64>().unwrap()).unwrap(), rat);

            let n = rat.to_approx_string(1000);
            let rat = Ratio::from_string(&n).unwrap();
            assert_eq!(Ratio::from_ieee754_f32(n.parse::<f32>().unwrap()).unwrap(), rat);
            assert_eq!(Ratio::from_ieee754_f64(n.parse::<f64>().unwrap()).unwrap(), rat);
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