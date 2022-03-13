mod big_int;
mod ratio;

pub use crate::big_int::{BigInt, funcs};
pub use crate::ratio::Ratio;

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

}