use crate::{ConversionError, F192};
use crate::fp192::{EXP_COEFF, SIGN_MASK};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

enum ParseState {
    Init,
    InitZero,
    ReadDigit,
    ReadBase(u32),
    ReadSubZero,
    ReadExp,
}

impl FromStr for F192 {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, ConversionError> {
        let mut curr_state = ParseState::Init;
        let mut is_neg = false;
        let mut digit_buffer = vec![];
        let mut exp = 0i64;
        let mut exp_buffer = vec![];

        for c in s.chars() {
            match curr_state {
                ParseState::Init => {
                    if c == '-' {
                        if is_neg {
                            return Err(ConversionError::InvalidChar(c));
                        }

                        else {
                            is_neg = true;
                        }
                    }

                    else if c == '0' {
                        curr_state = ParseState::InitZero;
                    }

                    else if c.is_ascii_digit() {
                        digit_buffer.push(c);
                        curr_state = ParseState::ReadDigit;
                    }

                    else {
                        return Err(ConversionError::InvalidChar(c));
                    }
                },
                ParseState::InitZero => {
                    if c == 'x' || c == 'X' {
                        curr_state = ParseState::ReadBase(16);
                    }

                    else if c == 'o' || c == 'O' {
                        curr_state = ParseState::ReadBase(8);
                    }

                    else if c == 'b' || c == 'B' {
                        curr_state = ParseState::ReadBase(2);
                    }

                    else if c == 'e' || c == 'E' {
                        curr_state = ParseState::ReadExp;
                    }

                    else if c == '.' {
                        curr_state = ParseState::ReadSubZero;
                    }

                    else {
                        return Err(ConversionError::InvalidChar(c));
                    }
                },
                ParseState::ReadBase(base) => {
                    if let Some(n) = c.to_digit(base) {
                        // I don't want to call `to_digit` again :)
                        digit_buffer.push((n as u8 + 32) as char);
                    }

                    else if c == '_' {}

                    else {
                        return Err(ConversionError::InvalidChar(c));
                    }
                },
                ParseState::ReadDigit => {
                    if c.is_ascii_digit() {
                        digit_buffer.push(c);
                    }

                    else if c == '_' {}

                    else if c == '.' {
                        curr_state = ParseState::ReadSubZero;
                    }

                    else if c == 'e' || c == 'E' {
                        curr_state = ParseState::ReadExp;
                    }

                    else {
                        return Err(ConversionError::InvalidChar(c));
                    }
                },
                ParseState::ReadSubZero => {
                    if c.is_ascii_digit() {
                        digit_buffer.push(c);
                        exp -= 1;
                    }

                    else if c == '_' {}

                    else if c == 'e' || c == 'E' {
                        curr_state = ParseState::ReadExp;
                    }

                    else {
                        return Err(ConversionError::InvalidChar(c));
                    }
                },
                ParseState::ReadExp => {
                    exp_buffer.push(c);
                },
            }
        }

        match curr_state {
            ParseState::Init => {
                Err(ConversionError::NoData)
            },
            ParseState::InitZero => {
                Ok(F192::ZERO)
            },
            ParseState::ReadBase(base) => {
                let mut curr = 0;
                let mut exp = 1;
                let mut result = F192::ZERO;

                for digit in digit_buffer.into_iter() {
                    let n = digit as u64 - 32;

                    exp *= base as u64;
                    curr *= base as u64;
                    curr += n;

                    if exp > (1 << 56) {
                        result = result.mul(&F192::from(exp));
                        result = result.add(&F192::from(curr));

                        curr = 0;
                        exp = 1;
                    }
                }

                if exp > 1 {
                    result = result.mul(&F192::from(exp));
                    result = result.add(&F192::from(curr));
                }

                Ok(result)
            },
            ParseState::ReadDigit
            | ParseState::ReadSubZero
            | ParseState::ReadExp => {
                if !exp_buffer.is_empty() {
                    exp += exp_buffer.iter().collect::<String>().parse::<i64>().or(Err(ConversionError::TryFromIntError))?;

                    if digit_buffer.is_empty() {
                        return Ok(F192::ZERO);
                    }
                }

                // let's just ignore underflow
                if exp < -(EXP_COEFF as i64) {
                    return Ok(F192::ZERO);
                }

                if digit_buffer.len() > 39 {
                    exp += digit_buffer.len() as i64 - 39;
                    digit_buffer = digit_buffer[0..39].to_vec();
                }

                let mut result = F192::ZERO;

                while digit_buffer.len() > 6 {
                    result = result.mul_i32(1000000);
                    result = result.add(
                        &digit_buffer[0..6].iter().collect::<String>().parse::<u32>().unwrap().into()
                    );

                    digit_buffer = digit_buffer[6..].to_vec();
                }

                while digit_buffer.len() > 0 {
                    result = result.mul_i32(10);
                    result = result.add(&(digit_buffer[0] as u8 - '0' as u8).into());
                    digit_buffer = digit_buffer[1..].to_vec();
                }

                if is_neg {
                    result = result.neg();
                }

                if exp != 0 {
                    let exp = pow10(exp);

                    Ok(result.mul(&exp))
                }

                else {
                    Ok(result)
                }
            },
        }
    }
}

