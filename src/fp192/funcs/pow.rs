use crate::F192;
use crate::fp192::{EXP_COEFF, SIGN_MASK};

impl F192 {
    /// 2^n
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn from_pow2(n: i64) -> Self {
        F192 {
            digits: 0,
            exp: (n + EXP_COEFF as i64 - 127) as u64,
        }
    }

    /// 2^self
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn pow2(&self) -> Self {
        // self = int + fraction, where 0 <= fraction < 1
        let int = self.floor();
        let fraction = self.sub(&int);

        let exp = if let Ok(exp) = i64::try_from(int) {
            exp
        }

        else {
            panic!("pow2: too big exp")
        };

        // calc 2^fraction
        // f(0) = 1, f'(0) = ln2, f''(0) = (ln2)^2, f'''(0) = (ln2)^3, ...
        // f(x) = 1 + x ln2 + (1/2) x x ln2 ln2 + (1/6) x x x ln2 ln2 ln2 + ...
        let mut approx = F192::ONE;
        let iter_unit = fraction.mul(&F192::LN_2);
        let mut iter = iter_unit;

        for i in 2..15 {
            approx = approx.add(&iter);
            iter = iter.mul(&iter_unit);
            iter = iter.div_i32(i);
        }

        approx = approx.add(&iter);
        approx.exp = (approx.exp as i64 + exp) as u64;

        approx
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn powi(&self, n: i64) -> Self {
        let n_abs = n.abs() as u64;
        let result = powi_abs(self, n_abs);

        if n < 0 {
            F192::ONE.div(&result)
        }

        else {
            result
        }
    }

    // inverse of `fast_log2`
    pub(crate) fn fast_pow2(n: i64) -> Self {
        let mut e = n >> 24;
        let d = if n < 0 {
            let mut d = n & 0xfff_fff;

            if d < 0 {
                d += 16777216;
                e -= 1;
            }

            d
        } else {
            n & 0xfff_fff
        };

        F192 {
            digits: fast_pow2_worker(d as u32) & !SIGN_MASK,
            exp: (e + EXP_COEFF as i64 - 127) as u64,
        }
    }
}

fn powi_abs(a: &F192, mut b: u64) -> F192 {
    if b < 8 {
        if b < 2 {
            if b == 0 {
                F192::ONE
            }

            else {
                a.clone()
            }
        }

        else {
            let aa = a.mul(a);
            let mut result = F192::ONE;

            while b >= 2 {
                result = result.mul(&aa);
                b -= 2;
            }

            if b == 1 {
                result = result.mul(a);
            }

            result
        }
    }

    else {
        let mut result = powi_abs(a, b / 2);
        result = result.mul(&result);

        if b % 2 == 1 {
            result = result.mul(a);
        }

        result
    }
}

// int(2^(127 + n / 16777216))
fn fast_pow2_worker(n: u32) -> u128 {
    let index = n as usize >> 13;
    let frac = index as u32 & 31;
    let index = index >> 5;

    let res = (POW_TABLE[index] * (32 - frac) + POW_TABLE[index + 1] * frac) >> 5;
    let res = res as u128;

    res << 103
}

const POW_TABLE: [u32; 65] = [
    16777216,  // 2^(24 + 0 / 64)
    16959907,  // 2^(24 + 1 / 64)
    17144589,  // 2^(24 + 2 / 64)
    17331281,  // 2^(24 + 3 / 64)
    17520006,  // 2^(24 + 4 / 64)
    17710787,  // 2^(24 + 5 / 64)
    17903645,  // 2^(24 + 6 / 64)
    18098602,  // 2^(24 + 7 / 64)
    18295683,  // 2^(24 + 8 / 64)
    18494910,  // 2^(24 + 9 / 64)
    18696307,  // 2^(24 + 10 / 64)
    18899896,  // 2^(24 + 11 / 64)
    19105702,  // 2^(24 + 12 / 64)
    19313750,  // 2^(24 + 13 / 64)
    19524063,  // 2^(24 + 14 / 64)
    19736666,  // 2^(24 + 15 / 64)
    19951584,  // 2^(24 + 16 / 64)
    20168843,  // 2^(24 + 17 / 64)
    20388467,  // 2^(24 + 18 / 64)
    20610483,  // 2^(24 + 19 / 64)
    20834916,  // 2^(24 + 20 / 64)
    21061794,  // 2^(24 + 21 / 64)
    21291142,  // 2^(24 + 22 / 64)
    21522987,  // 2^(24 + 23 / 64)
    21757357,  // 2^(24 + 24 / 64)
    21994279,  // 2^(24 + 25 / 64)
    22233781,  // 2^(24 + 26 / 64)
    22475891,  // 2^(24 + 27 / 64)
    22720637,  // 2^(24 + 28 / 64)
    22968049,  // 2^(24 + 29 / 64)
    23218155,  // 2^(24 + 30 / 64)
    23470984,  // 2^(24 + 31 / 64)
    23726566,  // 2^(24 + 32 / 64)
    23984931,  // 2^(24 + 33 / 64)
    24246110,  // 2^(24 + 34 / 64)
    24510133,  // 2^(24 + 35 / 64)
    24777031,  // 2^(24 + 36 / 64)
    25046835,  // 2^(24 + 37 / 64)
    25319577,  // 2^(24 + 38 / 64)
    25595289,  // 2^(24 + 39 / 64)
    25874004,  // 2^(24 + 40 / 64)
    26155753,  // 2^(24 + 41 / 64)
    26440571,  // 2^(24 + 42 / 64)
    26728489,  // 2^(24 + 43 / 64)
    27019544,  // 2^(24 + 44 / 64)
    27313767,  // 2^(24 + 45 / 64)
    27611195,  // 2^(24 + 46 / 64)
    27911861,  // 2^(24 + 47 / 64)
    28215801,  // 2^(24 + 48 / 64)
    28523051,  // 2^(24 + 49 / 64)
    28833647,  // 2^(24 + 50 / 64)
    29147625,  // 2^(24 + 51 / 64)
    29465021,  // 2^(24 + 52 / 64)
    29785874,  // 2^(24 + 53 / 64)
    30110221,  // 2^(24 + 54 / 64)
    30438100,  // 2^(24 + 55 / 64)
    30769549,  // 2^(24 + 56 / 64)
    31104608,  // 2^(24 + 57 / 64)
    31443315,  // 2^(24 + 58 / 64)
    31785710,  // 2^(24 + 59 / 64)
    32131834,  // 2^(24 + 60 / 64)
    32481726,  // 2^(24 + 61 / 64)
    32835429,  // 2^(24 + 62 / 64)
    33192984,  // 2^(24 + 63 / 64)
    33554432,  // 2^(24 + 64 / 64)
];

#[cfg(test)]
mod tests {
    use crate::F192;
    use crate::fp192::testbench::{assert_f64_close, assert_very_close};

