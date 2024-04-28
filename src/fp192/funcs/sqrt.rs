use crate::F192;
use crate::fp192::{EXP_COEFF, SIGN_MASK};

impl F192 {
    /// If self is negative, it returns 0.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sqrt(&self) -> Self {
        if self.is_zero() || self.is_neg() {
            return F192::ZERO;
        }

        let digits = self.digits | SIGN_MASK;
        let exp = self.exp as i64 - EXP_COEFF as i64;

        let mut digits_sqrt = isqrt(digits);
        let mut exp_sqrt = exp / 2;

        let exp_odd = exp.abs() % 2 == 1;

        digits_sqrt <<= 64;
        exp_sqrt -= 64;

        let mut approx = F192 {
            digits: digits_sqrt & !SIGN_MASK,
            exp: (exp_sqrt + EXP_COEFF as i64) as u64,
        };

        // if pos, odd -> has to mul sqrt(2)
        // if neg, odd -> has to mul sqrt(2) and shr(1)
        if exp_odd {
            approx = approx.mul(&F192::SQRT_2);

            if exp < 0 {
                approx = approx.shr(1);
            }
        }

        approx = approx.add(&self.div(&approx)).shr(1);
        approx = approx.add(&self.div(&approx)).shr(1);

        approx
    }
}

// 2^127 <= n < 2^128
#[inline]
fn isqrt(n: u128) -> u128 {
    // 2^63.5
    let mut x = 13043817825332782212;

    // it doesn't have to be 100% accurate. 99% is enough
    x = (x + n / x) >> 1;
    x = (x + n / x) >> 1;
    x = (x + n / x) >> 1;
    let x2 = (x + n / x) >> 1;

    x.min(x2)
}

#[cfg(test)]
#[test]
fn sqrt_test() {
    assert!(F192::from(10).sqrt().to_string().starts_with("3.1622776601683793"));
    assert!(F192::from(1000).sqrt().to_string().starts_with("31.622776601683793"));
    assert!(F192::from(100000).sqrt().to_string().starts_with("316.22776601683793"));
    assert!(F192::from(10000000).sqrt().to_string().starts_with("3162.2776601683793"));

    assert!(
        F192::from(1_0000_0000_0000_0000i128).sqrt().sqrt().sqrt().sqrt().sqrt()
            .to_string().starts_with("3.162277660168379")
    );

    for n in 2..32 {
        assert_eq!(F192::from(n), F192::from(n * n).sqrt());
    }

    // some of below have very small differences when compared exactly
    assert_eq!(F192::from(2).sqrt().to_string(), F192::SQRT_2.to_string());
    assert_eq!(F192::from(3).sqrt().to_string(), F192::SQRT_3.to_string());
    assert_eq!(F192::from(10).sqrt().to_string(), F192::SQRT_10.to_string());
}
