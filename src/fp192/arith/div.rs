use crate::F192;
use crate::fp192::SIGN_MASK;

impl F192 {

    // TODO: it takes too long
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn div(&self, other: &Self) -> Self {
        if self.is_zero() {
            return F192::ZERO;
        }

        else if other.is_zero() {
            panic!("Attempt to divide by zero");
        }

        let s = self.abs();
        let o = other.abs();
        let approx = self.fast_log2() - other.fast_log2();

        let (mut min, mut max) = (
            F192::fast_pow2(approx - 1048576),
            F192::fast_pow2(approx + 1048576),
        );

        loop {
            let mid = min.add(&max).shr(1);
            let s_ = o.mul(&mid);

            if s_ < s {
                min = mid;
            }

            else {
                max = mid;
            }

            if min.step_epsilon() == max {
                return if self.is_neg() != other.is_neg() {
                    max.neg()
                } else {
                    max
                };
            }
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn div_i32(&self, a: i32) -> Self {
        if a == 0 {
            panic!("Attempt to divide by zero");
        }

        if self.is_zero() {
            return F192::ZERO;
        }

        let mut digits = self.digits | SIGN_MASK;
        let mut exp = self.exp;
        let a_abs = a.abs() as u128;
        let mut rem = digits % a_abs;

        digits /= a_abs;

        while digits < SIGN_MASK {
            digits *= 2;
            rem *= 2;
            exp -= 1;
        }

        digits += rem / a_abs;  // no overflow

        F192 {
            digits: digits & !SIGN_MASK | (((self.is_neg() != (a < 0)) as u128) << 127),
            exp,
        }
    }
}
