use crate::Ratio;

impl Ratio {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn pow_i32(&self, exp: i32) -> Self {

        // Safety: if a and b are coprime, a^n and b^n are coprime.
        let mut result = Ratio::from_denom_and_numer_raw(
            self.denom.pow_u32(exp.abs() as u32),
            self.numer.pow_u32(exp.abs() as u32),
        );

        if exp < 0 {
            result.reci_mut();
        }

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn pow_i32_mut(&mut self, exp: i32) {
        self.denom.pow_u32_mut(exp.abs() as u32);
        self.numer.pow_u32_mut(exp.abs() as u32);

        if exp < 0 {
            self.reci_mut();
        }

        #[cfg(test)] assert!(self.is_valid());
    }
}