    #[test]
    fn fast_pow2_test() {
        for i in 1000..4000i64 {
            let f = F192::fast_pow2(F192::from(i).fast_log2()).try_into().unwrap();
    
            assert!(i.abs_diff(f) < 3);
        }
    
        assert!(F192::fast_pow2(8388608).to_string().starts_with("1.414213"));
    
        let n = F192::fast_pow2(4194304);
        let n = n.mul(&n);
        let n = n.mul(&n);
    
        assert!(n.to_string().starts_with("1.99999"));
    
        assert_eq!("0.5", F192::fast_pow2(-16777216).to_string());
        assert_eq!("0.25", F192::fast_pow2(-33554432).to_string());
        assert_eq!("0.0625", F192::fast_pow2(-67108864).to_string());
    
        assert!(F192::fast_pow2(-8388608).to_string().starts_with("0.707106"));
    }

    #[test]
    fn powi_test() {
        let ten = F192::from(10);

        for i in 0..200 {
            assert_very_close(format!("1e{i}").parse::<F192>().unwrap(), ten.powi(i));
        }
    }

    #[test]
    fn pow2_test() {
        assert_f64_close(
            "0.5".parse::<F192>().unwrap().pow2(),
            F192::SQRT_2,
        );

        assert_f64_close(
            "-0.25".parse::<F192>().unwrap().pow2(),
            F192::ONE.div(&F192::SQRT_2.sqrt()),
        );

        assert_f64_close(
            "1.25".parse::<F192>().unwrap().pow2(),
            F192::SQRT_2.sqrt().mul(&2.into()),
        );

        assert_f64_close(
            F192::ONE.div(&F192::LN_2).pow2(),
            F192::E,
        );

        assert_f64_close(
            F192::from(64).pow2(),
            F192::from(1u128 << 64),
        );

        assert_f64_close(
            F192::try_from(63.5).unwrap().pow2(),
            F192::try_from(13043817825332782212.0f64).unwrap(),
        );
    }
}
