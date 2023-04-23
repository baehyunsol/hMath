use crate::{Ratio, exp_iter, ln_iter};

// It returns `a^b`. It gets more accurate as `iter` gets bigger. If `b` is an integer, try `Ratio::pow_i32` instead.
pub fn pow_iter(a: &Ratio, b: &Ratio, iter: usize) -> Ratio {
    // a^b = e^(b*ln(a))
    exp_iter(&b.mul_rat(&ln_iter(a, iter)), iter)
}

#[cfg(test)]
mod tests {
    use crate::{Ratio, pow_iter};

    #[test]
    fn pow_iter_test() {
        assert_eq!("4617933561212708776.3", pow_iter(&Ratio::from_i32(2), &Ratio::from_denom_and_numer_i32(512, 62 * 512 + 1), 12).to_approx_string(21));
        assert_eq!("3.162277660168", pow_iter(&Ratio::from_i32(10), &Ratio::from_ieee754_f32(0.5).unwrap(), 12).to_approx_string(14));
        assert_eq!("16777215.99999", pow_iter(&Ratio::from_i32(8), &Ratio::from_i32(8), 12).to_approx_string(14));
    }

}