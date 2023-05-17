use crate::Ratio;

/// It returns `sin(x)`. It gets more accurate as `iter` gets bigger.\
/// TODO: not implemented yet
pub fn sin_iter(x: &Ratio, iter: usize) -> Ratio {
    todo!()
}

/// It returns `cos(x)`. It gets more accurate as `iter` gets bigger.\
/// TODO: not implemented yet
pub fn cos_iter(x: &Ratio, iter: usize) -> Ratio {
    todo!()
}

/// It returns `tan(x)`. It gets more accurate as `iter` gets bigger.\
/// TODO: not implemented yet
pub fn tan_iter(x: &Ratio, iter: usize) -> Ratio {
    sin_iter(x, iter).div_rat(&cos_iter(x, iter))
}

#[cfg(test)]
mod tests {
    use crate::{Ratio, sqrt_iter, sin_iter, cos_iter, pi_iter};
    use crate::utils::are_close;

    #[test]
    fn sin_test() {
        let iter = 8;
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
        ];
        let pi = pi_iter(iter);
        let cos_coeff = Ratio::from_denom_and_numer_i32(-2, 1);
        let accuracy = 0.001;

        for (numer, denom, value) in samples.into_iter() {
            let sin_val = sin_iter(&Ratio::from_denom_and_numer_i32(denom, numer).mul_rat(&pi), iter);
            let cos_val = cos_iter(&Ratio::from_denom_and_numer_i32(denom, numer).add_rat(&cos_coeff).mul_rat(&pi), iter);

            assert!(are_close(&sin_val, &value, accuracy));
            assert!(are_close(&cos_val, &value, accuracy));
        }

    }

}