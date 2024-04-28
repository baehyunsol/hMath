use crate::F192;
use crate::fp192::{EXP_COEFF, SIGN_MASK};

impl F192 {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return F192::ZERO;
        }

        let mut exp = match (self.exp as u64 + other.exp as u64).checked_sub(EXP_COEFF as u64) {
            Some(n) => n + 128,
            None => {  // underflow
                return F192::ZERO;
            },
        };
        let digits1 = SIGN_MASK | self.digits;
        let digits2 = SIGN_MASK | other.digits;

        let digits1_hi = digits1 >> 64;
        let digits1_lo = digits1 & 0xffff_ffff_ffff_ffff;
        let digits2_hi = digits2 >> 64;
        let digits2_lo = digits2 & 0xffff_ffff_ffff_ffff;

        let mul_hi = digits1_hi * digits2_hi;
        let mul_lo1 = digits1_hi * digits2_lo / 2;
        let mul_lo2 = digits2_hi * digits1_lo / 2;
        let mul_lo = (mul_lo1 + mul_lo2) >> 63;

        let mut digits = match mul_hi.checked_add(mul_lo) {
            Some(digits) => digits,
            None => {
                exp += 1;
                mul_hi / 2 + mul_lo / 2
            },
        };

        let leading_zeros = digits.leading_zeros();
        digits <<= leading_zeros;
        exp -= leading_zeros as u64;

        digits &= !SIGN_MASK;
        digits |= (self.digits ^ other.digits) & SIGN_MASK;

        F192 {
            digits,
            exp,
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul_i32(&self, a: i32) -> Self {
        if self.is_zero() || a == 0 {
            return F192::ZERO;
        }

        let mut digits = self.digits | SIGN_MASK;
        let mut exp = self.exp;
        let a_abs = a.abs() as u128;
        let mut rem = (digits & 0xffff_ffff) * a_abs;

        digits >>= 32;
        digits *= a_abs;
        exp += 32;

        while digits < SIGN_MASK {
            digits *= 2;
            rem *= 2;
            exp -= 1;
        }

        rem >>= 32;
        digits += rem;

        F192 {
            digits: digits & !SIGN_MASK | (((self.is_neg() != (a < 0)) as u128) << 127),
            exp,
        }
    }
}
