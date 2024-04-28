use crate::F192;
use crate::fp192::SIGN_MASK;

impl F192 {
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add(&self, other: &Self) -> Self {
        if self.is_neg() != other.is_neg() {
            if self.is_neg() {  // - +
                return other.sub(&self.neg());
            }

            else {  // + -
                return self.sub(&other.neg());
            }
        }

        if self.is_zero() {
            return other.clone();
        }

        else if other.is_zero() {
            return self.clone();
        }

        let digits1 = self.digits | SIGN_MASK;
        let digits2 = other.digits | SIGN_MASK;

        if self.exp > other.exp {
            let diff = self.exp - other.exp;

            if diff > 127 {
                return self.clone();
            }

            let digits2 = digits2 >> diff;

            match digits1.checked_add(digits2) {
                Some(digits) => F192 {
                    digits: (digits & !SIGN_MASK) | ((self.digits >> 127) << 127),
                    exp: self.exp,
                },
                None => {
                    let digits = digits1 / 2 + digits2 / 2 + (digits1 % 2 + digits2 % 2) / 2;

                    F192 {
                        digits: (digits & !SIGN_MASK) | ((self.digits >> 127) << 127),
                        exp: self.exp + 1,
                    }
                }
            }
        }

        else {
            let diff = other.exp - self.exp;

            if diff > 127 {
                return other.clone();
            }

            let digits1 = digits1 >> diff;

            match digits2.checked_add(digits1) {
                Some(digits) => F192 {
                    digits: (digits & !SIGN_MASK) | ((other.digits >> 127) << 127),
                    exp: other.exp,
                },
                None => {
                    let digits = digits2 / 2 + digits1 / 2 + (digits2 % 2 + digits1 % 2) / 2;

                    F192 {
                        digits: (digits & !SIGN_MASK) | ((other.digits >> 127) << 127),
                        exp: other.exp + 1,
                    }
                }
            }
        }
    }
}
