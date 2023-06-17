use crate::UBigInt;
use crate::err::ConversionError;
use std::fmt;
use std::str::FromStr;

mod from;
mod into;

impl UBigInt {

    #[inline]
    pub fn from_u32(n: u32) -> Self {
        UBigInt::from_raw(vec![n])
    }

    pub fn to_u32(&self) -> Result<u32, ConversionError> {

        if self.len() > 1 {
            Err(ConversionError::NotInRange { permitted: "0~4.29e9".to_string(), error: self.to_scientific_notation(5) })
        }

        else {
            Ok(self.0[0])
        }

    }

    #[inline]
    pub fn from_u64(n: u64) -> Self {

        if n >= (1 << 32) {
            UBigInt::from_raw(vec![(n % (1 << 32)) as u32, (n >> 32) as u32])
        }

        else {
            UBigInt::from_raw(vec![n as u32])
        }

    }

    pub fn to_u64(&self) -> Result<u64, ConversionError> {

        if self.len() > 2 {
            Err(ConversionError::NotInRange { permitted: "0~1.84e19".to_string(), error: self.to_scientific_notation(5) })
        }

        else if self.len() == 2 {
            Ok(self.0[0] as u64 + ((self.0[1] as u64) << 32))
        }

        else {
            Ok(self.0[0] as u64)
        }

    }

    pub fn from_u128(n: u128) -> Self {

        if n < 1 << 32 {
            UBigInt::from_raw(vec![n as u32])
        }

        else if n < 1 << 64 {
            UBigInt::from_raw(vec![(n % (1 << 32)) as u32, (n >> 32) as u32])
        }

        else if n < 1 << 96 {
            UBigInt::from_raw(vec![(n % (1 << 32)) as u32, ((n >> 32) % (1 << 32)) as u32, (n >> 64) as u32])
        }

        else {
            UBigInt::from_raw(vec![(n % (1 << 32)) as u32, ((n >> 32) % (1 << 32)) as u32, ((n >> 64) % (1 << 32)) as u32, (n >> 96) as u32])
        }

    }

    pub fn to_u128(&self) -> Result<u128, ConversionError> {

        match self.len() {
            1 => Ok(self.0[0] as u128),
            2 => Ok(self.0[0] as u128 + ((self.0[1] as u128) << 32)),
            3 => Ok(self.0[0] as u128 + ((self.0[1] as u128) << 32) + ((self.0[2] as u128) << 64)),
            4 => Ok(self.0[0] as u128 + ((self.0[1] as u128) << 32) + ((self.0[2] as u128) << 64) + ((self.0[3] as u128) << 96)),
            _ => Err(ConversionError::NotInRange { permitted: "0~3.4e38".to_string(), error: self.to_scientific_notation(5) })
        }

    }

    /// `('0' '_'*) | ([1-9] ([0-9] | '_')*)`\
    /// `('0x' | '0X') ([0-9a-fA-F] | '_')+`\
    /// `('0b' | '0B') ('0' | '1' | '_')+`\
    /// `('0o' | '0O') ([0-7] | '_')+`
    pub fn from_string(s: &str) -> Result<Self, ConversionError> {
        let mut curr_state = StringToNumFSM::Init;
        let mut int_buffer = 0;
        let mut ubi_buffer = UBigInt::zero();
        let mut int_buffer_coeff = 1;
        let mut base = 10;

        for c in s.chars() {

            if c == '_' && curr_state != StringToNumFSM::Init {
                continue;
            }

            match curr_state {
                StringToNumFSM::Init => {

                    if c == '0' {
                        curr_state = StringToNumFSM::InitialZero;
                    }

                    else if let Some(n) = c.to_digit(10) {
                        curr_state = StringToNumFSM::ReadNum;
                        int_buffer = n;
                        int_buffer_coeff = 10;
                    }

                    else {
                        return Err(ConversionError::InvalidChar(c));
                    }

                }
                StringToNumFSM::InitialZero => {

                    if c.to_ascii_lowercase() == 'x' {
                        curr_state = StringToNumFSM::InitNum;
                        base = 16;
                    }

                    else if c.to_ascii_lowercase() == 'b' {
                        curr_state = StringToNumFSM::InitNum;
                        base = 2;
                    }

                    else if c.to_ascii_lowercase() == 'o' {
                        curr_state = StringToNumFSM::InitNum;
                        base = 8;
                    }

                    else {
                        return Err(ConversionError::InvalidChar(c));
                    }

                }
                StringToNumFSM::InitNum => {

                    match c.to_digit(16) {
                        Some(n) if n < base => {
                            curr_state = StringToNumFSM::ReadNum;
                            int_buffer = n;
                            int_buffer_coeff = base;
                        }
                        _ => {
                            return Err(ConversionError::InvalidChar(c));
                        }
                    }

                }
                StringToNumFSM::ReadNum => {

                    match c.to_digit(16) {
                        Some(n) if n < base => {
                            int_buffer *= base;
                            int_buffer += n;
                            int_buffer_coeff *= base;

                            if int_buffer_coeff > 0x8_000_000 {
                                ubi_buffer.mul_u32_mut(int_buffer_coeff);
                                ubi_buffer.add_u32_mut(int_buffer);

                                int_buffer = 0;
                                int_buffer_coeff = 1;
                            }

                        }
                        _ => {
                            return Err(ConversionError::InvalidChar(c));
                        }
                    }

                }
            }

        }

        match curr_state {
            StringToNumFSM::InitialZero => Ok(UBigInt::zero()),
            StringToNumFSM::ReadNum => {

                if int_buffer_coeff > 1 {
                    ubi_buffer.mul_u32_mut(int_buffer_coeff);
                    ubi_buffer.add_u32_mut(int_buffer);
                }

                Ok(ubi_buffer)
            }
            StringToNumFSM::Init => Err(ConversionError::NoData),  // empty string
            StringToNumFSM::InitNum => Err(ConversionError::NoData),  // no number
        }

    }

