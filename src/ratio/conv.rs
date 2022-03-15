use crate::{Ratio, BigInt};

impl Ratio {

    #[inline]
    pub fn from_u32s(denom: u32, numer: u32) -> Ratio {
        Ratio::new(BigInt::from_u32(denom), BigInt::from_u32(numer))
    }

    #[inline]
    pub fn from_u32(n: u32) -> Ratio {
        Ratio {denom: BigInt::one(), numer: BigInt::from_u32(n)}
    }

    #[inline]
    pub fn from_u64s(denom: u64, numer: u64) -> Ratio {
        Ratio::new(BigInt::from_u64(denom), BigInt::from_u64(numer))
    }

    #[inline]
    pub fn from_u64(n: u64) -> Ratio {
        Ratio {denom: BigInt::one(), numer: BigInt::from_u64(n)}
    }

    #[inline]
    pub fn from_i32s(denom: i32, numer: i32) -> Ratio {
        Ratio::new(BigInt::from_i32(denom), BigInt::from_i32(numer))
    }

    #[inline]
    pub fn from_i32(n: i32) -> Ratio {
        Ratio {denom: BigInt::one(), numer: BigInt::from_i32(n)}
    }

    #[inline]
    pub fn from_i64s(denom: i64, numer: i64) -> Ratio {
        Ratio::new(BigInt::from_i64(denom), BigInt::from_i64(numer))
    }

    #[inline]
    pub fn from_i64(n: i64) -> Ratio {
        Ratio {denom: BigInt::one(), numer: BigInt::from_i64(n)}
    }

    #[inline]
    pub fn from_big_int(n: BigInt) -> Ratio {
        Ratio {denom: BigInt::one(), numer: n}
    }

    #[inline]
    pub fn from_big_ints(denom: BigInt, numer: BigInt) -> Ratio {
        Ratio::new(denom, numer)
    }

    pub fn from_string(s: String) -> Result<Ratio, &'static str> {
        _from_string(s.as_bytes())
    }

    pub fn to_string(&self) -> String {
        panic!("Not Implemented!")
    }

}


fn _from_string(st: &[u8]) -> Result<Ratio, &'static str> {

    if st.len() == 0 {
        Err("cannot convert an empty string!")
    }

    else if st[0] == 45 {  // `-`
        let negated = _from_string(&st[1..])?;

        Ok(-&negated)
    }

    else if st[0] == 46 {  // `.4` -> `0.4`
        let mut st = st.to_vec();
        st.insert(0, 48);
        _from_string(&st)
    }

    else if st[0] == 95 {  // `_`
        Err("num literal cannot start with `_`")
    }

    else if st.contains(&101) || st.contains(&69) {  // `e` || `E`
        let exp_split: Vec<&[u8]> = st.split(|c| c == &101 || c == &69).collect();

        if exp_split.len() > 2 {
            Err("Multiple `E` notation does not make sense!")
        }

        else {
            let result = _from_string(exp_split[0])?;
            let exp = BigInt::from_string(String::from_utf8(exp_split[1].to_vec()).unwrap())?.to_i32()?;
            let powered = BigInt::from_u32(10).pow(exp.abs() as u32);

            if exp < 0 {
                Ok(&result / &powered)
            }

            else {
                Ok(&result * &powered)
            }

        }

    }

    else if st.contains(&46) {  // '.'
        let frac_split: Vec<&[u8]> = st.split(|c| c == &46).collect();

        if frac_split.len() > 2 {
            Err("Multiple `.` notation does not make sense!")
        }

        else {
            let integer = BigInt::from_string(String::from_utf8(frac_split[0].to_vec()).unwrap())?;

            let mut frac_string = frac_split[1].to_vec();
            frac_string = frac_string.into_iter().filter(|c| c != &95).collect();  // remove `_`s.

            if frac_string.len() == 0 {
                return Err("num literal cannot start with `_`");
            }

            let exp = frac_string.len();

            // remove prefixed `0`s
            frac_string = {

                while frac_string[0] == 48 && frac_string.len() > 1 {
                    frac_string = frac_string[1..].to_vec();
                }

                frac_string
            };

            let frac = BigInt::from_string(String::from_utf8(frac_string).unwrap())?;

            if frac.is_negative {
                return Err("fractional part cannot be negative!");
            }

            let denom = BigInt::from_u32(10).pow(exp as u32);
            let numer = &(&integer * &denom) + &frac;

            Ok(Ratio::new(denom, numer))
        }

    }

    else {
        let num = BigInt::from_string(String::from_utf8(st.to_vec()).unwrap())?;
        Ok(Ratio::from_big_int(num))
    }

}


#[cfg(test)]
mod tests {

    #[test]
    fn string_test() {
        use crate::Ratio;

        let oks = vec![
            (Ratio::from_string("3.001".to_string()).unwrap(), Ratio::from_i32s(1000, 3001)),
            (Ratio::from_string("-4.2".to_string()).unwrap(), Ratio::from_i32s(5, -21)),
            (Ratio::from_string("24".to_string()).unwrap(), Ratio::from_i32s(1, 24)),
            (Ratio::from_string("3.1415".to_string()).unwrap(), Ratio::from_i32s(10000, 31415)),
            (Ratio::from_string("3.14_15".to_string()).unwrap(), Ratio::from_i32s(10000, 31415)),
            (Ratio::from_string(".4".to_string()).unwrap(), Ratio::from_i32s(5, 2)),
            (Ratio::from_string("-.4".to_string()).unwrap(), Ratio::from_i32s(-5, 2)),
            (Ratio::from_string("-.0".to_string()).unwrap(), Ratio::from_i32s(1, 0)),
            (Ratio::from_string(".0".to_string()).unwrap(), Ratio::from_i32s(1, 0)),
            (Ratio::from_string(".0e3".to_string()).unwrap(), Ratio::from_i32s(1, 0)),
            (Ratio::from_string(".0e-3".to_string()).unwrap(), Ratio::from_i32s(1, 0)),
            (Ratio::from_string("2e3".to_string()).unwrap(), Ratio::from_i32s(1, 2000)),
            (Ratio::from_string("2e-3".to_string()).unwrap(), Ratio::from_i32s(500, 1)),
            (Ratio::from_string("3.14e3".to_string()).unwrap(), Ratio::from_i32s(1, 3140)),
            (Ratio::from_string("3.14e-3".to_string()).unwrap(), Ratio::from_i32s(100000, 314)),
            (Ratio::from_string("3.14e0".to_string()).unwrap(), Ratio::from_i32s(100, 314)),
        ];

        oks.iter().for_each(|ok| assert_eq!(ok.0, ok.1));

        let errors = vec![
            Ratio::from_string("_.001".to_string()),
            Ratio::from_string("1._".to_string()),
            Ratio::from_string("3..1".to_string()),
            Ratio::from_string("3.1.2".to_string()),
            Ratio::from_string("-3.-12".to_string())
        ];
        assert!(errors.iter().all(|e| e.is_err()));
    }

}