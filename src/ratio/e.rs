use crate::{BigInt, Ratio};

/// It returns an approximate value of E.
/// It gets more and more accurate as `k` gets bigger.
pub fn e_iter(k: usize) -> Ratio {
    let mut numer = BigInt::from_i64(330665665962404000);
    let mut curr_fac = 20;

    for _ in 0..k {
        numer.mul_i32_mut(curr_fac);
        numer.add_i32_mut(1);
        curr_fac += 1;
    }

    Ratio::from_denom_and_numer(BigInt::factorial(curr_fac as u32 - 1), numer)
}

#[cfg(test)]
mod tests {
    use super::e_iter;

    #[test]
    fn e_test() {
        assert_eq!(
            e_iter(1).to_approx_string(21),
            "2.7182818284590452353",
        );
        assert_eq!(
            e_iter(3).to_approx_string(24),
            "2.7182818284590452353602",
        );
        assert_eq!(
            e_iter(5).to_approx_string(27),
            "2.7182818284590452353602874",
        );
        assert_eq!(
            e_iter(7).to_approx_string(29),
            "2.718281828459045235360287471",
        );

        assert_eq!(
            std::f64::consts::E,
            e_iter(3).to_ieee754_f64().unwrap(),
        );
    }
}
