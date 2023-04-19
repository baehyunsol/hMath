use super::UBigInt;

impl UBigInt {

    /// It returns 0 when `self` is 0.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2(&self) -> Self {
        // It assumes that `self` is less than 2^(2^64)
        UBigInt::from_u64((self.len() as u64 - 1) * 32 + log2_u32(self.0[self.len() - 1]) as u64)
    }

    /// It returns `truncate(log2(self) * 16777216)`.
    /// Warning: This function is very expensive.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2_accurate(&self) -> Self {
        let mut result = UBigInt::zero();
        let mut self_clone = if self.len() > 3 {
            result.add_ubi_mut(&UBigInt::from_u64((self.len() - 3) as u64).mul_u32(32));
            self.shift_right(self.len() - 3)
        } else {
            self.clone()
        };

        // self = self^256
        for _ in 0..8 {
            self_clone = self_clone.mul_ubi(&self_clone);
        }

        result.mul_u32_mut(256);

        for _ in 0..2 {

            if self_clone.len() > 3 {
                result.add_ubi_mut(&UBigInt::from_u64((self_clone.len() - 3) as u64).mul_u32(32));
                self_clone.shift_right_mut(self_clone.len() - 3);
            }

            // self = self^256
            for _ in 0..8 {
                self_clone = self_clone.mul_ubi(&self_clone);
            }

            result.mul_u32_mut(256);
        }

        result.add_ubi_mut(&self_clone.log2());
        result
    }

}

// floor(log2(n))
pub fn log2_u32(mut n: u32) -> u32 {
    let mut result = 0;

    while n > 1024 {
        n /= 1024;
        result += 10;
    }

    while n > 32 {
        n /= 32;
        result += 5;
    }

    while n > 1 {
        n /= 2;
        result += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::UBigInt;
    use crate::consts::RUN_ALL_TESTS;

    #[test]
    fn log_test() {
        if !RUN_ALL_TESTS { return; }

        let mut n = UBigInt::from_u32(2);
        let mut i = 1;

        for _ in 0..256 {
            assert_eq!(UBigInt::from_u32(i), n.log2());
            assert_eq!(UBigInt::from_u32(i), n.add_u32(1).log2());
            assert_eq!(UBigInt::from_u32(i - 1), n.sub_u32(1).log2());
            n.mul_u32_mut(2);
            i += 1;
        }

        use crate::{Ratio, BigInt};
        let denom = BigInt::from_i32(16777216);

        assert_eq!(
            Ratio::from_denom_and_numer(
                denom.clone(),
                BigInt::from_i32(3).log2_accurate()
            ).to_approx_string(6),
            "1.5849"
        );
        assert_eq!(
            Ratio::from_denom_and_numer(
                denom.clone(),
                BigInt::from_i32(9900).log2_accurate()
            ).to_approx_string(6),
            "13.273"
        );
    }
}