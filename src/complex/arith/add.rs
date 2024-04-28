use crate::Complex;

impl Complex {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add(&self, other: &Self) -> Self {
        Complex {
            real_: self.real_.add(&other.real_),
            imag_: self.imag_.add(&other.imag_),
        }
    }

    pub fn add_mut(&mut self, other: &Self) {
        self.real_.add_mut(&other.real_);
        self.imag_.add_mut(&other.imag_);
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add_i32(&self, other: i32) -> Self {
        Complex {
            real_: self.real_.add_i32(other),
            imag_: self.imag_.clone(),
        }
    }

    pub fn add_i32_mut(&mut self, other: i32) {
        self.real_.add_i32_mut(other);
    }
}
