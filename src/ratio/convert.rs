use crate::{Ratio, BigInt, UBigInt};
use crate::err::ConversionError;
use crate::utils::gcd_i32;
use crate::ubigint::convert::_to_scientific_notation;

mod from;
mod ieee754;

impl Ratio {

    pub fn from_denom_and_numer(denom: BigInt, numer: BigInt) -> Self {
        let mut result = Ratio::from_denom_and_numer_raw(denom, numer);
        result.fit();

        result
    }

    pub fn from_denom_and_numer_i32(mut denom: i32, mut numer: i32) -> Self {
        let r = gcd_i32(denom, numer);

        if r != 1 {
            denom /= r;
            numer /= r;
        }

        Ratio::from_denom_and_numer_raw(BigInt::from_i32(denom), BigInt::from_i32(numer))
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

    pub fn from_ubi(n: UBigInt) -> Self {
        #[cfg(test)] assert!(n.is_valid());

        // Safety: 1 and another integer are always coprime. 1 is positive. denom is 1 when n is 0.
        Ratio::from_denom_and_numer_raw(BigInt::one(), BigInt::from_ubi(n, false))
    }

    pub fn from_i32(n: i32) -> Self {
        // Safety: 1 and another integer are always coprime. 1 is positive. denom is 1 when n is 0.
        Ratio::from_denom_and_numer_raw(BigInt::one(), BigInt::from_i32(n))
    }

    pub fn from_i64(n: i64) -> Self {
        // Safety: 1 and another integer are always coprime. 1 is positive. denom is 1 when n is 0.
        Ratio::from_denom_and_numer_raw(BigInt::one(), BigInt::from_i64(n))
    }

    pub fn from_i128(n: i128) -> Self {
        // Safety: 1 and another integer are always coprime. 1 is positive. denom is 1 when n is 0.
        Ratio::from_denom_and_numer_raw(BigInt::one(), BigInt::from_i128(n))
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
    pub fn to_approx_string(&self, max_len: usize) -> String {
        let mut max_len = max_len.max(6);

        let log2 = self.numer.log2_accurate().sub_bi(&self.denom.log2_accurate());

        // 2^70777 = 10^21306 + small
        let log10i64 = log2.mul_i32(21306).div_i32(70777).shift_right(1).to_i64().unwrap();
        let is_neg = self.is_neg();

        let sign_part = if is_neg { "-" } else { "" };

        if log10i64.abs() > 9990 {
            let log10 = Ratio::from_denom_and_numer(
                BigInt::from_i64(70777 << 32),
                log2.mul_i32(21306)
            );

            // `truncate` and `frac` does the same operation twice
            let (mut exp, mut frac) = log10.truncate_and_frac();

            if exp.is_neg() && !frac.is_zero() {
                exp.sub_i32_mut(1);
                frac.abs_mut();
                frac = Ratio::one().sub_rat(&frac);
            }

            // it takes only 4 digits
            let mut digits = Ratio::from_denom_and_numer(
                BigInt::from_i64(1 << 32),
                BigInt::from_i64(exp10(&frac))
            ).mul_i32(1_000_000).truncate_bi().to_i32().unwrap();

            if digits % 1000 > 992 {
                digits += 1000 - digits % 1000;
            }

            digits /= 1000;

            let digits = if digits % 1000 == 0 {
                format!("{}", digits / 1000)
            } else if digits % 100 == 0 {
                format!("{}.{}", digits / 1000, digits / 100 % 10)
            } else if digits % 10 == 0 {
                format!("{}.{:02}", digits / 1000, digits % 1000 / 10)
            } else {
                format!("{}.{:03}", digits / 1000, digits % 1000)
            };

            return format!("{sign_part}{digits}e{exp}");
        }

        let result = if log10i64 > max_len as i64 - 3 {
            // we don't need the fractional part

            let mut bi = self.truncate_bi();
            bi.abs_mut();
            let mut exp = 0;

            if log10i64 > 15 {
                bi.div_bi_mut(&BigInt::from_i32(10).pow_u32(log10i64 as u32 - 10));
                exp += log10i64 - 10;
            }

            while bi.len() > 1 || bi.gt_i32(1_000_000_000) {
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

            // At least 2 digits
            max_len = max_len.max(exp.len() + 3);

            if digits.len() + exp.len() + sign_part.len() > max_len {
                digits = digits.get(0..(max_len - exp.len() - sign_part.len())).unwrap().to_string();

                // '3.e720' doesn't make sense; it should be `3e720`
                if digits.ends_with('.') {
                    digits = digits.get(0..(digits.len() - 1)).unwrap().to_string();
                }

            }

            format!("{sign_part}{digits}{exp}")
        }

        else if log10i64 < -(max_len as i64 - 3) {
            let mut self_clone = self.abs();
            let mut exp = -1;

            if log10i64 < -6 {
                let pow10 = -log10i64;
                let mut pow2 = pow10;
                let mut pow5 = pow10;

                while pow2 > 0 && self_clone.denom.rem_pow2(2).is_zero() {
                    self_clone.denom.div_i32_mut(2);
                    pow2 -= 1;
                }

                while pow5 > 0 && self_clone.denom.rem_i32(5).is_zero() {
                    self_clone.denom.div_i32_mut(5);
                    pow5 -= 1;
                }

                self_clone.numer.mul_bi_mut(&BigInt::exp2(pow2 as u64).mul_bi(&BigInt::from_i32(5).pow_u32(pow5 as u32)));
                exp -= pow10;
            }

            let mut self_int = self_clone.mul_i32(1_000_000_000).truncate_bi();
            exp -= 9;

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

            let mut curr_len = digits.len() + sign_part.len() + exp.len() + 1;

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

    /// '9.8e5'
    pub fn to_scientific_notation(&self, digits_max_len: usize) -> String {
        let len_min = self.numer.len().min(self.denom.len());
        let mut self_clone = if len_min > 4 {
            Ratio::from_denom_and_numer(
                self.denom.shift_right(len_min - 4),
                self.numer.shift_right(len_min - 4)
            )
        }

        else {
            self.clone()
        };

        self_clone.abs_mut();

        // truncate(log10(abs(self)))
        let approx_digits = self_clone.numer.log2_accurate().sub_bi(&self_clone.denom.log2_accurate()).mul_i32(21306).div_i32(70777).shift_right(1).to_i64().unwrap();

        let mut exp = 0;

        let self_bi = if approx_digits < 17 {
            exp -= 17 - approx_digits;
            self_clone.numer.mul_bi(&BigInt::from_i32(10).pow_u32((17 - approx_digits) as u32)).div_bi(&self_clone.denom).to_i64().unwrap()
        } else {
            exp += approx_digits - 17;
            self_clone.numer.div_bi(&self_clone.denom.mul_bi(&BigInt::from_i32(10).pow_u32(approx_digits as u32 - 17))).to_i64().unwrap()
        };

        let (digits, new_exp) = _to_scientific_notation(self_bi as u64, digits_max_len);

        format!(
            "{}{digits}e{}",
            if self.is_neg() { "-" } else { "" },
            exp + new_exp as i64
        )
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

// truncate(10^n * 4294967296), n is between 0 and 1
// internal function
// It's very inaccurate and very inefficient
fn exp10(n: &Ratio) -> i64 {
    // binary search
    // sqrt(10^a * 10^b) = 10^((a+b)/2)
    let mut small = BigInt::from_raw(vec![0, 1], false);  // 10^0 * 4294967296
    let mut small_exp = Ratio::zero();

    let mut big = BigInt::from_i32(10).shift_left(1);  // 10^1 * 4294967296
    let mut big_exp = Ratio::one();

    for _ in 0..32 {
        let mid = small.mul_bi(&big).sqrt();
        let mid_exp = small_exp.add_rat(&big_exp).div_i32(2);

        if mid_exp.lt_rat(n) {
            small = mid;
            small_exp = mid_exp;
        }

        else {
            big = mid;
            big_exp = mid_exp;
        }

    }

    small.to_i64().unwrap()
}

impl std::fmt::Display for Ratio {

    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.to_string())
    }

}

#[cfg(test)]
mod tests {
    use crate::{Ratio, BigInt};

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

        assert_eq!("1.099e20000", Ratio::from_string("1.09999999999e20000").unwrap().to_approx_string(8));
        assert_eq!("9.099e20000", Ratio::from_string("9.09999999999e20000").unwrap().to_approx_string(8));
        assert_eq!("3.141e120000", Ratio::from_string("3.14159e120000").unwrap().to_approx_string(8));
        assert_eq!("3.141e-120000", Ratio::from_string("3.14159e-120000").unwrap().to_approx_string(8));
        assert_eq!("1e10000", Ratio::from_string("1.0001e10000").unwrap().to_approx_string(8));
        assert_eq!("1e-10000", Ratio::from_string("1.0001e-10000").unwrap().to_approx_string(8));
        assert_eq!("1.1e10000", Ratio::from_string("1.1001e10000").unwrap().to_approx_string(8));
        assert_eq!("1.1e-10000", Ratio::from_string("1.1001e-10000").unwrap().to_approx_string(8));
        assert_eq!("1.01e10000", Ratio::from_string("1.01e10000").unwrap().to_approx_string(8));
        assert_eq!("1.01e-10000", Ratio::from_string("1.01e-10000").unwrap().to_approx_string(8));
    }

    #[test]
    fn ratio_scientific_notation_test() {
        let exp = -1024;

        for i in 0..64 {
            let s = Ratio::from_string(&format!("3.1415926535e{}", exp + i * 32)).unwrap();
            let s2 = s.neg();

            let ans = format!("3.14159e{}", exp + i * 32);
            let ans2 = format!("-{ans}");

            assert_eq!(ans, s.to_scientific_notation(6));
            assert_eq!(ans2, s2.to_scientific_notation(6));
        }

    }

}