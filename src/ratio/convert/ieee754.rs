use crate::{Ratio, BigInt, ConversionError};

impl Ratio {

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// Though the ieee 754 standard distinguishes negative 0 and positive 0, it doesn't distinguish between them.
    /// It returns an error if `n` is NaN or Inf.
    pub fn from_ieee754_f32(n: f32) -> Result<Self, ConversionError> {
        match inspect_ieee754_f32(n) {
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
            Err(e) => Err(e),
        }
    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// This function does not return `f32::NAN` or `f32::INFINITY` for overflows, but it returns `ConversionError::NotInRange` instead.
    /// For underflow numbers like `1-e20000`, it returns 0.
    pub fn to_ieee754_f32(&self) -> Result<f32, ConversionError> {
        if self.is_zero() {
            return Ok(0.0)
        }

        let mut self_clone = self.clone();

        let neg = self_clone.is_neg();
        let mut exp = 0i32;

        if neg { self_clone.neg_mut(); }

        let approx_log2 = self.numer.log2().sub(&self.denom.log2());

        if approx_log2.gt_i32(128) {
            return Err(ConversionError::NotInRange { permitted: "1.18e-38~3.4e38".to_string(), error: self.to_scientific_notation(5) });
        }

        let approx_log2 = approx_log2.to_i32().unwrap();

        if approx_log2 < -2 {
            self_clone.mul_bi_mut(&BigInt::exp2(approx_log2.abs() as u64));
            exp += approx_log2;
        }

        else if approx_log2 > 2 {
            self_clone.div_bi_mut(&BigInt::exp2(approx_log2 as u64));
            exp += approx_log2;
        }

        while self_clone.gt_one() {
            self_clone.div_i32_mut(2);
            exp += 1;
        }

        while self_clone.lt_one() {
            self_clone.mul_i32_mut(2);
            exp -= 1;
        }

        let (mut frac, frac_rem) = self_clone.sub_i32(1).mul_i32(1 << 23).truncate_and_frac();

        // IEEE754 chooses the closest approximation
        if frac_rem.gt(&Ratio::from_denom_and_numer_i32(2, 1)) {
            frac.add_i32_mut(1);
        }

        let mut frac = frac.to_i32().unwrap() as u32;

        if exp < -126 {
            frac += 1 << 23;
            frac *= 16;

            if exp < -127 {
                frac >>= (-127 - exp).min(31);
                exp = -127;
            }

            // it has to choose the closest value
            if frac % 32 > 15 {
                frac /= 32;
                frac += 1;
            }

            else {
                frac /= 32;
            }

            if frac == 0 {
                return Ok(0.0);
            }
        }

        exp += 127;

        if exp < 0 || exp > 255 {
            return Err(ConversionError::NotInRange { permitted: "1.18e-38~3.4e38".to_string(), error: self.to_scientific_notation(5) });
        }

        let result = ((neg as u32) << 31) | ((exp as u32) << 23) | frac;
        let rp = &result as *const u32 as *const f32;

        unsafe { Ok(*rp) }
    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// Though the ieee 754 standard distinguishes negative 0 and positive 0, it doesn't distinguish between them.
    /// It returns an error if `n` is NaN or Inf.
    pub fn from_ieee754_f64(n: f64) -> Result<Self, ConversionError> {
        match inspect_ieee754_f64(n) {
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
            Err(e) => Err(e),
        }
    }

    /// If you don't know what `ieee754` is, you're okay to use this function.
    /// This function does not return `f64::NAN` or `f64::INFINITY` for overflows, but it returns `ConversionError::NotInRange` instead.
    /// For underflow numbers like `1-e20000`, it returns 0.
    pub fn to_ieee754_f64(&self) -> Result<f64, ConversionError> {
        if self.is_zero() {
            return Ok(0.0)
        }

        let mut self_clone = self.clone();

        let neg = self_clone.is_neg();
        let mut exp = 0i32;

        if neg { self_clone.neg_mut(); }

        let approx_log2 = self.numer.log2().sub(&self.denom.log2());

        if approx_log2.gt_i32(1024) {
            return Err(ConversionError::NotInRange { permitted: "2.23e-308~1.8e308".to_string(), error: self.to_scientific_notation(5) });
        }

        let approx_log2 = approx_log2.to_i32().unwrap();

        if approx_log2 < -2 {
            self_clone.mul_bi_mut(&BigInt::exp2(approx_log2.abs() as u64));
            exp += approx_log2;
        }

        else if approx_log2 > 2 {
            self_clone.div_bi_mut(&BigInt::exp2(approx_log2 as u64));
            exp += approx_log2;
        }

        while self_clone.gt_one() {
            self_clone.div_i32_mut(2);
            exp += 1;
        }

        while self_clone.lt_one() {
            self_clone.mul_i32_mut(2);
            exp -= 1;
        }

        let (mut frac, frac_rem) = self_clone.sub_i32(1).mul_bi(&BigInt::from_i64(1 << 52)).truncate_and_frac();

        // IEEE754 chooses the closest approximation
        if frac_rem.gt(&Ratio::from_denom_and_numer_i32(2, 1)) {
            frac.add_i32_mut(1);
        }

        let mut frac = frac.to_i64().unwrap() as u64;

        if exp < -1022 {
            frac += 1 << 52;
            frac *= 16;

            if exp < -1023 {
                frac >>= (-1023 - exp).min(63);
                exp = -1023;
            }

            // it has to choose the closest value
            if frac % 32 > 15 {
                frac /= 32;
                frac += 1;
            }

            else {
                frac /= 32;
            }

            if frac == 0 {
                return Ok(0.0);
            }
        }

        exp += 1023;

        if exp < 0 || exp > 2047 {
            return Err(ConversionError::NotInRange { permitted: "2.23e-308~1.8e308".to_string(), error: self.to_scientific_notation(5) });
        }

        let result = ((neg as u64) << 63) | ((exp as u64) << 52) | frac;
        let rp = &result as *const u64 as *const f64;

        unsafe { Ok(*rp) }
    }
}

/// You may find this function useful when you're dealing with [ieee 754 numbers](https://en.wikipedia.org/wiki/IEEE_754).\
/// This function returns `(neg, exp, frac)`, which means `n` is `(-1)^(neg) * 2^(exp) * (1 + frac/2^23)` regardless of denormalization.\
/// It returns (false, i32::MIN, 0) when n is 0.
pub fn inspect_ieee754_f32(n: f32) -> Result<(bool, i32, u32), ConversionError> {  // (neg, exp, frac)
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
        exp += 1;

        if frac == 0 {
            return Ok((false, i32::MIN, 0));
        }

        if frac < (1 << 23) {
            let to_shift = 23 - frac.ilog2();
            frac <<= to_shift;
            exp -= to_shift as i32;
        }

        frac -= 1 << 23;
    }

    else if exp == 128 {
        if frac == 0 {
            if neg {
                return Err(ConversionError::NegInfinity);
            }

            else {
                return Err(ConversionError::Infinity);
            }
        }

        else {
            return Err(ConversionError::NotANumber);
        }
    }

    // if neg { -1 } else { 1 } * 2^exp * (1 + frac / 2^23)
    Ok((neg, exp, frac))
}

/// You may find this function useful when you're dealing with [ieee 754 numbers](https://en.wikipedia.org/wiki/IEEE_754).\
/// This function returns `(neg, exp, frac)`, which means `n` is `(-1)^(neg) * 2^(exp) * (1 + frac/2^52)` regardless of denormalization.\
/// It returns (false, i32::MIN, 0) when n is 0.
pub fn inspect_ieee754_f64(n: f64) -> Result<(bool, i32, u64), ConversionError> {  // (neg, exp, frac)
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
        exp += 1;

        if frac == 0 {
            return Ok((false, i32::MIN, 0));
        }

        if frac < (1 << 52) {
            let to_shift = 52 - frac.ilog2();
            frac <<= to_shift;
            exp -= to_shift as i32;
        }

        frac -= 1 << 52;
    }

