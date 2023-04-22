use crate::{Ratio, e_iter};

/// It returns `e^x`. It gets more accurate as `iter` gets bigger.\
/// TODO: It doesn't work if `x` is greater than `2^31`.
pub fn exp_iter(x: &Ratio, iter: usize) -> Ratio {
    let (trun, mut frac) = x.truncate_and_frac();

    // e^x = sigma{n=0}{inf} x^n / n!
    let mut result = Ratio::one();
    let mut iterator = frac.clone();
    let mut n = 2;
    let mut inverse_iter = false;
    let e_approx = e_iter(iter);

    // e^0.99 = e/e^0.01 -> converges much faster
    if iterator.geq_rat(&Ratio::from_denom_and_numer_i32(10, 9)) {
        iterator = Ratio::one().sub_rat(&iterator);
        frac = iterator.clone();
        inverse_iter = true;
    }

    for _ in 0..iter {
        result.add_rat_mut(&iterator);
        iterator.mul_rat_mut(&frac);
        iterator.div_i32_mut(n);
        n += 1;
    }

    if inverse_iter {
        result = e_approx.div_rat(&result);
    }

    result.mul_rat_mut(&e_approx.pow_i32(trun.to_i32().unwrap()));

    result
}

#[cfg(test)]
mod tests {
    use crate::{Ratio, exp_iter, ln2_iter};

    #[test]
    fn exp_test() {
        assert_eq!("2.459603", exp_iter(&Ratio::from_string("0.9").unwrap(), 12).to_approx_string(8));
        assert_eq!("1.99999999999", exp_iter(&ln2_iter(12), 12).to_approx_string(13));
        assert_eq!("0.5", exp_iter(&ln2_iter(12).neg(), 12).to_approx_string(13));
        assert_eq!("2.718281", exp_iter(&Ratio::from_i32(1), 10).to_approx_string(8));
        assert_eq!("0.367879", exp_iter(&Ratio::from_i32(-1), 9).to_approx_string(8));
        assert_eq!("7.389056", exp_iter(&Ratio::from_i32(2), 14).to_approx_string(8));
        assert_eq!("20.08553", exp_iter(&Ratio::from_i32(3), 15).to_approx_string(8));
        assert_eq!("10", exp_iter(&ln2_iter(8).mul_i32(70777).div_i32(21306), 8).to_approx_string(10));
    }

}