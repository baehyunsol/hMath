use crate::Ratio;

mod from_points;

pub use from_points::{from_points, from_points_generic, cubic_2_points, quadratic_3_points, linear_2_points};

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

    pub fn to_vec(&self) -> &Vec<Ratio> {
        &self.coeffs
    }

    /// It panics if a coefficient cannot be converted to an f32. (Eg. not in range)
    pub fn to_vec_f32(&self) -> Vec<f32> {
        self.coeffs.iter().map(|n| n.to_ieee754_f32().unwrap()).collect()
    }

    /// It panics if a coefficient cannot be converted to an f64. (Eg. not in range)
    pub fn to_vec_f64(&self) -> Vec<f64> {
        self.coeffs.iter().map(|n| n.to_ieee754_f64().unwrap()).collect()
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
            |n| n.mul(&k)
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
            n.mul_mut(&k);
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
        let mut result = Vec::with_capacity(self.coeffs.len());

        for (ind, value) in self.coeffs.iter().rev().enumerate().rev() {
            result.push(value.mul_i32(ind as i32));
        }

        result.pop().unwrap();

        if result.is_empty() {
            result.push(Ratio::zero());
        }

        let result = Polynomial::from_vec(result);

        #[cfg(test)] {
            let mut p = self.clone();
            p.differentiate_mut();

            assert_eq!(result, p);
        }

        result
    }

    pub fn differentiate_mut(&mut self) {
        for (ind, value) in self.coeffs.iter_mut().rev().enumerate() {
            value.mul_i32_mut(ind as i32);
        }

        self.coeffs.pop().unwrap();

        if self.coeffs.is_empty() {
            self.coeffs.push(Ratio::zero());
        }
    }

    /// f(x)
    pub fn calc(&self, x: &Ratio) -> Ratio {
        let mut result = Ratio::zero();

        for coeff in self.coeffs.iter() {
            result.mul_mut(x);
            result.add_mut(coeff);
        }

        result
    }

    /// It returns `x - (f(x)/f'(x))`. If you pre-calculated `f'`, pass it to `prime`. Set `prime` to `None` otherwise.
    pub fn newton_method(&self, x: &Ratio, prime: &Option<Polynomial>) -> Ratio {
        let fpx = match prime {
            Some(p) => p.calc(x),
            None => {
                let fp = self.differentiate();
                fp.calc(x)
            },
        };

        x.sub(&self.calc(x).div(&fpx))
    }

    pub fn to_approx_string(&self, max_len: usize) -> String {
        self.to_string(Some(max_len))
    }

    pub fn to_ratio_string(&self) -> String {
        self.to_string(None)
    }

    fn to_string(&self, approx: Option<usize>) -> String {
        let mut result = Vec::with_capacity(self.coeffs.len());

        for (ind, value) in self.coeffs.iter().rev().enumerate().rev() {
            if value.is_zero() {
                continue;
            }

            let abs_val = value.abs();

            if !result.is_empty() {
                if value.is_neg() {
                    result.push(" - ".to_string());
                } else {
                    result.push(" + ".to_string());
                }
            } else if value.is_neg() {
                result.push("-".to_string());
            }

            if !abs_val.is_one() {
                if let Some(n) = approx {
                    result.push(abs_val.to_approx_string(n));
                } else {
                    result.push(abs_val.to_ratio_string());
                }

                if ind > 0 {
                    result.push(" * ".to_string());
                }
            } else if ind == 0 {
                if let Some(n) = approx {
                    result.push(abs_val.to_approx_string(n));
                } else {
                    result.push(abs_val.to_ratio_string());
                }
            }

            if ind != 0 {
                result.push("x".to_string());
            }

            if ind > 1 {
                result.push(format!("^{ind}"));
            }
        }

        if result.is_empty() {
            result.push("0".to_string());
        }

        result.concat()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Polynomial, Ratio};

    #[test]
    fn newtons_method() {
        let f = Polynomial::from_vec_generic(vec![1, 0, -10]);
        let fp = Some(f.differentiate());
        let mut n = Ratio::from_i32(3);

        for _ in 0..6 {
            n = f.newton_method(&n, &fp);
        }

        assert_eq!("3.162277660168379331998893544432", n.to_approx_string(32));
    }

    #[test]
    fn diff_test() {
        // 3x^3 + 4x^2 + 5x + 6
        let p1 = Polynomial::from_vec_generic(vec![3, 4, 5, 6]);

        // 9x^2 + 8x + 5
        let p2 = Polynomial::from_vec_generic(vec![9, 8, 5]);

        // 18x + 8
        let p3 = Polynomial::from_vec_generic(vec![18, 8]);

        // 18
        let p4 = Polynomial::from_vec_generic(vec![18]);

        // 0
        let p5 = Polynomial::from_vec_generic(vec![0]);

        assert_eq!(p1.differentiate(), p2);
        assert_eq!(p2.differentiate(), p3);
        assert_eq!(p3.differentiate(), p4);
        assert_eq!(p4.differentiate(), p5);
        assert_eq!(p5.differentiate(), p5);
    }

    #[test]
    fn to_string_test() {
        let v = vec![3.5, 4.25, 5.0, 6.5, 0.0, 1.0, 2.0, -3.0, -4.0];
        let v = v.into_iter().map(|n| Ratio::from_ieee754_f32(n).unwrap()).collect();
    
        assert_eq!("7/2 * x^8 + 17/4 * x^7 + 5 * x^6 + 13/2 * x^5 + x^3 + 2 * x^2 - 3 * x - 4", Polynomial::from_vec(v).to_ratio_string());
        assert_eq!("0", Polynomial::from_vec_generic(vec![0]).to_ratio_string());
        assert_eq!("1", Polynomial::from_vec_generic(vec![1]).to_ratio_string());
        assert_eq!("-1", Polynomial::from_vec_generic(vec![-1]).to_ratio_string());
        assert_eq!("0", Polynomial::from_vec_generic(vec![0; 3]).to_ratio_string());
        assert_eq!("x^2 + x + 1", Polynomial::from_vec_generic(vec![1; 3]).to_ratio_string());
        assert_eq!("-x^2 - x - 1", Polynomial::from_vec_generic(vec![-1; 3]).to_ratio_string());
        assert_eq!("x^3 + x", Polynomial::from_vec_generic(vec![1, 0, 1, 0]).to_ratio_string());
        assert_eq!("x^2 + 1", Polynomial::from_vec_generic(vec![0, 1, 0, 1]).to_ratio_string());
        assert_eq!("x^3 + x", Polynomial::from_vec_generic(vec![0, 1, 0, 1, 0]).to_ratio_string());
        assert_eq!("x^4 + x^2 + 1", Polynomial::from_vec_generic(vec![1, 0, 1, 0, 1]).to_ratio_string());
        assert_eq!("-x^3 - x", Polynomial::from_vec_generic(vec![-1, 0, -1, 0]).to_ratio_string());
        assert_eq!("-x^2 - 1", Polynomial::from_vec_generic(vec![0, -1, 0, -1]).to_ratio_string());
        assert_eq!("-x^3 - x", Polynomial::from_vec_generic(vec![0, -1, 0, -1, 0]).to_ratio_string());
        assert_eq!("-x^4 - x^2 - 1", Polynomial::from_vec_generic(vec![-1, 0, -1, 0, -1]).to_ratio_string());
    }
}
