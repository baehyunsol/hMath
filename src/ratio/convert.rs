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

    /// if you don't know what `ieee754` is, you're okay to use this function
    pub fn from_ieee754_f32(n: f32) -> Self {
        todo!()
    }

    /// if you don't know what `ieee754` is, you're okay to use this function
    pub fn to_ieee754_f32(&self) -> Result<f32, ConversionError> {
        todo!()
    }

    /// if you don't know what `ieee754` is, you're okay to use this function
    pub fn from_ieee754_f64(n: f64) -> Self {
        todo!()
    }

    /// if you don't know what `ieee754` is, you're okay to use this function
    pub fn to_ieee754_f64(&self) -> Result<f64, ConversionError> {
        todo!()
    }

    /// BIGINT |\
    /// '-'? ('0' | [1-9] ([0-9] | '_')*) ('.' ([0-9] | '_')+)? (('e' | 'E') ('-' | '+')? [0-9]+)? |\
    /// `0x1.23` -> 1 + 2/16 + 3/256
    // `+-*/` 등등도 가능하게 할까?? -> 기본적인 사칙연산이 되면 훨씬 편할텐데... -> ㄷㄷㄷ 그럼 스크립트 언어 만드는 거임??
    pub fn from_string(s: &str) -> Result<Self, ConversionError> {
        todo!()
    }

}

#[cfg(test)]
mod tests {
    use crate::Ratio;

    #[test]
    fn ieee754_test() {
        let samples = vec![
            "0.0", "-0.0",
            "1.0", "-1.0",
            "2.0", "-2.0",
            "3.0", "-3.0",
            "1.125", "-1.125",
            "17.0", "-17.0",
            "1048576.0", "-1048576.0",
            "0.0625", "-0.0625",
            "0.01171875", "-0.01171875",
            "15.640625", "-15.640625",
            "625e-3", "-625e-3",
            "15.625e-2", "-15.625e-2",
            "12e3", "-12e3",
            "16.3e2", "-16.3e2",
            // TODO: denormalized numbers
        ];

        for n in samples.into_iter() {
            assert_eq!(Ratio::from_ieee754_f32(n.parse::<f32>().unwrap()), Ratio::from_string(n).unwrap());
            assert_eq!(Ratio::from_ieee754_f64(n.parse::<f64>().unwrap()), Ratio::from_string(n).unwrap());
        }

    }

}