    /// `9.8e5`
    pub fn to_scientific_notation(&self, digits_max_len: usize) -> String {
        let mut n = self.clone();
        let mut exp = 0;

        if n.len() > 8 {
            // 10^64
            let big_number = UBigInt::from_raw(vec![0, 0, 3211403009, 1849224548, 3668416493, 3913284084, 1593091]);

            while n.len() > 8 {
                n.div_ubi_mut(&big_number);
                exp += 64;
            }

        }

        while n.len() > 2 {
            n.div_u32_mut(1_000_000_000);
            exp += 9;
        }

        let n = n.to_u64().unwrap();

        let (digits, new_exp) = _to_scientific_notation(n, digits_max_len);

        format!("{digits}e{}", exp + new_exp)
    }

    pub fn to_string_dec(&self) -> String {
        let mut n = self.clone();
        let mut buffer = Vec::with_capacity(self.len());

        while n.len() > 1 {
            buffer.push(format!("{:09}", n.rem_u32(1_000_000_000).0[0]));
            n.div_u32_mut(1_000_000_000);
        }

        buffer.push(n.0[0].to_string());
        buffer.reverse();

        buffer.concat()
    }

    /// `prefix` is for '0x'
    pub fn to_string_hex(&self, prefix: bool) -> String {
        let mut buffer = self.0.iter().map(
            |n| format!("{:08x}", n)
        ).collect::<Vec<String>>();

        if prefix { buffer.push("0x".to_string()); } else { buffer.push(String::new()); }

        buffer.reverse();
        buffer[1] = buffer[1].trim_start_matches('0').to_string();

        if buffer[1].len() == 0 {
            buffer[1] = "0".to_string();
        }

        buffer.concat()
    }

    /// `prefix` is for '0o'
    pub fn to_string_oct(&self, prefix: bool) -> String {
        let mut n = self.clone();
        let mut buffer = Vec::with_capacity(self.len());

        while n.len() > 1 {
            buffer.push(format!("{:010o}", n.rem_pow2(0o10_000_000_000).0[0]));
            n.div_u32_mut(0o10_000_000_000);
        }

        buffer.push(format!("{:o}", n.0[0]));

        if prefix { buffer.push("0o".to_string()); } else { buffer.push(String::new()); }

        buffer.reverse();
        buffer[1] = buffer[1].trim_start_matches('0').to_string();

        if buffer[1].len() == 0 {
            buffer[1] = "0".to_string();
        }

        buffer.concat()
    }

    /// `prefix` is for '0b'
    pub fn to_string_bin(&self, prefix: bool) -> String {
        let mut buffer = self.0.iter().map(
            |n| format!("{:032b}", n)
        ).collect::<Vec<String>>();

        if prefix { buffer.push("0b".to_string()); } else { buffer.push(String::new()); }

        buffer.reverse();
        buffer[1] = buffer[1].trim_start_matches('0').to_string();

        if buffer[1].len() == 0 {
            buffer[1] = "0".to_string();
        }

        buffer.concat()
    }

}

