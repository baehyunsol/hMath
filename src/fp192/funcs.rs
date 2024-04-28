use super::{F192, EXP_COEFF};

mod atrigo;
mod ln;
mod pow;
mod sqrt;
mod trigo;

impl F192 {
    pub fn floor(&self) -> Self {
        let t = self.truncate();

        if t == *self {
            t
        }

        else if self.is_neg() {
            t.sub(&1.into())
        }

        else {
            t
        }
    }

    pub fn ceil(&self) -> Self {
        let t = self.truncate();

        if t == *self {
            t
        }

        else if self.is_neg() {
            t
        }

        else {
            t.add(&1.into())
        }
    }

    pub fn round(&self) -> Self {
        let t = self.truncate();

        if t == *self {
            t
        }

        else {
            let f = self.sub(&t);
            let half = F192::ONE.shr(1);
            let neg_half = half.neg();

            if f <= neg_half {
                t.add(&(-1).into())
            }

            else if f >= half {
                t.add(&1.into())
            }

            else {
                t
            }
        }
    }

    pub fn truncate(&self) -> Self {
        if self.is_zero() {
            F192::ZERO
        }

        else {
            let e = self.exp as i64 - EXP_COEFF as i64;

            if e < -127 {
                F192::ZERO
            }

            else if e >= 0 {
                *self
            }

            else {
                let e = -e as u32;

                let digits = (self.digits >> e) << e;

                F192 {
                    digits,
                    exp: self.exp,
                }
            }
        }
    }

    pub fn frac(&self) -> Self {
        self.sub(&self.truncate())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! round_test {
        ($name: ident, $n: expr, $fl: expr, $ce: expr, $ro: expr, $tr: expr, $fr: expr) => {
            #[test]
            fn $name() {
                test_runner(($n).parse::<F192>().unwrap(), $fl, $ce, $ro, $tr, $fr);
            }
        };
    }

    fn test_runner(
        n: F192,
        fl: i64,
        ce: i64,
        ro: i64,
        tr: i64,
        fr: &str,
    ) {
        let n_f64 = f64::from(n);
        let n_f32 = f32::from(n);

        // ieee754 round_trip
        assert_eq!(f64::from(F192::try_from(n_f64).unwrap()), n_f64);
        assert_eq!(f32::from(F192::try_from(n_f32).unwrap()), n_f32);

        // string round_trip
        assert_eq!(n.to_string().parse::<F192>(), Ok(n));

        // code13 round_trip
        assert_eq!(F192::from_code13(&n.into_code13().unwrap()).unwrap().to_string(), n.to_string());

        assert_eq!(n.to_string().parse::<f32>(), Ok(n_f32));
        assert_eq!(n.to_string().parse::<f64>(), Ok(n_f64));
        assert_eq!(n.to_string().parse::<F192>(), Ok(n));

        assert_eq!(n.floor(), F192::try_from(n_f64.floor()).unwrap());
        assert_eq!(n.floor(), F192::from(fl));
        assert_eq!(n.ceil(), F192::try_from(n_f64.ceil()).unwrap());
        assert_eq!(n.ceil(), F192::from(ce));
        assert_eq!(n.round(), F192::try_from(n_f64.round()).unwrap());
        assert_eq!(n.round(), F192::from(ro));
        assert_eq!(n.truncate(), F192::try_from(n_f64.trunc()).unwrap());
        assert_eq!(n.truncate(), F192::from(tr));
        assert_eq!(n.frac().to_string(), fr);
    }

    round_test!(round_test_1, "5.999", 5, 6, 6, 5, "0.999");
    round_test!(round_test_2, "-5.999", -6, -5, -6, -5, "-0.999");
    round_test!(round_test_3, "3.77", 3, 4, 4, 3, "0.77");
    round_test!(round_test_4, "-3.77", -4, -3, -4, -3, "-0.77");
    round_test!(round_test_5, "1.11", 1, 2, 1, 1, "0.11");
    round_test!(round_test_6, "-1.11", -2, -1, -1, -1, "-0.11");
    round_test!(round_test_7, "9.001", 9, 10, 9, 9, "0.001");
    round_test!(round_test_8, "-9.001", -10, -9, -9, -9, "-0.001");
    round_test!(round_test_9, "3.0", 3, 3, 3, 3, "0");
    round_test!(round_test_a, "-3.0", -3, -3, -3, -3, "0");
    round_test!(round_test_b, "0.0", 0, 0, 0, 0, "0");
    round_test!(round_test_c, "-0.0", 0, 0, 0, 0, "0");
    round_test!(round_test_d, "0.0006", 0, 1, 0, 0, "0.0006");
    round_test!(round_test_e, "-0.0006", -1, 0, 0, 0, "-0.0006");
    round_test!(round_test_f, "162931", 162931, 162931, 162931, 162931, "0");
    round_test!(round_test_g, "-162931", -162931, -162931, -162931, -162931, "0");
    round_test!(round_test_h, "10.5", 10, 11, 11, 10, "0.5");
    round_test!(round_test_i, "-10.5", -11, -10, -11, -10, "-0.5");
    round_test!(round_test_j, "11.5", 11, 12, 12, 11, "0.5");
    round_test!(round_test_k, "-11.5", -12, -11, -12, -11, "-0.5");
    round_test!(round_test_l, "12.5", 12, 13, 13, 12, "0.5");
    round_test!(round_test_m, "-12.5", -13, -12, -13, -12, "-0.5");
}
