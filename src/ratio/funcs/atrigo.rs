use crate::{Ratio, pi_iter};

// TODO: when x ~= 1

pub fn asin_iter(x: &Ratio, iter: usize) -> Ratio {
    let mut result = x.clone();

    if result.gt_one() {
        panic!("Math Domain Error: asin({x})");
    }

    result.neg_mut();

    if result.gt_one() {
        result.neg_mut();

        panic!("Math Domain Error: asin({x})");
    }

    result.neg_mut();

    let mut curr_coeff = Ratio::from_denom_and_numer_i32(6, 1);
    let x_sqr = x.mul(x);
    let mut curr_x_coeff = x_sqr.mul(x);

    for i in 0..iter {
        result.add_mut(&curr_coeff.mul(&curr_x_coeff));

        curr_coeff.mul_i32_mut(((2 * i + 3) * (2 * i + 3) * (2 * i + 4)) as i32);
        curr_coeff.div_i32_mut(((i + 2) * (i + 2) * (2 * i + 5) * 4) as i32);
        curr_x_coeff.mul_mut(&x_sqr);
    }

    result.add_mut(&curr_coeff.mul(&curr_x_coeff));

    result
}

pub fn acos_iter(x: &Ratio, iter: usize) -> Ratio {
    pi_iter(iter).div_i32(2).sub(&asin_iter(x, iter))
}

// x - x^3/3 + x^5/5 - x^7/7 + ...
pub fn atan_iter(x: &Ratio, iter: usize) -> Ratio {
    let mut result = x.clone();
    let mut is_neg = false;
    let mut is_reci = false;

    if result.is_neg() {
        result.neg_mut();
        is_neg = true;
    }

    if result.gt_one() {
        result.reci_mut();
        is_reci = true;
    }

    let x_sqr = result.mul(&result);
    let mut curr_x_coeff = x_sqr.mul(&result);
    let mut curr_coeff = 3;

    for _ in 0..iter {
        result.sub_mut(&curr_x_coeff.div_i32(curr_coeff));
        curr_coeff += 2;
        curr_x_coeff.mul_mut(&x_sqr);

        result.add_mut(&curr_x_coeff.div_i32(curr_coeff));
        curr_coeff += 2;
        curr_x_coeff.mul_mut(&x_sqr);
    }

    result.sub_mut(&curr_x_coeff.div_i32(curr_coeff));

    if is_reci {
        let pi = pi_iter(iter).div_i32(2);
        result = pi.sub(&result);
    }

    if is_neg {
        result.neg_mut();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ratio;
    use crate::{sin_iter, cos_iter, tan_iter};

    #[test]
    fn atrigo_test() {
        let err1 = Ratio::from_ieee754_f64(1e-2).unwrap();
        let err2 = Ratio::from_ieee754_f64(1e-8).unwrap();

        for n in -64..64 {
            let x1 = Ratio::from_denom_and_numer_i32(64, n);
            let x2 = Ratio::from_denom_and_numer_i32(16, n);

            let mut rad_sin = asin_iter(&x1, 16);
            rad_sin.shrink(4).unwrap();
            let sin = sin_iter(&rad_sin, 12);

            let mut rad_cos = acos_iter(&x1, 16);
            rad_cos.shrink(4).unwrap();
            let cos = cos_iter(&rad_cos, 12);

            let mut rad_tan = atan_iter(&x2, 16);
            rad_tan.shrink(4).unwrap();
            let tan = tan_iter(&rad_tan, 12);

            if n.abs() > 60 {
                assert!(x1.sub(&sin).abs().lt(&err1));
                assert!(x1.sub(&cos).abs().lt(&err1));
            }

            else {
                assert!(x1.sub(&sin).abs().lt(&err2));
                assert!(x1.sub(&cos).abs().lt(&err2));
            }

            if 13 < n.abs() && n.abs() < 17 {
                assert!(x2.sub(&tan).abs().lt(&err1));
            }

            else {
                assert!(x2.sub(&tan).abs().lt(&err2));
            }
        }
    }
}