impl Display for F192 {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.to_approx_string())
    }
}

impl F192 {
    pub fn to_approx_string(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }

        // 2^70777 = 10^21306 + small
        let approx_log10 = (self.fast_log2() as i128 * 21306 / 70777 / 16777216) as i64;
        let mut n = self.abs();
        let mut exp = 0i64;

        if approx_log10 > 22 {
            n = n.div(&pow10(approx_log10 - 20));
            exp += approx_log10 - 20;
        }

        else if approx_log10 < 18 {
            n = n.mul(&pow10(20 - approx_log10));
            exp += approx_log10 - 20;
        }

        let mut n = u128::try_from(n).unwrap();

        let lower_bound = 100_000_000_000_000_000_000u128;
        let upper_bound = lower_bound * 10;

        while n >= upper_bound {
            n /= 10;
            exp += 1;
        }

        while n < lower_bound {
            n *= 10;
            exp -= 1;
        }

        let rem = n % 100;
        n /= 100;
        exp += 2;

        if rem >= 50 {
            n += 1;
        }

        while n % 10 == 0 {
            n /= 10;
            exp += 1;
        }

        let log10_n = log10(n);

        let ds = if exp >= 0 {
            let exp = exp as u32;

            // small integer
            if exp + log10_n < 10 {
                (n * 10u128.pow(exp)).to_string()
            }

            else if log10_n == 0 {
                format!("{n}e{exp}")
            }

            else {
                let ns = n.to_string();

                format!(
                    "{}.{}e{}",
                    ns.get(0..1).unwrap(),
                    ns.get(1..).unwrap(),
                    exp + log10_n,
                )
            }
        }

        else {
            if log10_n >= -exp as u32 {
                let index = (log10_n as i64 + exp + 1) as usize;
                let ns = n.to_string();
                let pre = ns.get(0..index).unwrap();
                let post = ns.get(index..).unwrap();

                format!("{pre}.{post}")
            }

            else {
                let zeros = (-exp - log10_n as i64 - 1) as usize;

                if zeros < 8 {
                    let mut result = format!("0.{}{n}", "0".repeat(zeros));

                    if result.len() > 22 {
                        result = result.get(0..22).unwrap().to_string();
                    }

                    while result.ends_with('0') {
                        result = result.trim_end_matches('0').to_string();
                    }

                    result
                }

                else if log10_n == 0 {
                    format!("{n}e{exp}")
                }

                else {
                    let ns = n.to_string();

                    format!(
                        "{}.{}e{}",
                        ns.get(0..1).unwrap(),
                        ns.get(1..).unwrap(),
                        exp + log10_n as i64,
                    )
                }
            }
        };

        if self.is_neg() {
            format!("-{ds}")
        }

