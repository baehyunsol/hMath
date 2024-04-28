use crate::F192;
use crate::fp192::{EXP_COEFF, SIGN_MASK};

impl F192 {
    /// It returns `a` where `self` = `d * 2^a` where `1 <= d < 2`.
    /// It panics if `self` is less than or equal to 0.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn ilog2(&self) -> Self {
        if self.is_neg() {
            panic!("logarithm of a negative number is undefined");
        }

        else if self.is_zero() {
            panic!("logarithm of 0 is undefined");
        }

        F192::from(self.exp as i64 - EXP_COEFF as i64 + 127)
    }

    // `self` = `D * 2^E`
    // `log(self)` = `E + log(D)`
    /// It panics if `self` is less than or equal to 0.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2(&self) -> Self {
        if self.is_neg() {
            panic!("logarithm of a negative number is undefined");
        }

        else if self.is_zero() {
            panic!("logarithm of 0 is undefined");
        }

        let digits = self.digits | SIGN_MASK;
        let d = F192 {
            digits,
            exp: EXP_COEFF,
        };
        let e = F192::from(self.exp as i64 - EXP_COEFF as i64);

        e.add(&d.log2_iter(16))
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn ln(&self) -> Self {
        self.log2().mul(&F192::LN_2)
    }

    // it returns log2(self)
    // it's guaranteed that 2^127 <= self < 2^128
    fn log2_iter(&self, iter: usize) -> Self {
        if iter == 0 {
            return 127.into();
        }

        let a = self.mul(self);  // self^2
        let b = a.mul(&a);       // self^4
        let c = b.mul(&b);       // self^8
        let d = c.mul(&c);       // self^16
        let e = d.mul(&d);       // self^32
        let f = e.mul(&e);       // self^64
        let g = f.mul(&f);       // self^128
        let h = g.mul(&g);       // self^256

        let digits = h.digits | SIGN_MASK;
        let d = F192 {
            digits,
            exp: EXP_COEFF,
        };
        let e = F192::from(h.exp as i64 - EXP_COEFF as i64);

        e.add(&d.log2_iter(iter - 1)).shr(8)
    }

    // returns truncate(log2(self) * 16777216)
    pub(crate) fn fast_log2(&self) -> i64 {
        let e = (self.exp as i64 - EXP_COEFF as i64 + 127) << 24;

        let index = ((self.digits & !SIGN_MASK) >> 116) as usize;
        let frac = index as u32 & 31;
        let index = index >> 5;

        // linear interpolation
        e + (((LOG_TABLE[index] * (32 - frac) + LOG_TABLE[index + 1] * frac) as i64) >> 5)
    }
}

// LOG_TABLE[i] = truncate((log2(64 + i) - 6) * 16777216)
const LOG_TABLE: [u32; 65] = [
    0, 375269, 744809, 1108792, 1467382, 1820738, 2169009,
    2512339, 2850868, 3184727, 3514044, 3838940, 4159533,
    4475935, 4788254, 5096595, 5401057, 5701736, 5998727,
    6292117, 6581994, 6868440, 7151535, 7431358, 7707983,
    7981482, 8251925, 8519380, 8783912, 9045583, 9304456,
    9560590, 9814042, 10064867, 10313119, 10558851, 10802114,
    11042956, 11281425, 11517567, 11751428, 11983051, 12212478,
    12439751, 12664910, 12887994, 13109040, 13328086, 13545167,
    13760319, 13973575, 14184969, 14394532, 14602296, 14808293,
    15012550, 15215099, 15415967, 15615181, 15812769, 16008757,
    16203171, 16396036, 16587376, 16777216,
];

#[cfg(test)]
mod tests {
    use crate::F192;
    use crate::fp192::testbench::assert_f64_close;

    #[test]
    fn log2_test() {
        // some of below have very small differences when compared exactly
        assert_eq!(F192::LN_3.div(&F192::LN_2).to_string(), F192::from(3).log2().to_string());
        assert_eq!(F192::LN_10.div(&F192::LN_2).to_string(), F192::from(10).log2().to_string());
        assert_eq!(F192::LN_2.to_string(), F192::from(1).div(&F192::E.log2()).to_string());

        for i in 10..240 {
            let i = i * 7;
            let a = F192::from(i).fast_log2();
            let b = F192::from(i).log2().shl(24).try_into().unwrap();
    
            assert!(
                a.abs_diff(b) < 686
            );
        }
    }

    #[test]
    fn ln_test() {
        assert_f64_close(F192::LN_2, F192::from(2).ln());
        assert_f64_close(F192::LN_3, F192::from(3).ln());
    }
}
