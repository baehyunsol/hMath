use crate::big_int::{BigInt, BASE};


impl BigInt {

    pub fn from_u32(n: u32) -> Self {
        let mut result = BigInt {
            is_negative: false,
            data: vec![n]
        };

        result.trim();

        result
    }

    pub fn from_i32(n: i32) -> Self {
        let mut result = BigInt {
            is_negative: n < 0,
            data: vec![n.abs() as u32]
        };

        result.trim();
        result.trim_zero();

        result
    }

    pub fn from_u64(mut n: u64) -> Self {
        let mut data = vec![];
        let base_u64 = BASE as u64;

        while n > 0 {
            data.push((n % base_u64) as u32);
            n /= base_u64;
        }

        BigInt {
            is_negative: false,
            data
        }
    }

    pub fn from_i64(n: i64) -> Self {
        let is_negative = n < 0;
        let mut n = n.abs() as u64;
        let mut data = vec![];
        let base_u64 = BASE as u64;

        while n > 0 {
            data.push((n % base_u64) as u32);
            n /= base_u64;
        }

        BigInt {
            is_negative, data
        }
    }

    pub fn from_usize(n: usize) -> Self {
        BigInt::from_u64(n as u64)
    }

    pub fn from_isize(n: isize) -> Self {
        BigInt::from_i64(n as i64)
    }

    pub fn from_u128(mut n: u128) -> Self {
        let mut data = vec![];
        let base_u128 = BASE as u128;

        while n > 0 {
            data.push((n % base_u128) as u32);
            n /= base_u128;
        }

        BigInt {
            is_negative: false,
            data
        }
    }

    pub fn from_i128(n: i128) -> Self {
        let is_negative = n < 0;
        let mut n = n.abs() as u128;
        let mut data = vec![];
        let base_u128 = BASE as u128;

        while n > 0 {
            data.push((n % base_u128) as u32);
            n /= base_u128;
        }

        BigInt {
            is_negative, data
        }
    }

    pub fn to_u32(&self) -> Result<u32, &'static str> {

        if self.is_zero() {
            Ok(0)
        }

        else if self.len() == 1 {

            if !self.is_negative {
                Ok(self.data[0])
            }

            else {
                Err("u32 cannot be negative!")
            }

        }

        else {
            Err("too big to be u32")
        }

    }

    pub fn to_i32(&self) -> Result<i32, &'static str> {

        if self.is_zero() {
            Ok(0)
        }

        else if self.len() == 1 {

            if !self.is_negative {
                Ok(self.data[0] as i32)
            }

            else {
                Ok(-(self.data[0] as i32))
            }

        }

        else {
            Err("too big to be i32")
        }

    }

    pub fn from_string(st: String) -> Result<BigInt, &'static str> {
        _from_string(st.as_bytes())
    }

    pub fn to_string(&self) -> String {

        let mut n = self.abs();
        let mut digits = vec![];

        while n.len() > 0 {
            digits.push((&n % 10).to_u32().unwrap());
            n = &n / 10;
        }

        if digits.len() == 0 {
            digits.push(0);
        }

        digits.reverse();

        let mut result = digits.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("");

        if self.is_negative {
            result.insert(0, '-');
        }

        result
    }

}


fn _from_string(st: &[u8]) -> Result<BigInt, &'static str> {

    if st.len() == 0 {
        return Err("cannot convert an empty string!");
    }

    if st[0] == 45 {  // `-`
        let negated = _from_string(&st[1..])?;
        return Ok(-&negated);
    }

    let mut base = 10;
    let mut curr_index = 0;

    if st[0] == 48 && st.len() > 1 {

        if st[1] == 120 || st[1] == 88 {  // `0x`
            base = 16;
        }

        else if st[1] == 111 || st[1] == 79 {  // `0o`
            base = 8;
        }

        else if st[1] == 98 || st[1] == 66 {
            base = 2;
        }

        else {
            return Err("invalid character for base");
        }

        curr_index = 2;
    }

    if st[curr_index] == 95 {
        return Err("string cannot start with `_`");
    }

    let mut result = BigInt::zero();

    while curr_index < st.len() {

        if st[curr_index] == 95 {  // `_`
            curr_index += 1;
            continue;
        }

        let curr_digit = char_to_int(st[curr_index])?;

        if curr_digit >= base {
            return Err("invalid character!");
        }

        result = &result * base;
        result = &result + curr_digit;
        curr_index += 1;
    }

    Ok(result)
}


fn char_to_int(n: u8) -> Result<u8, &'static str> {

    if n < 48 {
        Err("not a valid character!")
    }

    else if n < 58 {
        Ok(n - 48)
    }

    else if n < 65 {
        Err("not a valid character!")
    }

    else if n < 71 {
        Ok(n - 55)
    }

    else if n < 97 {
        Err("not a valid character!")
    }

    else if n < 103 {
        Ok(n - 87)
    }

    else {
        Err("not a valid character!")
    }

}


#[cfg(test)]
mod tests {

    #[test]
    fn string_test() {
        use crate::big_int::BigInt;

        let n1 = BigInt::from_string("-0x1_000".to_string());
        let n2 = BigInt::from_string("0xc0ffee".to_string());
        let n3 = BigInt::from_string("0xf00d".to_string());
        let n4 = BigInt::from_string("256f".to_string());
        assert_eq!(n1.unwrap(), BigInt::from_i128(-0x1_000));
        assert_eq!(n2.unwrap(), BigInt::from_i128(0xc0ffee));
        assert_eq!(n3.unwrap(), BigInt::from_i128(0xf00d));
        assert!(n4.is_err());

        for _ in 0..0x400 {
            let n = rand::random::<i32>();
            let n_str = n.to_string();
            let n_str_n = BigInt::from_string(n_str).unwrap();
            let n_str_n_str = n_str_n.to_string();
            let n_str_n_str_n = BigInt::from_string(n_str_n_str).unwrap();

            assert_eq!(n, n_str_n_str_n.to_i32().unwrap());
        }

    }

}