        else {
            ds
        }
    }

    /// It turns the number into a 1 ~ 13 characters long string. It loses some precision.
    /// The result string only consists of `0-9a-zA-Z_`
    pub fn into_code13(&self) -> Result<Vec<u8>, ConversionError> {
        if self.is_zero() {
            return Ok(code13_zero());
        }

        let mut digits = (self.digits | SIGN_MASK) >> 57;
        let is_neg = self.is_neg();
        let mut exp = self.exp as i64 - EXP_COEFF as i64 + 57;

        while digits >= BASE * 64 {
            digits >>= 1;
            exp += 1;
        }

        let rem = digits & 31;
        digits >>= 5;
        exp += 5;

        if rem > 15 {
            digits += 1;

            if digits == BASE * 2 {
                digits = BASE;
                exp += 1;
            }
        }

        let d = digits - BASE;

        #[cfg(test)] assert!(d < BASE);

        exp += 64;

        // let's just ignore underflow
        if exp < -1921 {
            return Ok(code13_zero());
        }

        else if exp > 1922 {
            return Err(ConversionError::NotInRange {
                permitted: "TODO".to_string(),
                error: self.to_string(),
            });
        }

        let mut e = (exp.abs() as u64 * 4).max(2) - 2;
        e += (exp < 0) as u64 * 2;
        e += is_neg as u64;

        let result = vec![
            from_int((e / 124) as u8),
            from_int((e % 124 / 2) as u8),
            from_int((e % 2 * 31) as u8 + (d % 31) as u8),
            from_int((d / 419649682934170112) as u8),
            from_int((d / 6768543273131776 % 62) as u8),
            from_int((d / 109170052792448 % 62) as u8),
            from_int((d / 1760807303104 % 62) as u8),
            from_int((d / 28400117792 % 62) as u8),
            from_int((d / 458066416 % 62) as u8),
            from_int((d / 7388168 % 62) as u8),
            from_int((d / 119164 % 62) as u8),
            from_int((d / 1922 % 62) as u8),
            from_int((d / 31 % 62) as u8),
        ];

        let mut index = 0;

        while index != 12 && result[index] == '0' as u8 {
            index += 1;
        }

        Ok(result[index..].to_vec())
    }

    /// Inverse function of `into_code13`.
    pub fn from_code13(s: &[u8]) -> Result<Self, ConversionError> {
        if is_code13_zero(s) {
            return Ok(F192::ZERO);
        }

        let s = if s.len() <= 13 {
            if s.is_empty() {
                return Err(ConversionError::NoData);
            }

            vec![
                vec!['0' as u8; 13 - s.len()],
                s.to_vec()
            ].concat()
        } else {
            return Err(ConversionError::InvalidChar(s[13] as char));  // TODO: unexpected char
        };

        let e = to_int(s[0])? as i32 * 124
        + to_int(s[1])? as i32 * 2
        + to_int(s[2])? as i32 / 31;

        let mut d = 0;

        for c in s[3..].iter() {
            d += to_int(*c)? as u128;
            d *= 62;
        }

        d /= 2;
        d += (to_int(s[2])? % 31) as u128;

        d += BASE;

        let sign = (e % 2) * -2 + 1;
        let mut exp = (e + 2) / 4 * (e % 4 / 2 * 2 - 1);

        // now, the number is `sign * d * 2^(exp - 64)`

        let dlz = d.leading_zeros();
        d <<= dlz;
        exp -= dlz as i32;

        d &= !SIGN_MASK;
        d |= ((sign == -1) as u128) << 127;

        let exp = exp as i64;
        let f192_e = exp - 64 + EXP_COEFF as i64;

        Ok(F192 {
            digits: d,
            exp: f192_e as u64,
        })
    }
}

fn log10(mut n: u128) -> u32 {
    if n > 1_000_000_000 {
        log10(n / 1_000_000_000) + 9
    }

    else {
        let mut result = 0;

        while n >= 10 {
            n /= 10;
            result += 1;
        }

        result
    }
}

fn pow10(n: i64) -> F192 {
    let n_abs = n.abs() as u64;

    if n < 0 {
        F192::ONE.div(&pow10_worker(n_abs))
    }

    else {
        pow10_worker(n_abs)
    }
}

fn pow10_worker(n: u64) -> F192 {
    if n < 37 {
        F192::from((10u128).pow(n as u32))
    }

    else {
        let mut result = pow10_worker(n / 2);

        result = result.mul(&result);

        if n % 2 == 1 {
            result = result.mul(&F192::from(10));
        }

        result
    }
}

// 62^10 * 31
const BASE: u128 = 26018280341918546944;

pub fn code13_zero() -> Vec<u8> {
    vec![122, 121, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48]
}

fn is_code13_zero(s: &[u8]) -> bool {
    s == [122, 121, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48]
}

fn from_int(n: u8) -> u8 {
    #[cfg(test)] assert!(n < 62);

    if n < 10 {
        n + '0' as u8
    }

    else if n < 36 {
        n - 10 + 'A' as u8
    }

    else {
        n - 36 + 'a' as u8
    }
}

