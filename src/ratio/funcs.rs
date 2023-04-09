use crate::{Ratio, BigInt};

mod exp;

pub use exp::exp_iter;

impl Ratio {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn neg(&self) -> Self {
        // Safety: if a and b are coprime, a and -b are also coprime. property 2 and 3 are satisfied because it doesn't change the sign of denom
        Ratio::from_denom_and_numer_raw(self.denom.clone(), self.numer.neg())
    }

    pub fn neg_mut(&mut self) {
        self.numer.neg_mut();
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn abs(&self) -> Self {
        // Safety: if a and b are coprime, a and -b are also coprime. property 2 and 3 are satisfied because it doesn't change the sign of denom
        Ratio::from_denom_and_numer_raw(self.denom.clone(), self.numer.abs())
    }

    pub fn abs_mut(&mut self) {
        self.numer.abs_mut();
    }

    /// 1 / self
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn reci(&self) -> Self {
        // Safety: `self.denom` and `self.numer` is already coprime
        let mut result = Ratio::from_denom_and_numer_raw(
            self.numer.clone(),
            self.denom.clone()
        );

        if result.denom.is_zero() {
            panic!("Attempt to divide by zero: 1 / {self:?}");
        }

        if result.denom.is_neg() {
            result.denom.neg_mut();
            result.numer.neg_mut();
        }

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    /// self = 1 / self
    pub fn reci_mut(&mut self) {
        *self = self.reci();
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn truncate(&self) -> Self {
        Ratio::from_bi(self.truncate_bi())
    }

    pub fn truncate_mut(&mut self) {
        self.numer.div_bi_mut(&self.denom);
        self.denom = BigInt::one();

        #[cfg(test)] assert!(self.is_valid());
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn truncate_bi(&self) -> BigInt {
        let result = self.numer.div_bi(&self.denom);

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    /// self - truncate(self)
    pub fn frac(&self) -> Self {
        // Safety: (a % b) and b are coprime
        let result = Ratio::from_denom_and_numer_raw(self.denom.clone(), self.numer.rem_bi(&self.denom));

        #[cfg(test)] {
            assert!(result.is_valid());
            assert_eq!(&result.add_rat(&self.truncate()), self);

            let mut self_clone = self.clone();
            self_clone.frac_mut();

            assert_eq!(self_clone, result);
        }

        result
    }

    /// self -= truncate(self)
    pub fn frac_mut(&mut self) {
        self.numer.rem_bi_mut(&self.denom);
    }

    /// It returns a number between 0 and 1 (both exclusive).
    #[cfg(feature = "rand")]
    pub fn random() -> Self {
        let mut result = Ratio::from_raw(
            vec![0, 0, 0, 0, 1],
            false,
            (0..4).map(|_| rand::random::<u32>().max(1)).collect(),
            false
        );
        result.fit();

        #[cfg(test)] {
            assert!(result.is_valid());
            assert!(result.lt_one());
            assert!(!result.is_zero());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Ratio;

    #[test]
    fn frac_trunc_test() {
        let samples = vec![
            ("3.7", "3.0"),
            ("-3.7", "-3.0"),
            ("4.0", "4.0"),
            ("-4.0", "-4.0"),
            ("0.0", "0.0"),
            ("-0.0", "-0.0"),
        ];

        for (before, after) in samples.into_iter() {
            assert_eq!(
                Ratio::from_string(before).unwrap().truncate(),
                Ratio::from_string(after).unwrap()
            );

            // test code is inside the `.frac()` method
            let _ = Ratio::from_string(before).unwrap().frac();
            let _ = Ratio::from_string(after).unwrap().frac();
        }

    }

}