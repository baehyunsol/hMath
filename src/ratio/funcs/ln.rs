use crate::{Ratio, BigInt, ln2_iter};

/// It returns `ln(x)`. It gets more accurate as `iter` gets bigger. It panics when `x` is less than 0.
pub fn ln_iter(x: &Ratio, iter: usize) -> Ratio {
    if x.is_neg() {
        panic!("logarithm of a negative number is undefined");
    }

    else if x.is_zero() {
        panic!("logarithm of 0 is undefined");
    }

    // ln(x) = ln(1 + a) = sum{k=1}{inf} -(-a)^k/k = a - a^2/2 + a^3/3 - a^4/4...
    // it's best when a is close to 0 -> log_2(1 + a) = log_2(x) is close to 0
    // approximation of log_2 is very easily calculated: log2_accurate
    let log2_approx = x.numer.log2_accurate().sub(&x.denom.log2_accurate()).shift_right(1).to_i64().unwrap();
    let mut x_iter = x.clone();
    let mut log2_approx_counter = log2_approx.abs();

    // x /= 2^log2_approx
    if log2_approx > 0 {
        while log2_approx_counter > 0 && x_iter.numer.rem_pow2(2).is_zero() {
            log2_approx_counter -= 1;
            x_iter.numer.div_i32_mut(2);
        }

        if log2_approx_counter % 32 == 31 {
            log2_approx_counter -= 1;
            x_iter.denom.mul_i32_mut(2);
        }

        x_iter.denom.mul_i32_mut((1 << (log2_approx_counter % 32)) as i32);
        x_iter.denom.shift_left_mut((log2_approx_counter / 32) as usize);
    }

    // x *= 2^log2_approx.abs()
    else {
        while log2_approx_counter > 0 && x_iter.denom.rem_pow2(2).is_zero() {
            log2_approx_counter -= 1;
            x_iter.denom.div_i32_mut(2);
        }

        if log2_approx_counter % 32 == 31 {
            log2_approx_counter -= 1;
            x_iter.numer.mul_i32_mut(2);
        }

        x_iter.numer.mul_i32_mut((1 << (log2_approx_counter % 32)) as i32);
        x_iter.numer.shift_left_mut((log2_approx_counter / 32) as usize);
    }

    // now, x = x_iter * 2^log2_approx
    // ln(x) = ln(x_iter) + log2_approx * ln(2)
    x_iter.sub_i32_mut(1);
    let a = x_iter.clone();
    let mut result = a.clone();

    for k in 0..iter {
        x_iter.mul_mut(&a);
        result.sub_mut(&x_iter.div_i32((2 * k + 2) as i32));
        x_iter.mul_mut(&a);
        result.add_mut(&x_iter.div_i32((2 * k + 3) as i32));
    }

    result.add(&ln2_iter(iter).mul_bi(&BigInt::from_i64(log2_approx)))
}

/// It returns log(x) with base `base`. It gets more accurate as `iter` gets bigger. It panics when `x` or `base` is less than or equal 0.
pub fn log_iter(base: &Ratio, x: &Ratio, iter: usize) -> Ratio {
    ln_iter(x, iter).div(&ln_iter(base, iter))
}

#[cfg(test)]
mod tests {
    use crate::{Ratio, BigInt, ln_iter, exp_iter, log_iter};
    use crate::utils::are_close;

    #[test]
    fn ln_test() {
        assert_eq!("3.141592", exp_iter(&ln_iter(&Ratio::from_string("3.14159265").unwrap(), 11), 11).to_approx_string(8));
        assert_eq!("9.999999", exp_iter(&ln_iter(&Ratio::from_string("10").unwrap(), 6), 6).to_approx_string(8));
        assert_eq!("1", ln_iter(&Ratio::from_string("2.718281828459045").unwrap(), 6).to_approx_string(8));
        assert_eq!("0.6931471", ln_iter(&Ratio::from_string("2").unwrap(), 6).to_approx_string(9));
        assert_eq!("-1.386294", ln_iter(&Ratio::from_string("0.25").unwrap(), 6).to_approx_string(9));

        assert_eq!(
            std::f64::consts::LN_10,
            ln_iter(&10.into(), 11).to_ieee754_f64().unwrap(),
        );
    }

    #[test]
    fn log_test() {
        assert_eq!(
            std::f64::consts::LOG2_10,
            log_iter(&2.into(), &10.into(), 12).to_ieee754_f64().unwrap(),
        );
        assert_eq!(
            std::f64::consts::LOG10_2,
            log_iter(&10.into(), &2.into(), 11).to_ieee754_f64().unwrap(),
        );

        let nums = vec![
            0.5f64, 1.6, 3.2,
            2.0, 1624.5, 4.9,
            1.01, 9932.0, 0.1,
        ];
        let accr = 3e-8;

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                let a = nums[i];
                let b = nums[j];
                let ans_f64 = b.log(a);
                let a_rat = Ratio::from_ieee754_f64(a).unwrap();
                let b_rat = Ratio::from_ieee754_f64(b).unwrap();
                let ans_f64_rat = Ratio::from_ieee754_f64(ans_f64).unwrap();
                let ans_rat_1 = Ratio::from_denom_and_numer(
                    BigInt::from_raw(vec![0, 1], false),
                    b_rat.numer.log2_accurate().sub(&b_rat.denom.log2_accurate()),
                ).div(&Ratio::from_denom_and_numer(
                    BigInt::from_raw(vec![0, 1], false),
                    a_rat.numer.log2_accurate().sub(&a_rat.denom.log2_accurate()),
                ));
                let ans_rat_2 = log_iter(
                    &a_rat, &b_rat, 14
                );

                assert!(are_close(&ans_f64_rat, &ans_rat_1, accr));
                assert!(are_close(&ans_f64_rat, &ans_rat_2, accr));
                assert!(are_close(&ans_rat_1, &ans_rat_2, accr));
            }
        }
    }
}