#[derive(PartialEq)]
enum StringToNumFSM {
    Init,
    InitialZero,
    InitNum,
    ReadNum,
}

impl fmt::Display for UBigInt {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string_dec())
    }

}

impl fmt::LowerHex for UBigInt {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string_hex(fmt.alternate()))
    }

}

impl fmt::Octal for UBigInt {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string_oct(fmt.alternate()))
    }

}

impl fmt::Binary for UBigInt {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string_bin(fmt.alternate()))
    }

}

impl fmt::LowerExp for UBigInt {

    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_scientific_notation(5))
    }

}

impl FromStr for UBigInt {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        UBigInt::from_string(s)
    }

}

pub fn _to_scientific_notation(mut n: u64, digits_max_len: usize) -> (String, usize) {
    let mut exp = 0;
    let mut digits = Vec::with_capacity(20);

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
        exp += 1;
    }

    if digits.len() > 0 {
        exp -= 1;
    }

    digits.reverse();

    while digits.len() > digits_max_len || digits.len() > 1 && digits[digits.len() - 1] == 0 {
        digits.pop().unwrap();
    }

    let digits = if digits.len() == 0 {
        format!("0")
    } else if digits.len() == 1 {
        format!("{}", digits[0])
    } else {
        format!("{}.{}", digits[0], digits[1..].iter().map(|c| c.to_string()).collect::<Vec<String>>().join(""))
    };

    (digits, exp)
}

#[cfg(test)]
mod tests {
    use crate::UBigInt;
    use crate::consts::RUN_ALL_TESTS;

    #[test]
    fn integer_conversion_test() {
        if !RUN_ALL_TESTS { return; }
        let mut n: u128 = 0;

        for _ in 0..31 {
            let ubi32 = UBigInt::from_u32(n as u32);
            let ubi64 = UBigInt::from_u64(n as u64);
            let ubi128 = UBigInt::from_u128(n);

            assert_eq!(ubi32, ubi64);
            assert_eq!(ubi64, ubi128);

            assert_eq!(ubi32.to_u32().unwrap(), n as u32);
            assert_eq!(ubi64.to_u64().unwrap(), n as u64);
            assert_eq!(ubi128.to_u128().unwrap(), n);

            n *= 2;
            n += 1;
        }

        for _ in 0..31 {
            let ubi64 = UBigInt::from_u64(n as u64);
            let ubi128 = UBigInt::from_u128(n);

            assert_eq!(ubi64, ubi128);

            assert_eq!(ubi64.to_u64().unwrap(), n as u64);
            assert_eq!(ubi128.to_u128().unwrap(), n);

            n *= 2;
            n += 1;
        }

        for _ in 0..64 {
            let ubi128 = UBigInt::from_u128(n);
            assert_eq!(ubi128.to_u128().unwrap(), n);

            n *= 2;
            n += 1;
        }

    }

    #[test]
    fn scientific_notation_test() {
        use crate::Ratio;

        let samples = vec![
            "1e0", "1e1", "0e0",
            "2e1", "2e0", "3.14e2",
            "3.14e6", "3.1415e200",
            "2.7182e20000", "3e3"
        ];

        for sample in samples.iter() {
            let n = Ratio::from_string(sample).unwrap().truncate_bi().to_ubi().unwrap();
            assert_eq!(sample.to_string(), n.to_scientific_notation(5));
        }

    }

    #[test]
    fn string_conversion_test() {

        if !RUN_ALL_TESTS { return; }
        assert_eq!(UBigInt::from_raw(vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1]), UBigInt::from_string("2135987035920910082395021706169552114602704522356652769947041607822219725780658996767035796488192").unwrap());

        let samples = vec![
            ("0", 0),
            ("0_", 0_),
            ("1", 1),
            ("1_000", 1_000),
            ("0x10_000", 0x10_000),
            ("0xc0ffee", 0xc0ffee),
            ("12", 12),
            ("123", 123),
            ("1234", 1234),
            ("12345", 12345),
            ("123456", 123456),
            ("1234567", 1234567),
            ("12345678", 12345678),
            ("123456789", 123456789),
            ("0x1", 0x1),
            ("0x12", 0x12),
            ("0x123", 0x123),
            ("0x1234", 0x1234),
            ("0x12345", 0x12345),
            ("0x123456", 0x123456),
            ("0x1234567", 0x1234567),
            ("0x12345678", 0x12345678),
            ("0b1", 0b1),
            ("0b1010", 0b1010),
            ("0b1010101", 0b1010101),
            ("0b1010101010", 0b1010101010),
            ("0b1010101010101", 0b1010101010101),
            ("0b1010101010101010", 0b1010101010101010),
            ("0b1010101010101010101", 0b1010101010101010101),
            ("0b1010101010101010101010", 0b1010101010101010101010),
            ("0x80_000_000", 0x80_000_000),
        ];

