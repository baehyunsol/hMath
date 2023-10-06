use crate::{BigInt, Ratio, pi_iter};

/// It returns `sin(x)`. It gets more accurate as `iter` gets bigger.
pub fn sin_iter(x: &Ratio, iter: usize) -> Ratio {

    // -inf ~ 0 -> sin(x) = -sin(-x)
    // 0 ~ pi/4 -> good to go
    // pi/4 ~ pi/2 -> sin(x) = cos(pi/2 - x)
    // pi/2 ~ pi -> sin(x) = sin(pi - x)
    // pi ~ 2pi -> sin(x) = -sin(x - pi)
    // 2pi ~ inf -> sin(x) = sin(x - 2pi)
    let pi = pi_iter(iter);
    let mut pi_div = x.div_rat(&pi);
    let mut negate = false;

    if pi_div.is_neg() {
        negate = !negate;
        pi_div.neg_mut();
    }

    if pi_div.gt_i32(2) {
        pi_div.sub_bi_mut(&pi_div.truncate_bi().div_i32(2).mul_i32(2));
    }

    if pi_div.gt_one() {
        pi_div.sub_i32_mut(1);
        negate = !negate;
    }

    if pi_div.mul_i32(2).gt_one() {
        pi_div = Ratio::one().sub_rat(&pi_div);
    }

    if pi_div.mul_i32(4).lt_one() {
        let mut result = sin_iter_worker(&pi_div.mul_rat(&pi), iter);
        if negate { result.neg_mut(); }
        result
    }

    else {
        let mut result = cos_iter_worker(&Ratio::from_denom_and_numer_i32(2, 1).sub_rat(&pi_div).mul_rat(&pi), iter);
        if negate { result.neg_mut(); }
        result
    }

}

// x - x^3/3! + x^5/5! - x^7/7! + ...
fn sin_iter_worker(x: &Ratio, iter: usize) -> Ratio {
    let mut result = x.clone();
    let mut numer = x.pow_i32(3);
    let x_sq = x.mul_rat(x);
    let mut denom = BigInt::from_i32(6);

    for i in 0..iter {
        result.sub_rat_mut(
            &numer.div_bi(&denom)
        );

        denom.mul_i32_mut((i * 4 + 4) as i32);
        denom.mul_i32_mut((i * 4 + 5) as i32);
        numer.mul_rat_mut(&x_sq);

        result.add_rat_mut(
            &numer.div_bi(&denom)
        );

        denom.mul_i32_mut((i * 4 + 6) as i32);
        denom.mul_i32_mut((i * 4 + 7) as i32);
        numer.mul_rat_mut(&x_sq);
    }

    result.sub_rat_mut(
        &numer.div_bi(&denom)
    );

    result
}

/// It returns `cos(x)`. It gets more accurate as `iter` gets bigger.
pub fn cos_iter(x: &Ratio, iter: usize) -> Ratio {
    // -inf ~ 0 -> cos(x) = cos(-x)
    // 0 ~ pi/4 -> good to go
    // pi/4 ~ pi/2 -> cos(x) = sin(pi/2 - x)
    // pi/2 ~ pi -> cos(x) = -cos(pi - x)
    // pi ~ 2pi -> cos(x) = cos(2pi - x)
    // 2pi ~ inf -> cos(x) = cos(x - 2pi)
    let pi = pi_iter(iter);
    let mut pi_div = x.div_rat(&pi);
    let mut negate = false;

    if pi_div.is_neg() {
        pi_div.neg_mut();
    }

    if pi_div.gt_i32(2) {
        pi_div.sub_bi_mut(&pi_div.truncate_bi().div_i32(2).mul_i32(2));
    }

    if pi_div.gt_one() {
        pi_div = Ratio::from_i32(2).sub_rat(&pi_div);
    }

    if pi_div.mul_i32(2).gt_one() {
        pi_div = Ratio::one().sub_rat(&pi_div);
        negate = !negate;
    }

    if pi_div.mul_i32(4).lt_one() {
        let mut result = cos_iter_worker(&pi_div.mul_rat(&pi), iter);
        if negate { result.neg_mut(); }
        result
    }

    else {
        let mut result = sin_iter_worker(&Ratio::from_denom_and_numer_i32(2, 1).sub_rat(&pi_div).mul_rat(&pi), iter);
        if negate { result.neg_mut(); }
        result
    }
}

// 1 - x^2/2! + x^4/4! - x^6/6! + ...
fn cos_iter_worker(x: &Ratio, iter: usize) -> Ratio {
    let mut result = Ratio::one();
    let x_sq = x.mul_rat(x);
    let mut numer = x_sq.clone();
    let mut denom = BigInt::from_i32(2);

    for i in 0..iter {
        result.sub_rat_mut(
            &numer.div_bi(&denom)
        );

        denom.mul_i32_mut((i * 4 + 3) as i32);
        denom.mul_i32_mut((i * 4 + 4) as i32);
        numer.mul_rat_mut(&x_sq);

        result.add_rat_mut(
            &numer.div_bi(&denom)
        );

        denom.mul_i32_mut((i * 4 + 5) as i32);
        denom.mul_i32_mut((i * 4 + 6) as i32);
        numer.mul_rat_mut(&x_sq);
    }

    result.sub_rat_mut(
        &numer.div_bi(&denom)
    );

    result
}

/// It returns `tan(x)`. It gets more accurate as `iter` gets bigger.
pub fn tan_iter(x: &Ratio, iter: usize) -> Ratio {
    sin_iter(x, iter).div_rat(&cos_iter(x, iter))
}

