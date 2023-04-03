use crate::{Ratio, BigInt};

impl Ratio {

    #[must_use]
    pub fn neg(&self) -> Self {
        // Safety: if a and b are coprime, a and -b are also coprime. property 2 and 3 are satisfied because it doesn't change the sign of denom
        Ratio::from_denom_and_numer_raw(self.denom.clone(), self.numer.neg())
    }

    pub fn neg_mut(&mut self) {
        self.numer.neg_mut();
    }

    #[must_use]
    pub fn abs(&self) -> Self {
        // Safety: if a and b are coprime, a and -b are also coprime. property 2 and 3 are satisfied because it doesn't change the sign of denom
        Ratio::from_denom_and_numer_raw(self.denom.clone(), self.numer.abs())
    }

    pub fn abs_mut(&mut self) {
        self.numer.abs_mut();
    }

    #[must_use]
    pub fn truncate(&self) -> Self {
        Ratio::from_bi(self.truncate_bi())
    }

    pub fn truncate_mut(&mut self) {
        todo!()
    }

    #[must_use]
    pub fn truncate_bi(&self) -> BigInt {
        let result = self.numer.div_bi(&self.denom);

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    #[must_use]
    /// self - truncate(self)
    pub fn frac(&self) -> Self {
        // Safety: (a % b) and b are coprime
        let result = Ratio::from_denom_and_numer_raw(self.denom.clone(), self.numer.rem_bi(&self.denom));

        #[cfg(test)] {
            assert!(result.is_valid());
            assert_eq!(&result.add_rat(&self.truncate()), self);
        }

        result
    }

    /// self -= truncate(self)
    pub fn frac_mut(&mut self) {
        todo!()
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
        }

    }

}