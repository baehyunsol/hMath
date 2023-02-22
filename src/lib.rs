mod big_int;
mod ratio;
mod wrapper;
mod matrix;

pub use crate::big_int::{BigInt, funcs};
pub use crate::ratio::Ratio;
pub use crate::wrapper::Number;
pub use crate::matrix::Matrix;

#[macro_export]
macro_rules! n {($s: expr) => ({
    Number::from_string(String::from($s)).unwrap()
})}

#[cfg(test)]
mod tests {
    #[test]
    fn factorial_test() {
        use crate::BigInt;
        let mut result = BigInt::from_u32(1);

        for i in 1..16u32 {
            result = &result * i;
        }

        assert_eq!(result, BigInt::from_u64(1_3076_7436_8000));
    }

    #[test]
    fn e_test() {
        use crate::{BigInt, Ratio};
        let mut e = Ratio::from_u32(1);
        let mut curr_fac = BigInt::one();

        for i in 1..32u32 {
            curr_fac = &curr_fac * i;
            e = &e + &Ratio::from_big_ints(curr_fac.clone(), BigInt::one());
        }

        let ans = Ratio::from_string("2.71828182845904523".to_string()).unwrap();
        let err = Ratio::from_string("1e-16".to_string()).unwrap();

        assert!((&e - &ans).abs() < err);
    }

    #[test]
    fn pi_test() {
        use crate::{BigInt, Ratio};

        let mut pi = Ratio::zero();

        // https://en.wikipedia.org/wiki/Pi#Spigot_algorithms
        for k in 0..36 {
            let n1 = Ratio::from_big_int(BigInt::from_u32(16).pow(k)).reci();
            let n2 = Ratio::from_u32s(8 * k + 1, 4);
            let n3 = Ratio::from_u32s(8 * k + 4, 2);
            let n4 = Ratio::from_u32s(8 * k + 5, 1);
            let n5 = Ratio::from_u32s(8 * k + 6, 1);

            pi = &pi + &(&n1 * &(&(&(&n2 - &n3) - &n4) - &n5))
        }

        let ans = Ratio::from_string("3.141592653589793238462643383279".to_string()).unwrap();
        let err = Ratio::from_string("1e-30".to_string()).unwrap();

        assert!((&pi - &ans).abs() < err);
    }

}