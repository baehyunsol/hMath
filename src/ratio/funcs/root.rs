use crate::{UBigInt, BigInt, Ratio};

/// It returns `sqrt(abs(x))`. It gets more accurate as `iter` gets bigger.
pub fn sqrt_iter(x: &Ratio, iter: usize) -> Ratio {
    Ratio::from_denom_and_numer(
        x.denom.shift_left(1 + iter).sqrt(),
        x.numer.shift_left(1 + iter).sqrt(),
    )
}

/// It returns `cbrt(x)`. It gets more accurate as `iter` gets bigger.
pub fn cbrt_iter(x: &Ratio, iter: usize) -> Ratio {

    // for now, ln2_accurate(0) returns 0, which makes the below code invalid
    if x.is_zero() {
        return Ratio::zero();
    }

    // Safety: `log2_accurate` doesn't return a negative number
    let mut approx = Ratio::from_denom_and_numer(
        BigInt::from_ubi(UBigInt::exp2_accurate(&x.denom.shift_left(1).log2_accurate().div_i32(3).to_ubi().unwrap()), false),
        BigInt::from_ubi(UBigInt::exp2_accurate(&x.numer.shift_left(1).log2_accurate().div_i32(3).to_ubi().unwrap()), x.is_neg())
    );
    let x_third = x.div_i32(3);
    let two_third = Ratio::from_denom_and_numer_i32(3, 2);

    // a2 = a1 - (a1^3 - x)/3a1^2 = a1 * 2/3 + x/3a1^2
    for _ in 0..iter {
        let len_pre = approx.denom.len().min(approx.numer.len());
        approx = approx.mul_rat(&two_third).add_rat(&x_third.div_rat(&approx).div_rat(&approx));
        let len_post = approx.denom.len().min(approx.numer.len());

        // ratio operations generate unnecessary digits, which has to be remove for the sake of performance -> it doesn't hurt accuracy at all!
        if len_pre + 4 < len_post {
            approx.denom.shift_right_mut(len_post - len_pre - 4);
            approx.numer.shift_right_mut(len_post - len_pre - 4);
            approx.fit();
        }
    }

    approx
}

#[cfg(test)]
mod tests {
    use crate::{Ratio, sqrt_iter, cbrt_iter, pow_iter};
    use crate::utils::are_close;
    use crate::consts::RUN_ALL_TESTS;

    #[test]
    fn root_test() {
        assert_eq!("0.3162277660168379331998", sqrt_iter(&Ratio::from_string("0.1").unwrap(), 4).to_approx_string(24));
        assert_eq!("10", cbrt_iter(&Ratio::from_i32(1000), 3).to_approx_string(24));

        if !RUN_ALL_TESTS { return; }

        let numbers = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
            16, 25, 27, 28, 29, 30, 1000, 2000, 3000,
            4000, 5000, 6000, 0x1000, 0x2000, 0x3000,
            0x3001, 0x3002, 0x123456789,
        ];
        let iter = 4;
        let accuracy = 0.005;
        let half = Ratio::from_denom_and_numer_i32(2, 1);
        let third = Ratio::from_denom_and_numer_i32(3, 1);

        for number in numbers.into_iter() {
            let number = Ratio::from_i128(number);
            let sqrt1 = sqrt_iter(&number, iter);
            let sqrt2 = pow_iter(&number, &half, iter * 4);

            if !are_close(&sqrt1, &sqrt2, accuracy) {
                panic!("{sqrt1}, {sqrt2}, {number}");
            }

            let cbrt1 = cbrt_iter(&number, iter);
            let cbrt2 = pow_iter(&number, &third, iter * 4);

            if !are_close(&cbrt1, &cbrt2, accuracy) {
                panic!("{cbrt1}, {cbrt2}, {number}");
            }
        }
    }

    #[test]
    fn sqrt2_test() {
        assert_eq!(
            std::f64::consts::SQRT_2,
            sqrt_iter(&2.into(), 3).to_ieee754_f64().unwrap(),
        );
    }
}