#[cfg(test)]
mod tests {
    use crate::{Ratio, sqrt_iter, sin_iter, cos_iter, pi_iter};
    use crate::utils::are_close;
    use crate::consts::RUN_ALL_TESTS;

    #[test]
    fn sin_test() {
        assert!(sin_iter(&"314.159265358979323846264338".parse::<Ratio>().unwrap(), 11).lt_rat(&"1e-21".parse::<Ratio>().unwrap()));

        if !RUN_ALL_TESTS { return; }

        let iter = 9;
        let samples = vec![
            // (a, b, c) -> sin(a * pi / b) = c
            (-2, 12, Ratio::from_denom_and_numer_i32(2, 1).neg()),       // sin(-pi/6) = -0.5
            (-1, 12, sqrt_iter(&Ratio::from_i32(6), iter).sub_rat(&sqrt_iter(&Ratio::from_i32(2), iter)).div_i32(4).neg()),  // sin(-pi/12) = -(sqrt(6) - sqrt(2))/4
            ( 0, 12, Ratio::zero()),                                     // sin(0) = 0
            ( 1, 12, sqrt_iter(&Ratio::from_i32(6), iter).sub_rat(&sqrt_iter(&Ratio::from_i32(2), iter)).div_i32(4)),  // sin(pi/12) = (sqrt(6) - sqrt(2))/4
            ( 2, 12, Ratio::from_denom_and_numer_i32(2, 1)),             // sin(pi/6) = 0.5
            ( 3, 12, sqrt_iter(&Ratio::from_i32(2), iter).reci()),       // sin(pi/4) = 1/sqrt(2)
            ( 4, 12, sqrt_iter(&Ratio::from_i32(3), iter).div_i32(2)),   // sin(pi/3) = sqrt(3)/2
            ( 5, 12, sqrt_iter(&Ratio::from_i32(6), iter).add_rat(&sqrt_iter(&Ratio::from_i32(2), iter)).div_i32(4)),  // sin(5*pi/12) = (sqrt(6) + sqrt(2))/4
            ( 6, 12, Ratio::one()),                                      // sin(pi/2) = 1
            ( 7, 12, sqrt_iter(&Ratio::from_i32(6), iter).add_rat(&sqrt_iter(&Ratio::from_i32(2), iter)).div_i32(4)),  // sin(7*pi/12) = (sqrt(6) + sqrt(2))/4
            ( 8, 12, sqrt_iter(&Ratio::from_i32(3), iter).div_i32(2)),   // sin(2*pi/3) = sqrt(3)/2
            ( 9, 12, sqrt_iter(&Ratio::from_i32(2), iter).reci()),       // sin(3*pi/4) = 1/sqrt(2)
            (10, 12, Ratio::from_denom_and_numer_i32(2, 1)),             // sin(5*pi/6) = 0.5
            (11, 12, sqrt_iter(&Ratio::from_i32(6), iter).sub_rat(&sqrt_iter(&Ratio::from_i32(2), iter)).div_i32(4)),  // sin(11*pi/12) = (sqrt(6) - sqrt(2))/4
            (12, 12, Ratio::zero()),                                     // sin(pi) = 0
            (13, 12, sqrt_iter(&Ratio::from_i32(6), iter).sub_rat(&sqrt_iter(&Ratio::from_i32(2), iter)).div_i32(4).neg()),  // sin(13*pi/12) = -(sqrt(6) - sqrt(2))/4
            (14, 12, Ratio::from_denom_and_numer_i32(2, -1)),            // sin(7*pi/6) = -0.5
        ];
        let pi = pi_iter(iter);
        let cos_coeff = Ratio::from_denom_and_numer_i32(-2, 1);
        let accuracy = 3e-7;

        for (numer, denom, value) in samples.into_iter() {
            let sin_val = sin_iter(&Ratio::from_denom_and_numer_i32(denom, numer).mul_rat(&pi), iter);
            let sin_val2 = sin_iter(&Ratio::from_denom_and_numer_i32(denom, numer).add_i32(6).mul_rat(&pi), iter);
            let sin_val3 = sin_iter(&Ratio::from_denom_and_numer_i32(denom, numer).add_i32(-6).mul_rat(&pi), iter);
            let cos_val = cos_iter(&Ratio::from_denom_and_numer_i32(denom, numer).add_rat(&cos_coeff).mul_rat(&pi), iter);
            let cos_val2 = cos_iter(&Ratio::from_denom_and_numer_i32(denom, numer).add_rat(&cos_coeff).add_i32(6).mul_rat(&pi), iter);
            let cos_val3 = cos_iter(&Ratio::from_denom_and_numer_i32(denom, numer).add_rat(&cos_coeff).add_i32(-6).mul_rat(&pi), iter);
            let ans_f64 = (3.14159265358979f64 * numer as f64 / denom as f64).sin();
            let ans_f64 = Ratio::from_ieee754_f64(ans_f64).unwrap();

            assert!(are_close(&ans_f64, &value, accuracy));
            assert!(are_close(&sin_val, &value, accuracy));
            assert!(are_close(&sin_val2, &value, accuracy));
            assert!(are_close(&sin_val3, &value, accuracy));
            assert!(are_close(&cos_val, &value, accuracy));
            assert!(are_close(&cos_val2, &value, accuracy));
            assert!(are_close(&cos_val3, &value, accuracy));
        }

    }

}
