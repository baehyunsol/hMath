use crate::F192;
use crate::fp192::{EXP_COEFF, SIGN_MASK};

impl F192 {
    pub const ZERO: F192 = F192 {
        digits: 0,
        exp: 0,
    };

    pub const ONE: F192 = F192 {
        digits: 0,
        exp: EXP_COEFF - 127,
    };

    pub const MAX: F192 = F192 {
        digits: u128::MAX & !SIGN_MASK,
        exp: u64::MAX,
    };

    pub const MIN: F192 = F192 {
        digits: u128::MAX,
        exp: u64::MAX,
    };

    pub const MIN_POSITIVE: F192 = F192 {
        digits: 1,
        exp: 0,
    };

    pub const PI: F192 = F192 {
        digits: 97115962555772455233232789574583590097,
        exp: 281474976710530,
    };

    pub const PI_OVER_2: F192 = F192 {
        digits: 97115962555772455233232789574583590097,
        exp: 281474976710529,
    };

    pub const PI_OVER_3: F192 = F192 {
        digits: 8030247217025226244926091811094358155,
        exp: 281474976710529,
    };

    pub const PI_OVER_4: F192 = F192 {
        digits: 97115962555772455233232789574583590097,
        exp: 281474976710528,
    };

    pub const PI_OVER_6: F192 = F192 {
        digits: 8030247217025226244926091811094358155,
        exp: 281474976710528,
    };

    pub const E: F192 = F192 {
        digits: 61104660176085852556040455244950093042,
        exp: 281474976710530,
    };

    pub const SQRT_2: F192 = F192 {
        digits: 70474785707535279813346468761740951199,
        exp: 281474976710529,
    };

    pub const SQRT_3: F192 = F192 {
        digits: 124551990752961009652400151969882971589,
        exp: 281474976710529,
    };

    pub const SQRT_10: F192 = F192 {
        digits: 98875648305356569987839135600239050579,
        exp: 281474976710530,
    };

    pub const ONE_OVER_SQRT_2: F192 = F192 {
        digits: 70474785707535279813346468761740951199,
        exp: 281474976710528,
    };

    pub const LN_2: F192 = F192 {
        digits: 65724579765044062406256839048270378671,
        exp: 281474976710528,
    };

    pub const LN_3: F192 = F192 {
        digits: 16778011497737602070059788614351306331,
        exp: 281474976710529,
    };

    pub const LN_10: F192 = F192 {
        digits: 25741092909751534969647316959977736745,
        exp: 281474976710530,
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::fp192::testbench::assert_very_close;

    #[test]
    fn const_tests() {
        assert_eq!(F192::ZERO, 0.into());
        assert_eq!(F192::ONE, 1.into());

        let pi = pi_iter(15);
        let pi: F192 = pi.into();
        assert_eq!(pi, F192::PI);

        assert_very_close(
            "3.1415926535897932384626433832795028841971693993".parse::<F192>().unwrap(),
            F192::PI,
        );

        let pi_over_2 = pi_iter(15).div_i32(2);
        let pi_over_2: F192 = pi_over_2.into();
        assert_eq!(pi_over_2, F192::PI_OVER_2);

        let pi_over_3 = pi_iter(15).div_i32(3);
        let pi_over_3: F192 = pi_over_3.into();
        assert_eq!(pi_over_3, F192::PI_OVER_3);

        let pi_over_4 = pi_iter(15).div_i32(4);
        let pi_over_4: F192 = pi_over_4.into();
        assert_eq!(pi_over_4, F192::PI_OVER_4);

        let pi_over_6 = pi_iter(15).div_i32(6);
        let pi_over_6: F192 = pi_over_6.into();
        assert_eq!(pi_over_6, F192::PI_OVER_6);

        let e = e_iter(16);
        let e: F192 = e.into();
        assert_eq!(e, F192::E);

        assert_very_close(
            "2.718281828459045235360287471352662497757247093".parse::<F192>().unwrap(),
            F192::E,
        );

        let sqrt_2 = sqrt_iter(&2.into(), 17);
        let sqrt_2: F192 = sqrt_2.into();
        assert_eq!(sqrt_2, F192::SQRT_2);

        assert_very_close(
            "1.41421356237309504880168872420969807856967187537".parse::<F192>().unwrap(),
            F192::SQRT_2,
        );

        let sqrt_3 = sqrt_iter(&3.into(), 17);
        let sqrt_3: F192 = sqrt_3.into();
        assert_eq!(sqrt_3, F192::SQRT_3);

        let sqrt_10 = sqrt_iter(&10.into(), 17);
        let sqrt_10: F192 = sqrt_10.into();
        assert_eq!(sqrt_10, F192::SQRT_10);

        assert_very_close(
            "3.16227766016837933199889354443271853371955513".parse::<F192>().unwrap(),
            F192::SQRT_10,
        );

        let one_over_sqrt_2 = Ratio::one().div_rat(&sqrt_iter(&2.into(), 17));
        let one_over_sqrt_2: F192 = one_over_sqrt_2.into();
        assert_eq!(one_over_sqrt_2, F192::ONE_OVER_SQRT_2);

        let ln_2 = ln2_iter(21);
        let ln_2: F192 = ln_2.into();
        assert_eq!(ln_2, F192::LN_2);

        let ln_3 = ln_iter(&3.into(), 62);
        let ln_3: F192 = ln_3.into();
        assert_eq!(ln_3, F192::LN_3);

        let ln_10 = ln_iter(&10.into(), 33);
        let ln_10: F192 = ln_10.into();
        assert_eq!(ln_10, F192::LN_10);
    }
}
