use crate::{Ratio, BigInt};

impl Ratio {

    pub fn from_f32(f: f32) -> Ratio {
        panic!("Not Implemented!")
    }

    pub fn from_f64(f: f64) -> Ratio {
        panic!("Not Implemented!")
    }

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

    else if st[0] == 95 {  // `_`
        Err("num literal cannot start with `_`")
    }

    else if st.contains(&46) {  // '.'
        panic!("Not Implemented Yet!")
    }

    else {
        let num = BigInt::from_string(String::from_utf8(st.to_vec()).unwrap())?;
        Ok(Ratio::from_big_int(num))
    }

}