        let big_numbers = vec![
            "1000_0000_0000",
            "1000_0000_0000_0000_0000",
            "0x123456789abcdef10111213141516",
            "0xbad_c0ffee",
            "0xffff_ffff_ffff_ffff_ffff_ffff_ffff",
            "12345678910111213141516",
            "1_3076_7436_8000",
            "0o123456701234567012345670",
            "0o123456701234567012345670123",
            "0o1234567012345670123456701234",
            "0b101100111000111100001111100000111111000000111111100000001111111100000000",
            "1234567891",
            "123456789123",
            "12345678912345",
            "1234567891234567",
            "123456789123456789",
            "12345678912345678912",
            "1234567891234567891234",
            "123456789123456789123456",
            "12345678912345678912345678",
            "1234567891234567891234567891",
            "123456789123456789123456789123",
            "12345678912345678912345678912345",
            "1234567891234567891234567891234567",
            "123456789123456789123456789123456789",
            "12345678912345678912345678912345678912",
            "1234567891234567891234567891234567891234",
            "123456789123456789123456789123456789123456",
            "12345678912345678912345678912345678912345678",
            "1234567891234567891234567891234567891234567891",
            "123456789123456789123456789123456789123456789123",
            "12345678912345678912345678912345678912345678912345",
            "1234567891234567891234567891234567891234567891234567",
            "0x1234567891",
            "0x123456789123",
            "0x12345678912345",
            "0x1234567891234567",
            "0x123456789123456789",
            "0x12345678912345678912",
            "0x1234567891234567891234",
            "0x123456789123456789123456",
            "0x12345678912345678912345678",
            "0x1234567891234567891234567891",
            "0x123456789123456789123456789123",
            "0x12345678912345678912345678912345",
            "0x1234567891234567891234567891234567",
            "0x123456789123456789123456789123456789",
            "0x12345678912345678912345678912345678912",
            "0x1234567891234567891234567891234567891234",
            "0x123456789123456789123456789123456789123456",
            "0x12345678912345678912345678912345678912345678",
            "0x1234567891234567891234567891234567891234567891",
            "0x123456789123456789123456789123456789123456789123",
            "0x12345678912345678912345678912345678912345678912345",
            "0x1234567891234567891234567891234567891234567891234567",
        ];

        let invalid_samples = vec![
            "00", "_0", "-0", "_1",
            "00x123", "0x", "", "0c1",
            "0b123"
        ];

        for (string, number) in samples.into_iter() {
            let ubi = UBigInt::from_string(string).unwrap();
            let ubi2 = UBigInt::from_u32(number);
            assert_eq!(ubi, ubi2);
            assert_eq!(UBigInt::from_string(&ubi.to_string_dec()).unwrap(), ubi2);
            assert_eq!(UBigInt::from_string(&ubi.to_string_hex(true)).unwrap(), ubi2);
            assert_eq!(UBigInt::from_string(&ubi.to_string_oct(true)).unwrap(), ubi2);
            assert_eq!(UBigInt::from_string(&ubi.to_string_bin(true)).unwrap(), ubi2);

            assert_eq!(UBigInt::from_string(string).unwrap().to_string_dec().parse::<u32>().unwrap(), number);
        }

        for big_number in big_numbers.into_iter() {
            let ubi = UBigInt::from_string(big_number).unwrap();
            assert_eq!(UBigInt::from_string(&ubi.to_string_dec()).unwrap(), ubi);
            assert_eq!(UBigInt::from_string(&ubi.to_string_hex(true)).unwrap(), ubi);
            assert_eq!(UBigInt::from_string(&ubi.to_string_oct(true)).unwrap(), ubi);
            assert_eq!(UBigInt::from_string(&ubi.to_string_bin(true)).unwrap(), ubi);
        }

        for sample in invalid_samples.into_iter() {
            assert!(UBigInt::from_string(sample).is_err());
        }

    }
}
