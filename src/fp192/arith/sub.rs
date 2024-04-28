use crate::{F192, Ratio};

impl F192 {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sub(&self, other: &Self) -> Self {
        if self.is_neg() != other.is_neg() {
            if self.is_neg() {  // - +
                return other.add(&self.neg()).neg();
            }

            else {  // + -
                return self.add(&other.neg());
            }
        }

        if self.is_zero() {
            return other.neg();
        }

        else if other.is_zero() {
            return self.clone();
        }

        // TODO
        Ratio::from(self).sub(&other.into()).into()
    }
}