    else if exp == 1024 {
        if frac == 0 {
            if neg {
                return Err(ConversionError::NegInfinity);
            }

            else {
                return Err(ConversionError::Infinity);
            }
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
    use crate::consts::RUN_ALL_TESTS;
    use super::*;

    #[cfg(feature = "rand")]
    use crate::err::ConversionError;

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

            // 19 * 2^(-129)
            "0.000000000000000000000000000000000000027917990832029328314257492759028334848193306973337078635832853024112409912049770355224609375",

            // 3 * 2^(-136)
            "0.0000000000000000000000000000000000000000344383110592467043350215782389329788423437174835572516067017101448755056480877101421356201171875",
        ];

        let samples: Vec<String> = vec![
            samples.iter().map(|s| format!("-{s}")).collect::<Vec<String>>(),
            samples.iter().map(|s| s.to_string()).collect(),
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

        // numbers that single precision IEEE754 cannot represent perfectly
        let samples2 = vec![
            "3.1", "3.14", "3.141", "3.1415", "3.14159",
            "2.7", "2.71", "2.718", "2.7182", "2.71828",
            "4e-40", "4e-320", "4e-310", "4e-300",
            "0.00123", "0.00123e4", "0.00123e-4",
            "1.00123", "1.00123e4", "1.00123e-4",
            "1307674368000",
        ];

        for n in samples2.into_iter() {
            let num1 = Ratio::from_ieee754_f32(n.parse::<f32>().unwrap()).unwrap();
            let num2 = Ratio::from_ieee754_f32(Ratio::from_string(n).unwrap().to_ieee754_f32().unwrap()).unwrap();
            assert_eq!(num1, num2);

            let num3 = Ratio::from_ieee754_f64(n.parse::<f64>().unwrap()).unwrap();
            let num4 = Ratio::from_ieee754_f64(Ratio::from_string(n).unwrap().to_ieee754_f64().unwrap()).unwrap();
            assert_eq!(num3, num4);
        }

    }

    #[test]
    fn ieee754_subnormal_test() {
        let sub_normal_f32 = vec![
            0b0_00000000_00000000000000000000001,
            0b0_00000000_00000000000000000000111u32,
            0b1_00000000_01100000000001011010101,
            0b1_00000000_01000000011100110101010,
            0b1_00000000_00000010111101101010011,
            0b1_00000000_00001100111010110000100,
            0b0_00000000_00001010111101000101000,
            0b0_00000000_01111011011111110101100,
        ];

        for n32 in sub_normal_f32.into_iter() {
            let nf32 = unsafe { *(&n32 as *const u32 as *const f32) };
            let rat32 = Ratio::from_ieee754_f32(nf32).unwrap();
            let nf32_ = rat32.to_ieee754_f32().unwrap();
            let rat32_ = Ratio::from_ieee754_f32(nf32_).unwrap();

            assert_eq!(rat32, rat32_);
            assert_eq!(nf32, nf32_);
        }

        let sub_normal_f64 = vec![
            0b0_00000000000_0000000000000000000000000000000000000000000000000001u64,
            0b1_00000000000_0000000000000000000000000000000000000000000000000001,
            0b0_00000000000_0011001101000111010101010000110000001100110100111111,
            0b1_00000000000_0100000000000000000000000000000000000000000000011111,
            0b0_00000000000_0100000000000000000000000000000000000000000000011111,
        ];

        for n64 in sub_normal_f64.into_iter() {
            let nf64 = unsafe { *(&n64 as *const u64 as *const f64) };
            let rat64 = Ratio::from_ieee754_f64(nf64).unwrap();
            let nf64_ = rat64.to_ieee754_f64().unwrap();
            let rat64_ = Ratio::from_ieee754_f64(nf64_).unwrap();

            assert_eq!(nf64, nf64_);
            assert_eq!(rat64, rat64_);
        }

        // underflown numbers are converted to 0
        assert_eq!(Ratio::from_string("1e-3000").unwrap().to_ieee754_f64().unwrap(), 0.0);
        assert_eq!(Ratio::from_string("1e-3000").unwrap().to_ieee754_f32().unwrap(), 0.0);
    }

    #[test]
    fn ieee754_fuzzing() {

        #[cfg(feature = "rand")] {
            let iter_count = if RUN_ALL_TESTS {
                480
            } else {
                12
            };

            for _ in 0..iter_count {
                let n32 = rand::random::<u32>();
                let nf32 = unsafe { *(&n32 as *const u32 as *const f32) };
                let rat32 = Ratio::from_ieee754_f32(nf32);

                if nf32.is_nan() {
                    assert_eq!(rat32, Err(ConversionError::NotANumber), "{n32}");
                }

                else if nf32.is_infinite() {
                    assert_eq!(rat32, Err(ConversionError::NotANumber), "{n32}");
                }

                else {
                    let nf32_ = match rat32 {
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
                let rat64 = Ratio::from_ieee754_f64(nf64);

                if nf64.is_nan() {
                    assert_eq!(rat64, Err(ConversionError::NotANumber), "{n64}");
                }

                else if nf64.is_infinite() {
                    assert_eq!(rat64, Err(ConversionError::NotANumber), "{n64}");
                }

                else {
                    let nf64_ = match rat64 {
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
        assert_eq!((false, 1, 0), inspect_ieee754_f32(2.0).unwrap());
        assert_eq!((true, 1, 0), inspect_ieee754_f32(-2.0).unwrap());
        assert_eq!((false, 0, (1 << 22)), inspect_ieee754_f32(1.5).unwrap());

        // (1 + 1 / 16 + 1 / 32) * 16
        assert_eq!((false, 4, (1 << 19) + (1 << 18)), inspect_ieee754_f32(17.5).unwrap());

        // (1 + 1 / 2 + 1 / 4 + 1 / 8 + 1 / 16 + 1 / 32 + 1 / 128) * 32
        assert_eq!((false, 5, (1 << 51) + (1 << 50) + (1 << 49) + (1 << 48) + (1 << 47) + (1 << 45)), inspect_ieee754_f64(63.25).unwrap());

        // (1 + 1 / 512) * 1
        assert_eq!((false, 0, 1 << 43), inspect_ieee754_f64(1.001953125).unwrap());

        assert_eq!((false, i32::MIN, 0), inspect_ieee754_f32(0.0).unwrap());
        assert_eq!((false, i32::MIN, 0), inspect_ieee754_f32(-0.0).unwrap());
        assert_eq!((false, i32::MIN, 0), inspect_ieee754_f64(0.0).unwrap());
        assert_eq!((false, i32::MIN, 0), inspect_ieee754_f64(-0.0).unwrap());
    }

    // This test is not just for hmath::Ratio, but also for ieee754 implementation.
    // A failed assertion is either due to (1) a bug in hmath or (2) my wrong assumption on ieee754 implementation.
    #[test]
    fn ieee754_general_test() {
        if !RUN_ALL_TESTS { return; }
        let mut some_ints = vec![
            0, 1, 2, 3i32,
            4, 8, 16, 32, 64, 128,
            9, 27, 81, 243, 729,
            230716, 220331,
        ];
        let mut some_floats = vec![
            0.0, 1.0, 2.0, 3.0f32,
            0.5, 1.5, 2.5, 3.5,
            0.1, 1.1, 2.1, 3.1,
            3.1415, 3.1622, 2.7181,
            0.0078125, 0.0001234,
            1414252536365959.0,
            1234567890.1,
        ];
        some_ints = append_negs(some_ints);
        some_floats = append_negs(some_floats);

        // Assumption 1: int -> float is lossless (also roundtrip)
        for i in some_ints.clone().into_iter() {
            let a = Ratio::from(i);
            let b = Ratio::from_ieee754_f32(i as f32).unwrap();
            let c = Ratio::from_ieee754_f64(i as f64).unwrap();

            assert_eq!(a, b);
            assert_eq!(b, c);

            let d = b.to_ieee754_f32().unwrap();
            let e = c.to_ieee754_f32().unwrap();

            assert_eq!(i, d as i32);
            assert_eq!(i, e as i32);
        }

        // Assumption 2: dividing/multiplying power of 2 is lossless
        let mut pow2 = 1;
        let mut pow2f = 1.0f32;

        for _ in 0..24 {
            for fl in some_floats.clone().into_iter() {
                let a = Ratio::from_ieee754_f32(fl).unwrap().mul_i32(pow2);
                let b = Ratio::from_ieee754_f64(fl as f64).unwrap().mul_i32(pow2);
                let c = Ratio::from_ieee754_f32(fl * pow2f).unwrap();
                let d = Ratio::from_ieee754_f64(fl as f64 * pow2f as f64).unwrap();
                let e = Ratio::from_ieee754_f32(fl * pow2 as f32).unwrap();
                let f = Ratio::from_ieee754_f64(fl as f64 * pow2 as f64).unwrap();

                assert_eq!(a, b);
                assert_eq!(b, c);
                assert_eq!(c, d);
                assert_eq!(d, e);
                assert_eq!(e, f);

                let a = Ratio::from_ieee754_f32(fl).unwrap().div_i32(pow2);
                let b = Ratio::from_ieee754_f64(fl as f64).unwrap().div_i32(pow2);
                let c = Ratio::from_ieee754_f32(fl / pow2f).unwrap();
                let d = Ratio::from_ieee754_f64(fl as f64 / pow2f as f64).unwrap();
                let e = Ratio::from_ieee754_f32(fl / pow2 as f32).unwrap();
                let f = Ratio::from_ieee754_f64(fl as f64 / pow2 as f64).unwrap();

                assert_eq!(a, b);
                assert_eq!(b, c);
                assert_eq!(c, d);
                assert_eq!(d, e);
                assert_eq!(e, f);
            }

            pow2 *= 2;
            pow2f *= 2.0;
        }

        // Assumption 3: f32 -> f64 -> f32 roundtrip is lossless
        for f in some_floats.clone().into_iter() {
            let a = Ratio::from_ieee754_f32(f).unwrap();
            let b = Ratio::from_ieee754_f64(f as f64).unwrap();
            let c = Ratio::from_ieee754_f32(
                b.to_ieee754_f64().unwrap() as f32
            ).unwrap();

            assert_eq!(a, b);
            assert_eq!(b, c);
        }

        // Assumption 4: rounding rules don't make problems in most cases
        for n in 1..40 {
            let a = Ratio::from_denom_and_numer_i32(n, 1).to_ieee754_f32().unwrap();
            let b = Ratio::from_denom_and_numer_i32(n, 1).to_ieee754_f64().unwrap();

            assert_eq!(Ratio::from_ieee754_f32(a * n as f32).unwrap(), Ratio::one());
            assert_eq!(Ratio::from_ieee754_f64(b * n as f64).unwrap(), Ratio::one());
        }
    }

    use std::ops::Neg;

    fn append_negs<T: Neg + Clone>(v: Vec<T>) -> Vec<T> where Vec<T>: FromIterator<<T as Neg>::Output> {
        vec![
            v.clone(),
            v.into_iter().map(|n| -n).collect(),
        ].concat()
    }
}
