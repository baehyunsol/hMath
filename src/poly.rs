use crate::Ratio;

pub mod from_points;

pub use from_points::{cubic_2_points, quadratic_3_points, linear_2_points};

/// [3, 4, 5] -> 3x^2 + 4x + 5
#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial {
    coeffs: Vec<Ratio>,
}

impl Polynomial {
    pub fn from_vec(mut coeffs: Vec<Ratio>) -> Self {
        let mut i = 0;

        while let Some(n) = coeffs.get(i) {
            if n.is_zero() {
                i += 1;
            } else {
                break;
            }
        }

        // [0, 0, 3, 4] = 0x^3 + 0x^2 + 3x + 4 = 3x + 4 = [3, 4]
        if i != 0 {
            coeffs = coeffs[i..].to_vec();
        }

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
        let k = k.into();

        let result = Polynomial::from_vec(self.coeffs.iter().map(
            |n| n.mul_rat(&k)
        ).collect());

        #[cfg(test)] {
            let mut s = self.clone();
            s.mul_k_mut(k);

            assert_eq!(s, result);
        }

        result
    }

    pub fn mul_k_mut<T: Into<Ratio>>(&mut self, k: T) {
        let k = k.into();

        if k.is_zero() {
            *self = Polynomial::from_vec(vec![]);
            return;
        }

        for n in self.coeffs.iter_mut() {
            n.mul_rat_mut(&k);
        }
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

    pub fn to_string(&self) -> String {
        let result = self.coeffs.iter().rev().enumerate().rev().map(
            |(i, n)| if n.is_zero() {
                String::new()
            } else if i < 2 {
                if i == 0 {
                    n.to_ratio_string()
                } else if n.is_one() {
                    String::from("x")
                } else {
                    format!("{} * x", n.to_ratio_string())
                }
            } else {
                if n.is_one() {
                    format!("x^{i}")
                } else {
                    format!("{} * x^{i}", n.to_ratio_string())
                }
            }
        ).filter(|s| !s.is_empty()).collect::<Vec<String>>().join(" + ");

        if result.is_empty() {
            String::from("0")
        } else {
            result
        }
    }
}
