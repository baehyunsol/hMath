use crate::UBigInt;
use crate::utils::{v64_to_v32, remove_suffix_0};

const KARATSUBA_THRES: usize = 64;

#[cfg(test)]
const KARATSUBA_TEST: bool = crate::consts::RUN_ALL_TESTS & true;

#[cfg(test)]
static mut KARATSUBA_ENABLE: bool = true;

impl UBigInt {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul_ubi(&self, other: &UBigInt) -> Self {

        #[cfg(test)]
        let go_kara = unsafe { KARATSUBA_ENABLE };
        #[cfg(not(test))]
        let go_kara = true;

        // https://en.wikipedia.org/wiki/Karatsuba_algorithm
        if self.len() > KARATSUBA_THRES && other.len() > KARATSUBA_THRES && go_kara {

            // self: a, other: b
            // naive: O(a * b)
            // karatsuba: O(a + b + (a - m) * (b - m) + m * m + a1 * b1)
            let m = (self.len() / 2).min(other.len() / 2);
            let x1 = self.shift_right(m);   // O(a - m)
            let x0 = self.slice_right(m);   // O(m)
            let y1 = other.shift_right(m);  // O(b - m)
            let y0 = other.slice_right(m);  // O(m)
            let z2 = x1.mul_ubi(&y1);  // O((a - m) * (b - m))
            let z0 = x0.mul_ubi(&y0);  // O(m * m)

            // a1 = max(m, a - m), b1 = max(m, b - m)
            let z1 = x1.add_ubi(&x0).mul_ubi(&y1.add_ubi(&y0)).sub_ubi(&z2).sub_ubi(&z0);  // O(a1 * b1 + 3 * (a1 + b1))

            let result = z2.shift_left(2 * m).add_ubi(&z1.shift_left(m)).add_ubi(&z0);

            #[cfg(test)] unsafe {

                if KARATSUBA_TEST {
                    KARATSUBA_ENABLE = false;
                    let result2 = self.mul_ubi(&other);
                    KARATSUBA_ENABLE = true;

                    assert_eq!(result, result2);
                }

            }

            return result;
        }

        let mut result = vec![0; self.len() + other.len()];

        for i in 0..self.len() {

            for j in 0..other.len() {
                let curr = self.0[i] as u64 * other.0[j] as u64;
                result[i + j] += curr % (1 << 32);
                result[i + j + 1] += curr >> 32;
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

    #[must_use = "method returns a new number and does not mutate the original value"]
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
                        self.0[i] = ((n as u64 + carry) % (1 << 32)) as u32;
                        carry = (n as u64 + carry) >> 32;
                    }
                }
                _ => {
                    let curr = self.0[i] as u64 * other as u64 + carry;
                    carry = curr >> 32;
                    self.0[i] = (curr % (1 << 32)) as u32;
                }
            }

        }

        if carry > 0 {
            self.0.push(carry as u32);
        }

        remove_suffix_0(&mut self.0);
        #[cfg(test)] assert!(self.is_valid());
    }

    /// multiplies 2^`exp`
    // first multiply, then shift
    #[must_use = "method returns a new number and does not mutate the original value"]
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