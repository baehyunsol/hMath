use crate::F192;

const SOME_NUMBER: F192 = F192::ONE_OVER_SQRT_2;

impl F192 {
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn asin(&self) -> Self {
        let mut result = self.clone();
        let mut is_neg = false;

        if self.is_neg() {
            is_neg = true;
            result = result.neg();
        }

        if result > F192::ONE {
            panic!("Math Domain Error: asin({self})");
        }

        if self > &SOME_NUMBER {
            result = asin_near_one(result);
        }

        else {
            let x_sqr = result.mul(&result);
            let mut curr_coeff = x_sqr.mul(&result).div_i32(6);

            for i in 0..15 {
                result = result.add(&curr_coeff);
                curr_coeff = curr_coeff.mul(&x_sqr);
                curr_coeff = curr_coeff.mul_i32((2 * i + 3) * (2 * i + 3) * (2 * i + 4));
                curr_coeff = curr_coeff.div_i32((i + 2) * (i + 2) * (2 * i + 5) * 4);
            }
        }

        if is_neg {
            result = result.neg();
        }

        result
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn acos(&self) -> Self {
        F192::PI_OVER_2.sub(&self.asin())
    }

    pub fn atan(&self) -> Self {
        let mut result = self.clone();
        let mut is_neg = false;
        let mut is_reci = false;

        if result.is_neg() {
            is_neg = true;
            result = result.neg();
        }

        if result > F192::ONE {
            is_reci = true;
            result = F192::ONE.div(&result);
        }

        if result > SOME_NUMBER {
            result = atan_near_one(result);
        }

        else {
            let x_sqr = result.mul(&result);
            let mut curr_x_coeff = x_sqr.mul(&result);
            let mut curr_coeff = 3;

            for _ in 0..7 {
                result = result.sub(&curr_x_coeff.div_i32(curr_coeff));
                curr_coeff += 2;
                curr_x_coeff = curr_x_coeff.mul(&x_sqr);

                result = result.add(&curr_x_coeff.div_i32(curr_coeff));
                curr_coeff += 2;
                curr_x_coeff = curr_x_coeff.mul(&x_sqr);
            }

            result = result.sub(&curr_x_coeff.div_i32(curr_coeff));
        }

        if is_reci {
            result = F192::PI_OVER_2.sub(&result);
        }

        if is_neg {
            result = result.neg();
        }

        result
    }
}

fn asin_near_one(x: F192) -> F192 {
    todo!("F192::asin_near_one")
}

fn atan_near_one(x: F192) -> F192 {
    todo!("F192::atan_near_one")
}

#[cfg(test)]
mod tests {
    use crate::F192;
    use crate::fp192::testbench::assert_f64_close;

    #[test]
    fn atrigo_test() {
        for x in -128..128 {
            let x = F192::from(x).shr(7);
            let asin = x.asin();
            let x2 = asin.sin();

            let acos = x.acos();
            let x3 = acos.cos();

            assert_f64_close(x, x2);
            assert_f64_close(x, x3);

            let x = x.shl(3);
            let atan = x.atan();
            let x4 = atan.tan();

            assert_f64_close(x, x4);
        }
    }
}
