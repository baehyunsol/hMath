use crate::UBigInt;
use crate::consts::U32_OVER;
use crate::utils::{v64_to_v32, remove_suffix_0};

impl UBigInt {

    // TODO: https://en.wikipedia.org/wiki/Karatsuba_algorithm
    #[must_use]
    pub fn mul_ubi(&self, other: &UBigInt) -> Self {
        let mut result = vec![0; self.len() + other.len()];

        for i in 0..self.len() {

            for j in 0..other.len() {
                let curr = self.0[i] as u64 * other.0[j] as u64;
                result[i + j] += curr % U32_OVER;
                result[i + j + 1] += curr / U32_OVER;
            }

        }

        let mut result = v64_to_v32(result);
        remove_suffix_0(&mut result);

        let result = UBigInt::from_raw(result);

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn mul_ubi_mut(&mut self, other: &UBigInt) {
        let result = self.mul_ubi(other);
        *self = result;
    }

    #[must_use]
    pub fn mul_u32(&self, other: u32) -> Self {
        let mut result = self.clone();
        result.mul_u32_mut(other);

        #[cfg(test)] {
            let t = self.mul_ubi(&UBigInt::from_u32(other));
            assert_eq!(t, result);
            assert!(result.is_valid());
        }

        result
    }

    pub fn mul_u32_mut(&mut self, other: u32) {
        let mut carry = 0;

        for i in 0..self.len() {

            match self.0[i].checked_mul(other) {
                Some(n) => match n.checked_add(carry as u32) {
                    Some(n) => {
                        self.0[i] = n;
                        carry = 0;
                    }
                    _ => {
                        carry = (n as u64 + carry) / U32_OVER;
                        self.0[i] = ((n as u64 + carry) % U32_OVER) as u32;
                    }
                }
                _ => {
                    let curr = self.0[i] as u64 * other as u64 + carry;
                    carry = curr / U32_OVER;
                    self.0[i] = (curr % U32_OVER) as u32;
                }
            }

        }

        if carry > 0 {
            self.0.push(carry as u32);
        }

        #[cfg(test)] assert!(self.is_valid());
    }

    /// multiplies 2^`exp`
    // first multiply, then shift
    #[must_use]
    pub fn mul_pow2(&self, exp: u32) -> Self {
        let mut result = self.clone();
        result.mul_pow2_mut(exp);

        result
    }

    /// multiplies 2^`exp`
    // first multiply, then shift
    pub fn mul_pow2_mut(&mut self, exp: u32) {
        let small = 1 << (exp % 32);
        let big = exp / 32;

        self.mul_u32_mut(small);
        self.shift_left_mut(big as usize);
    }
}

#[cfg(test)]
mod tests {
    use crate::UBigInt;

    #[test]
    fn mul_pow2_test() {
        let two = UBigInt::from_u32(2);
        let three = UBigInt::from_u32(3);

        for i in 16..64 {
            assert_eq!(
                three.mul_pow2(i * 8),
                three.mul_ubi(&two.pow_u32(i * 8))
            );
        }

    }

}