use crate::UBigInt;

impl UBigInt {

    /// divide by 2^32
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn shift_right(&self, n: usize) -> Self {
        if n >= self.len() {
            UBigInt::zero()
        }

        else {
            UBigInt::from_raw(self.0[n..].to_vec())
        }
    }

    /// divide by 2^32
    pub fn shift_right_mut(&mut self, n: usize) {
        if n >= self.len() {
            self.0 = vec![0];
        }

        else {
            self.0 = self.0[n..].to_vec();
        }
    }

    /// multiply 2^32
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn shift_left(&self, n: usize) -> Self {
        if self.is_zero() {
            UBigInt::zero()
        }

        else {
            UBigInt::from_raw(vec![
                vec![0; n],
                self.0.clone()
            ].concat())
        }
    }

    /// multiply 2^32
    pub fn shift_left_mut(&mut self, n: usize) {
        if !self.is_zero() {
            self.0 = vec![
                vec![0; n],
                self.0.clone()
            ].concat();
        }
    }

    /// modulo 2^32\
    /// It panics when `n` is too big. It returns 0 when `n` is 0.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn slice_right(&self, n: usize) -> Self {
        if n == 0 {
            UBigInt::zero()
        }

        else {
            let mut result = self.0[0..n].to_vec();

            while result.len() > 1 && result[result.len() - 1] == 0 {
                result.pop().unwrap();
            }

            UBigInt::from_raw(result)
        }
    }

    /// modulo 2^32\
    /// It panics when `n` is too big. It makes `self` 0 when `n` is 0.
    pub fn slice_right_mut(&mut self, n: usize) {
        if n == 0 {
            *self = UBigInt::zero();
        }

        else {
            let mut result = self.0[0..n].to_vec();

            while result.len() > 1 && result[result.len() - 1] == 0 {
                result.pop().unwrap();
            }

            self.0 = result;
        }
    }
}
