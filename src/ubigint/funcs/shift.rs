use crate::UBigInt;

impl UBigInt {

    /// divide by 2^32
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn shift_right(&self, n: usize) -> Self {
        UBigInt::from_raw(self.0[n..].to_vec())
    }

    /// divide by 2^32
    pub fn shift_right_mut(&mut self, n: usize) {
        self.0 = self.0[n..].to_vec();
    }

    /// multiply 2^32
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn shift_left(&self, n: usize) -> Self {
        UBigInt::from_raw(vec![
            vec![0; n],
            self.0.clone()
        ].concat())
    }

    /// multiply 2^32
    pub fn shift_left_mut(&mut self, n: usize) {
        self.0 = vec![
            vec![0; n],
            self.0.clone()
        ].concat();
    }

    /// modulo 2^32
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn slice_right(&self, n: usize) -> Self {
        UBigInt::from_raw(self.0[0..n].to_vec())
    }

    /// modulo 2^32
    pub fn slice_right_mut(&mut self, n: usize) {
        self.0 = self.0[0..n].to_vec();
    }

}