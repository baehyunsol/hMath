use crate::Ratio;

pub mod from_points;

pub use from_points::{cubic_2_points, quadratic_3_points};

/// [3, 4, 5] -> 3x^2 + 4x + 5
pub struct Polynomial {
    coeffs: Vec<Ratio>,
}

impl Polynomial {
    pub fn from_vec(mut coeffs: Vec<Ratio>) -> Self {
        if coeffs.is_empty() { coeffs = vec![Ratio::zero()]; }

        Polynomial { coeffs }
    }

    pub fn from_vec_generic<T: Into<Ratio>>(coeffs: Vec<T>) -> Self {
        if coeffs.is_empty() { return Polynomial::from_vec(vec![]); }

        Polynomial { coeffs: coeffs.into_iter().map(|n| n.into()).collect() }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add(&self, other: &Polynomial) -> Self {
        todo!()
    }

    pub fn add_mut(&mut self, other: &Polynomial) {
        todo!()
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul_k<T: Into<Ratio>>(&self, k: T) -> Self {
        todo!()
    }

    pub fn mul_k_mut<T: Into<Ratio>>(&mut self, k: T) {
        todo!()
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul(&self, other: &Polynomial) -> Self {
        todo!()
    }

    pub fn mul_mut(&mut self, other: &Polynomial) {
        todo!()
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn differentiate(&self) -> Self {
        todo!()
    }

    pub fn differentiate_mut(&mut self) {
        todo!()
    }

    /// f(x)
    pub fn calc(&self, x: &Ratio) -> Ratio {
        let mut result = Ratio::zero();

        for coeff in self.coeffs.iter() {
            result.mul_rat_mut(x);
            result.add_rat_mut(coeff);
        }

        result
    }
}
