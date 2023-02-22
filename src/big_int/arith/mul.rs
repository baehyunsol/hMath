use crate::big_int::{BigInt, BASE, trim_u64_to_u32};
use std::ops::Mul;

impl Mul for &BigInt {
    type Output = BigInt;

    fn mul(self, other: &BigInt) -> BigInt {

        if self.is_zero() || other.is_zero() {
            return BigInt::zero();
        }

        let mut data = vec![0;self.len() + other.len()];
        let base_u64 = BASE as u64;

        for i in 0..self.len() {

            for j in 0..other.len() {
                let curr = self.data[i] as u64 * other.data[j] as u64;

                data[i + j] += (curr % base_u64) as u32;
                data[i + j + 1] += (curr / base_u64) as u32;

                if data[i + j] >= BASE {
                    data[i + j + 1] += data[i + j] / BASE;
                    data[i + j] %= BASE;
                }

                if data[i + j + 1] >= BASE {
                    data[i + j + 2] += data[i + j + 1] / BASE;
                    data[i + j + 1] %= BASE;
                }

            }

        }

        let mut result = BigInt {
            data, is_negative: self.is_negative != other.is_negative
        };

        result.trim();
        result.trim_zero();

        result
    }

}

impl Mul<u32> for &BigInt {
    type Output = BigInt;

    fn mul(self, other: u32) -> BigInt {

        if self.is_zero() || other == 0 {
            return BigInt::zero();
        }

        let other = other as u64;
        let data = self.data.iter().map(|n| *n as u64 * other).collect::<Vec<u64>>();

        BigInt {
            is_negative: self.is_negative,
            data: trim_u64_to_u32(data)
        }
    }

}

impl Mul<u8> for &BigInt {
    type Output = BigInt;

    fn mul(self, other: u8) -> BigInt {
        self * other as u32
    }

}

impl Mul<i32> for &BigInt {
    type Output = BigInt;

    fn mul(self, other: i32) -> BigInt {

        if self.is_zero() || other == 0 {
            BigInt::zero()
        }
        
        else if other > 0 {
            self * other as u32
        }

        else {
            self * &BigInt::from_i32(other)
        }
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn mul_test() {
        use crate::big_int::{BigInt, BASE};

        for _ in 0..0x400 {
            let n1 = rand::random::<i128>() % 100_000_000;
            let n2 = rand::random::<i128>() % 100_000_000;
            let ans = n1 * n2;

            if &BigInt::from_i128(n1) * &BigInt::from_i128(n2) != BigInt::from_i128(ans) {
                panic!(
                    "n1: {} -> {:?}\nn2: {} -> {:?}\nn1 * n2: {:?}\nans: {} -> {:?}",
                    n1, BigInt::from_i128(n1),
                    n2, BigInt::from_i128(n2),
                    &BigInt::from_i128(n1) * &BigInt::from_i128(n2),
                    ans, BigInt::from_i128(ans)
                );
            }

            let n3 = rand::random::<u32>();

            if &BigInt::from_i128(n1) * n3 != &BigInt::from_i128(n1) * &BigInt::from_u32(n3) {
                panic!(
                    "n1: {} -> {:?}\nn3: {} -> {:?}\nMul<u32>: {:?}\nMul<BigInt>: {:?}",
                    n1, BigInt::from_i128(n1),
                    n3, BigInt::from_u32(n3),
                    &BigInt::from_i128(n1) * n3,
                    &BigInt::from_i128(n1) * &BigInt::from_u32(n3),
                );
            }

        }

        let coeff = (BASE as i128 / 16).max(1);

        for i in -32..32 {
            let i: i128 = i * i * i;

            for j in -32..32 {
                let j: i128 = j * j * j;
                assert_eq!(&BigInt::from_i128(i * coeff) * &BigInt::from_i128(j * coeff), BigInt::from_i128(i * coeff * j * coeff));
                assert_eq!(&BigInt::from_i128(i * coeff) * j.abs() as u32, &BigInt::from_i128(i * coeff) * &BigInt::from_u32(j.abs() as u32));
            }

        }

    }

}
