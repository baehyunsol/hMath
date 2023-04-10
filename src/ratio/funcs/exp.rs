use crate::{Ratio, e_iter};

/// It returns `e^x`. It gets more accurate as `iter` gets bigger.\
pub fn exp_iter(x: &Ratio, iter: usize) -> Ratio {

    let (trun, frac) = x.truncate_and_frac();

    // e^x = sigma{n=0}{inf} x^n / n!
    let mut result = Ratio::one();
    let mut iterator = frac.clone();
    let mut n = 2;

    for _ in 0..iter {
        result.add_rat_mut(&iterator);
        iterator.mul_rat_mut(&frac);
        iterator.div_i32_mut(n);
        n += 1;
    }

    // TODO: what if `trun` is greater than 2^31?
    result.mul_rat_mut(&e_iter(iter).pow_i32(trun.to_i32().unwrap()));

    result
}

#[cfg(test)]
mod tests {
    use crate::{Ratio, exp_iter, ln2_iter};

    #[test]
    fn exp_test() {
        assert_eq!("1.99999999999", exp_iter(&ln2_iter(12), 12).to_approx_string(13));
        assert_eq!("2.718281", exp_iter(&Ratio::from_i32(1), 10).to_approx_string(8));
        assert_eq!("7.389056", exp_iter(&Ratio::from_i32(2), 14).to_approx_string(8));
        assert_eq!("20.08553", exp_iter(&Ratio::from_i32(3), 15).to_approx_string(8));
        assert_eq!("10", exp_iter(&ln2_iter(8).mul_i32(70777).div_i32(21306), 8).to_approx_string(10));
    }

}