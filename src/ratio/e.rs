use crate::{BigInt, Ratio};

/// It returns an approximate value of E.
/// It gets more and more accurate as `k` gets bigger.
pub fn e_iter(k: usize) -> Ratio {
    let mut numer = BigInt::from_i64(3554627472076);
    let mut denom = BigInt::from_i64(1307674368000);
    let mut curr_fac = 16;

    for _ in 0..k {
        denom.mul_i32_mut(curr_fac);
        numer.mul_i32_mut(curr_fac);
        numer.add_i32_mut(1);
        curr_fac += 1;
    }

    Ratio::from_denom_and_numer(denom, numer)
}

#[cfg(test)]
mod tests {
    use super::e_iter;

    #[test]
    fn e_test() {
        assert_eq!(
            e_iter(2).to_approx_string(17),
            "2.718281828459045"
        );
        assert_eq!(
            e_iter(4).to_approx_string(19),
            "2.71828182845904523"
        );
        assert_eq!(
            e_iter(5).to_approx_string(21),
            "2.7182818284590452353"
        );
        assert_eq!(
            e_iter(6).to_approx_string(21),
            "2.7182818284590452353"
        );
        assert_eq!(
            e_iter(7).to_approx_string(24),
            "2.7182818284590452353602"
        );
    }

}