fn to_int(c: u8) -> Result<u8, ConversionError> {
    if c < '0' as u8 {
        Err(ConversionError::InvalidChar(c as char))
    }

    else if c <= '9' as u8 {
        Ok(c - '0' as u8)
    }

    else if c < 'A' as u8 {
        Err(ConversionError::InvalidChar(c as char))
    }

    else if c <= 'Z' as u8 {
        Ok(c - 'A' as u8 + 10)
    }

    else if c < 'a' as u8 {
        Err(ConversionError::InvalidChar(c as char))
    }

    else if c <= 'z' as u8 {
        Ok(c - 'a' as u8 + 36)
    }

    else {
        Err(ConversionError::InvalidChar(c as char))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ratio;
    use crate::fp192::testbench::{assert_f64_close, assert_very_close};

    #[test]
    fn code13_test() {
        let samples = vec![
            "1234".as_bytes().to_vec(),
            "abcdef".as_bytes().to_vec(),
            "0".as_bytes().to_vec(),
            "1".as_bytes().to_vec(),
            "100".as_bytes().to_vec(),
            "abcd0123abcd1".as_bytes().to_vec(),
            F192::PI.into_code13().unwrap(),
            F192::SQRT_2.into_code13().unwrap(),
            F192::from(202307162122u64).into_code13().unwrap(),
            F192::from(-202307162122i64).into_code13().unwrap(),
            F192::from(0).into_code13().unwrap(),
            F192::from(1).into_code13().unwrap(),
            F192::try_from(1e-49).unwrap().into_code13().unwrap(),
            F192::try_from(-1e-49).unwrap().into_code13().unwrap(),
        ];

        for sample in samples.into_iter() {
            let n = F192::from_code13(&sample).unwrap();
            let s = n.into_code13().unwrap();

            assert_eq!(sample, s);
        }

        let errors = vec![
            "!!!".as_bytes().to_vec(),
            "".as_bytes().to_vec(),
            "aaaaaaaaaaaaaaaaaaaaaaaaaaa".as_bytes().to_vec(),
        ];

        for error in errors.iter() {
            assert!(F192::from_code13(error).is_err());
        }

        assert!("1e5000".parse::<F192>().unwrap().into_code13().is_err());
        assert!("-1e5000".parse::<F192>().unwrap().into_code13().is_err());
        assert_eq!("1e-5000".parse::<F192>().unwrap().into_code13(), Ok(code13_zero()));
        assert_eq!("-1e-5000".parse::<F192>().unwrap().into_code13(), Ok(code13_zero()));
    }

    #[test]
    fn string_test() {
        let samples = vec![
            "0",
            "0.1234",
            "0.00123",
            "0.00123e6",
            "0.100123",
            "0.100123e6",
            "1.00123",
            "1.00123e6",
            "1.100123",
            "1.100123e6",
            "11.100123",
            "11.100123e6",
            "1.5",
            "1.5e6",
            "17e80",
            "17e-80",
            "1e72",
            "1e73",
            "1e74",
            "11111111111111111111111111111111111111111",
        ];

        for sample in samples.iter() {
            for sample in [format!("-{sample}"), sample.to_string()] {
                let n_rat = sample.parse::<Ratio>().unwrap();
                let n_f192 = sample.parse::<F192>().unwrap();
                let n_f64 = sample.parse::<f64>().unwrap();

                assert_very_close(F192::from(n_rat), n_f192);
                assert_f64_close(
                    F192::try_from(f64::from(n_f192)).unwrap(),
                    F192::try_from(n_f64).unwrap(),
                );
            }
        }

        assert_eq!("9999".parse::<F192>().unwrap(), F192::from(9999));
        assert_eq!("9999_9999".parse::<F192>().unwrap(), F192::from(9999_9999));
        assert_eq!("9_9999_9999_9999_9999_9999_9999_9999_9999_9999".parse::<F192>().unwrap(), F192::from(9_9999_9999_9999_9999_9999_9999_9999_9999_9999u128));
        assert_eq!("0x1234abcd".parse::<F192>().unwrap(), F192::from(0x1234abcd));
        assert_eq!("0b1100101010001100".parse::<F192>().unwrap(), F192::from(0b1100101010001100));
        assert_eq!("0o123777111".parse::<F192>().unwrap(), F192::from(0o123777111));

        let roundtrip_ = vec![
            "1230", "123", "12.3", "1.23",
            "0.123", "0.0123", "0.00123",
            "1.23e5000", "1.23e-5000",
            "1e5000", "1e-5000",
        ];
        let mut roundtrip = vec![];

        for r in roundtrip_.into_iter() {
            roundtrip.push(r.to_string());
            roundtrip.push(format!("-{r}"));
        }

        for sample in roundtrip.into_iter() {
            let n = sample.parse::<F192>().unwrap();

            assert_eq!(n.to_string(), sample.to_string());
        }
    }
}
