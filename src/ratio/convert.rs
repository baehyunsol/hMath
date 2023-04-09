use crate::{Ratio, BigInt};
use crate::err::ConversionError;

impl Ratio {

    pub fn from_denom_and_numer(denom: BigInt, numer: BigInt) -> Self {
        let mut result = Ratio::from_denom_and_numer_raw(denom, numer);
        result.fit();

        result
    }

    pub fn from_denom_and_numer_i32(denom: i32, numer: i32) -> Self {
        Ratio::from_denom_and_numer(BigInt::from_i32(denom), BigInt::from_i32(numer))
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

        match inspect_f32(n) {
            Ok((neg, exp, frac)) => if exp == i32::MIN {
                Ok(Ratio::zero())
            } else if exp >= 23 {
                // (2^23 + frac) * 2^(exp-23)
                let mut numer = BigInt::from_i32((frac + (1 << 23)) as i32);
                numer.mul_pow2_mut((exp - 23) as u32);

                if neg { numer.neg_mut(); }

                Ok(Ratio::from_bi(numer))
            } else {
                // (2^23 + frac) / 2^(23-exp)
                let mut numer = BigInt::from_i32((frac + (1 << 23)) as i32);
                let denom = BigInt::pow2((23 - exp) as u32);

                if neg { numer.neg_mut(); }

                Ok(Ratio::from_denom_and_numer(denom, numer))
            },
            Err(e) => Err(e)
        }

    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// This function does not return `f32::NAN` or `f32::INFINITY`, but it returns `ConversionError::NotInRange` instead.
    pub fn to_ieee754_f32(&self) -> Result<f32, ConversionError> {

        if self.is_zero() {
            return Ok(0.0)
        }

        let mut self_clone = self.clone();

        let neg = self_clone.is_neg();
        let mut exp = 0;

        if neg { self_clone.neg_mut(); }

        while self_clone.gt_one() {
            self_clone.div_i32_mut(2);
            exp += 1;
        }

        while self_clone.lt_one() {
            self_clone.mul_i32_mut(2);
            exp -= 1;
        }

        let mut frac = self_clone.sub_i32(1).mul_i32(1 << 23).truncate_bi().to_i32().unwrap() as u32;

        if exp < -126 {
            frac += 1 << 23;
            frac /= 2;
            exp += 1;

            while exp < -127 {
                exp += 1;
                frac /= 2;
            }

            if frac == 0 {
                return Err(ConversionError::NotInRange);
            }

        }

        exp += 127;

        if exp < 0 || exp > 255 {
            return Err(ConversionError::NotInRange);
        }

        let result = ((neg as u32) << 31) | ((exp as u32) << 23) | frac;
        let rp = &result as *const u32 as *const f32;

        unsafe { Ok(*rp) }
    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// Though the ieee 754 standard distinguishes negative 0 and positive 0, it doesn't distinguish between them.
    /// It returns an error if `n` is NaN or Inf.
    pub fn from_ieee754_f64(n: f64) -> Result<Self, ConversionError> {

        match inspect_f64(n) {
            Ok((neg, exp, frac)) => if exp == i32::MIN {
                Ok(Ratio::zero())
            } else if exp >= 52 {
                // (2^52 + frac) * 2^(exp-52)
                let mut numer = BigInt::from_i64((frac + (1 << 52)) as i64);
                numer.mul_pow2_mut((exp - 52) as u32);

                if neg { numer.neg_mut(); }

                Ok(Ratio::from_bi(numer))
            } else {
                // (2^52 + frac) / 2^(52-exp)
                let mut numer = BigInt::from_i64((frac + (1 << 52)) as i64);
                let denom = BigInt::pow2((52 - exp) as u32);

                if neg { numer.neg_mut(); }

                Ok(Ratio::from_denom_and_numer(denom, numer))
            },
            Err(e) => Err(e)
        }

    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// This function does not return `f64::NAN` or `f64::INFINITY`, but it returns `ConversionError::NotInRange` instead.
    pub fn to_ieee754_f64(&self) -> Result<f64, ConversionError> {

        if self.is_zero() {
            return Ok(0.0)
        }

        let mut self_clone = self.clone();

        let neg = self_clone.is_neg();
        let mut exp = 0;

        if neg { self_clone.neg_mut(); }

        while self_clone.gt_one() {
            self_clone.div_i32_mut(2);
            exp += 1;
        }

        while self_clone.lt_one() {
            self_clone.mul_i32_mut(2);
            exp -= 1;
        }

        let mut frac = self_clone.sub_i32(1).mul_bi(&BigInt::from_i64(1 << 52)).truncate_bi().to_i64().unwrap() as u64;

        if exp < -1022 {
            frac += 1 << 52;
            frac /= 2;
            exp += 1;

            while exp < -1023 {
                exp += 1;
                frac /= 2;
            }

            if frac == 0 {
                return Err(ConversionError::NotInRange);
            }

        }

        exp += 1023;

        if exp < 0 || exp > 2047 {
            return Err(ConversionError::NotInRange);
        }

        let result = ((neg as u64) << 63) | ((exp as u64) << 52) | frac;
        let rp = &result as *const u64 as *const f64;

        unsafe { Ok(*rp) }
    }

    pub fn from_string(s: &str) -> Result<Self, ConversionError> {

        if s.len() == 0 {
            return Err(ConversionError::NoData);
        }

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

        let is_neg = s.starts_with('-');

        // `111e22.33` is invalid
        let integer_end_index = if let Some(i) = dot_index {
            i
        } else {
            e_index.unwrap()
        };

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

                // if i + 1 == fractional_part_end_index, fractional_part is 0
                // eg: '3.'
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

        if is_neg {
            fractional_part.neg_mut();
        }

        let mut result = Ratio::from_bi(integer_part);
        result.add_rat_mut(&fractional_part);

        // exponential_part is a multiple of 2 and 5. So, it uses `rem_i32(2)` and `rem_i32(5)` instead of `rem_bi` for gcd
        // -> that's much faster
        if exponential_part != 0 {
            let mut exponential_part_bi = BigInt::from_i32(10).pow_u32(exponential_part.abs() as u32);

            if exponential_part > 0 {

                while result.denom.rem_pow2(2).is_zero() && exponential_part_bi.rem_pow2(2).is_zero() {
                    result.denom.div_i32_mut(2);
                    exponential_part_bi.div_i32_mut(2);
                }

                while result.denom.rem_i32(5).is_zero() && exponential_part_bi.rem_i32(5).is_zero() {
                    result.denom.div_i32_mut(5);
                    exponential_part_bi.div_i32_mut(5);
                }

                result.numer.mul_bi_mut(&exponential_part_bi);
            }

            else if exponential_part < 0 {

                while result.numer.rem_pow2(2).is_zero() && exponential_part_bi.rem_pow2(2).is_zero() {
                    result.numer.div_i32_mut(2);
                    exponential_part_bi.div_i32_mut(2);
                }

                while result.numer.rem_i32(5).is_zero() && exponential_part_bi.rem_i32(5).is_zero() {
                    result.numer.div_i32_mut(5);
                    exponential_part_bi.div_i32_mut(5);
                }

                result.denom.mul_bi_mut(&exponential_part_bi);
            }

            #[cfg(test)] assert!(result.is_valid());
        }

        Ok(result)
    }

    /// Ratio { 4, 7 } -> "4/7".
    pub fn to_ratio_string(&self) -> String {
        format!("{}/{}", self.numer.to_string_dec(), self.denom.to_string_dec())
    }

    /// Ratio { 4, 7 } -> "1.75".
    /// The length of the returned string is less or equal to `digits`.
    /// If `digits` is less than 6, it'll count that as 6.
    ///
    /// TODO: This function is very expensive.
    pub fn to_approx_string(&self, max_len: usize) -> String {
        let mut max_len = max_len.max(6);

        let log2 = self.numer.log2_accurate().sub_bi(&self.denom.log2_accurate());

        // 2^70777 = 10^21306 + small
        let log10i64 = log2.mul_i32(21306).div_i32(70777).div_i32(16777216).to_i64().unwrap();
        let is_neg = self.is_neg();

        if log10i64.abs() > 8600 {
            let log10 = Ratio::from_denom_and_numer(
                BigInt::from_i32(70777).mul_i32(16777216),
                log2.mul_i32(21306)
            );

            // TODO: use log function to calc the approx representation
            panic!("{}", log10.to_approx_string(24));
        }

        let result = if log10i64 > max_len as i64 - 3 {
            // we don't need the fractional part

            let mut bi = self.truncate_bi();
            bi.abs_mut();
            let big_decimal = BigInt::from_i32(10).pow_u32(16);
            let mut exp = 0;

            while bi.gt_bi(&big_decimal) {
                bi.div_i32_mut(1_000_000);
                exp += 6;
            }

            while bi.gt_i32(1_000_000_000) {
                bi.div_i32_mut(10);
                exp += 1;
            }

            let mut bi = bi.to_i32().unwrap();
            let mut digits = Vec::with_capacity(20);

            while bi > 0 {
                digits.push(bi % 10);
                bi /= 10;
                exp += 1;
            }

            digits.reverse();
            exp -= 1;

            while digits.len() > 1 && digits[digits.len() - 1] == 0 {
                digits.pop().unwrap();
            }

            let mut digits = if digits.len() == 1 {
                digits[0].to_string()
            } else {
                format!(
                    "{}.{}",
                    digits[0].to_string(),
                    digits[1..].iter().map(|n| n.to_string()).collect::<Vec<String>>().concat()
                )
            };

            let exp = format!("e{exp}");
            let neg = if is_neg { "-" } else { "" };

            // At least 2 digits
            max_len = max_len.max(exp.len() + 3);

            if digits.len() + exp.len() + neg.len() > max_len {
                digits = digits.get(0..(max_len - exp.len() - neg.len())).unwrap().to_string();

                // '3.e720' doesn't make sense; it should be `3e720`
                if digits.ends_with('.') {
                    digits = digits.get(0..(digits.len() - 1)).unwrap().to_string();
                }

            }

            format!("{neg}{digits}{exp}")
        }

        else if log10i64 < -(max_len as i64 - 3) {
            let mut self_clone = self.abs();
            let mut exp = -1;

            while self_clone.lt_one() {
                self_clone.mul_i32_mut(1_000_000_000);
                exp -= 9;
            }

            let mut self_int = self_clone.mul_i32(1_000_000_000).truncate_bi();
            exp -= 9;

            let sign_part = if is_neg { "-" } else { "" };
            let mut digits = Vec::with_capacity(20);

            while self_int.gt_i32(0) {
                digits.push(self_int.rem_i32(10).to_i32().unwrap());
                self_int.div_i32_mut(10);
                exp += 1;
            }

            digits.reverse();
            let exp = format!("e{exp}");

            // At least 2 digits
            max_len = max_len.max(exp.len() + 3);

            let mut curr_len = sign_part.len() + exp.len() + 1;

            while curr_len > max_len && digits.len() > 0 {
                digits.pop().unwrap();
                curr_len -= 1;
            }

            while digits.len() > 0 && digits[digits.len() - 1] == 0 {
                digits.pop().unwrap();
            }

            let digits = if digits.len() > 1 {
                format!("{}.{}", digits[0], digits[1..].iter().map(|c| c.to_string()).collect::<Vec<String>>().join(""))
            } else {
                format!("{}", digits[0])
            };

            format!("{sign_part}{digits}{exp}")
        }

        else {
            let int_part = self.truncate_bi().abs().to_string();
            let mut frac_part = self.abs().frac();
            let sign_part = if is_neg {"-"} else {""};

            let mut digits = Vec::with_capacity(20);
            let mut curr_len = int_part.len() + 1 + sign_part.len();

            while curr_len < max_len {
                frac_part.mul_i32_mut(10);
                digits.push(frac_part.truncate_bi().to_i32().unwrap());
                frac_part.frac_mut();
                curr_len += 1;
            }

            while digits.len() > 0 && digits[digits.len() - 1] == 0 {
                digits.pop().unwrap();
            }

            if digits.len() > 0 {
                format!("{sign_part}{int_part}.{}", digits.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(""))
            } else {
                format!("{sign_part}{int_part}")
            }

        };

        #[cfg(test)] assert!(result.len() <= max_len);

        result
    }

    /// `self.to_approx_string(12)`
    pub fn to_string(&self) -> String {
        self.to_approx_string(12)
    }

}

// it returns the first non-decimal character, if exists
// it assumes that `s` is a valid numeric literal
fn get_non_decimal_char(s: &str) -> Option<char> {

    for c in s.chars() {

        match c.to_ascii_lowercase() {
            '-' | '0' | '.' => { continue; }
            x if x.is_digit(10) => { return None; }
            _ => { return Some(c); }
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

impl std::fmt::Display for Ratio {

    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.to_string())
    }

}

#[cfg(test)]
mod tests {
    use super::{inspect_f32, inspect_f64};
    use crate::{Ratio, BigInt};
    use crate::consts::RUN_ALL_TESTS;
    use crate::err::ConversionError;

    #[test]
    fn string_test() {
        assert_eq!(
            Ratio::from_string("3.141592e720").unwrap().to_approx_string(12),
            "3.141592e720"
        );
        assert_eq!(
            Ratio::from_string("-3.141592e720").unwrap().to_approx_string(16),
            "-3.141592e720"
        );
        assert_eq!(
            Ratio::from_string("3.141592e720").unwrap().to_approx_string(9),
            "3.141e720"
        );
        assert_eq!(
            Ratio::from_string("-3.141592e720").unwrap().to_approx_string(10),
            "-3.141e720"
        );
        assert_eq!(
            Ratio::from_string("3.141592e720").unwrap().to_approx_string(6),
            "3.1e720"
        );
        assert_eq!(
            Ratio::from_string("-3.141592e720").unwrap().to_approx_string(6),
            "-3e720"
        );
        assert_eq!(
            Ratio::from_string("3e20").unwrap().to_approx_string(0),
            "3e20"
        );
        assert_eq!(
            Ratio::from_string("-3e20").unwrap().to_approx_string(0),
            "-3e20"
        );
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
        assert_eq!(
            Ratio::from_string("1600e-1").unwrap(),
            Ratio::from_bi(BigInt::from_i32(160))
        );
        assert_eq!(
            Ratio::from_string("3.012").unwrap(),
            Ratio::from_denom_and_numer_i32(1000, 3012)
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
    }

    #[test]
    fn exp_number_test() {
        let samples = vec![
            "1.234e-80",
            "1.004e-65",
            "1.23e-120",
            "1.2345e-9",
            "3e-1200",
            "3.141e7"
        ];

        for sample in samples.into_iter() {
            let n = Ratio::from_string(sample).unwrap();

            assert_eq!(sample, n.to_approx_string(sample.len()));
            assert_eq!(sample, n.to_approx_string(sample.len() + 1));
        }

        assert_eq!("3.1e120000", Ratio::from_string("3.14159e120000").unwrap().to_approx_string(8));

        // TODO: it panics (stack overflow)
        // possible workaround: use log functions for gigantic divisions
        assert_eq!("3.1e-120000", Ratio::from_string("3.14159e-120000").unwrap().to_approx_string(8));
    }

    #[test]
    fn ieee754_test() {

        if !RUN_ALL_TESTS { return; }

        let samples = vec![
            "0.0", "1.0", "2.0", "3.0",
            "3.", "1.125", "17.5",
            "17.0", "1048576.0", "0.0625",
            "0.03125", "0.015625", "0.0078125",
            "19.015625", "6256255.5",
            "0.01171875", "15.640625",
            "625e-3", "15.625e-2", "12e3",
            "16.3e2",
            // TODO: denormalized numbers (2^n, where n < -126)
        ];

        let samples: Vec<String> = vec![
            samples.iter().map(|s| format!("-{s}")).collect::<Vec<String>>(),
            samples.iter().map(|s| s.to_string()).collect()
        ].concat();

        for n in samples.into_iter() {
            let rat = Ratio::from_string(&n).unwrap();
            let nf32 = n.parse::<f32>().unwrap();
            let nf64 = n.parse::<f64>().unwrap();

            assert_eq!(Ratio::from_ieee754_f32(nf32).unwrap(), rat);
            assert_eq!(Ratio::from_ieee754_f64(nf64).unwrap(), rat);
            assert_eq!(rat.to_ieee754_f32().unwrap(), nf32);
            assert_eq!(rat.to_ieee754_f64().unwrap(), nf64);

            let n = rat.to_approx_string(n.len());
            let rat = Ratio::from_string(&n).unwrap();
            let nf32 = n.parse::<f32>().unwrap();
            let nf64 = n.parse::<f64>().unwrap();
            assert_eq!(Ratio::from_ieee754_f32(nf32).unwrap(), rat);
            assert_eq!(Ratio::from_ieee754_f64(nf64).unwrap(), rat);
        }

    }

    #[test]
    fn ieee754_fuzzing() {

        #[cfg(feature = "rand")] {

            let iter_count = if RUN_ALL_TESTS {
                240
            } else {
                12
            };

            for _ in 0..iter_count {
                let n32 = rand::random::<u32>();
                let nf32 = unsafe { *(&n32 as *const u32 as *const f32) };
                let res32 = Ratio::from_ieee754_f32(nf32);

                if nf32.is_nan() {
                    assert_eq!(res32, Err(ConversionError::NotANumber), "{n32}");
                }

                else if nf32.is_infinite() {
                    assert_eq!(res32, Err(ConversionError::NotANumber), "{n32}");
                }

                else {
                    let nf32_ = match res32 {
                        Ok(n_) => match n_.to_ieee754_f32() {
                            Ok(n__) => n__,
                            Err(e) => panic!("{n32}, {e:?}")
                        },
                        Err(e) => panic!("{n32}, {e:?}")
                    };

                    let n32_ = unsafe { *(&nf32_ as *const f32 as *const u32) };
                    assert_eq!(n32, n32_);
                }

                let n64 = rand::random::<u64>();
                let nf64 = unsafe { *(&n64 as *const u64 as *const f64) };
                let res64 = Ratio::from_ieee754_f64(nf64);

                if nf64.is_nan() {
                    assert_eq!(res64, Err(ConversionError::NotANumber), "{n64}");
                }

                else if nf64.is_infinite() {
                    assert_eq!(res64, Err(ConversionError::NotANumber), "{n64}");
                }

                else {
                    let nf64_ = match res64 {
                        Ok(n_) => match n_.to_ieee754_f64() {
                            Ok(n__) => n__,
                            Err(e) => panic!("{n64}, {e:?}")
                        },
                        Err(e) => panic!("{n64}, {e:?}")
                    };

                    let n64_ = unsafe { *(&nf64_ as *const f64 as *const u64) };
                    assert_eq!(n64, n64_);
                }

            }

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