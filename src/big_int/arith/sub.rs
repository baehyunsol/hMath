use crate::big_int::{BigInt, BASE};
use std::ops::{Sub, Neg};

impl Sub for &BigInt {
    type Output = BigInt;

    fn sub(self, other: &BigInt) -> BigInt {

        if self.is_zero() {
            return -other;
        }

        if other.is_zero() {
            return self.clone();
        }

        if self.is_negative != other.is_negative {
            return self + &(-other);
        }

        let mut result = Vec::with_capacity(self.len().max(other.len()));
        let min_len = self.len().min(other.len());
        let mut index = 0;

        while index < min_len {
            result.push(self.data[index] as i64 - other.data[index] as i64);
            index += 1;
        }

        while index < self.len() {
            result.push(self.data[index] as i64);
            index += 1;
        }

        while index < other.len() {
            result.push(-(other.data[index] as i64));
            index += 1;
        }

        let last_index = result.len() - 1;

        let base_i64 = BASE as i64;
        let mut is_result_negative = false;

        for i in 0..last_index {

            while result[i] < 0 {
                result[i] += base_i64;
                result[i + 1] -= 1;
            }

        }

        if result[last_index] < 0 {

            if result.len() == 1 {
                result[0] *= -1;
            }

            else {
                result[0] = base_i64 - result[0];

                for i in 1..last_index {
                    result[i] = base_i64 - result[i] - 1;
                }

                result[last_index] = -result[last_index] - 1;
            }

            is_result_negative = true;
        }

        let mut result = BigInt {
            is_negative: is_result_negative ^ self.is_negative,
            data: result.iter().map(|n| *n as u32).collect()
        };

        result.trim();
        result.trim_zero();

        result
    }

}

impl Neg for &BigInt {
    type Output = BigInt;

    fn neg(self) -> BigInt {

        if self.is_zero() {
            self.clone()
        }

        else {
            BigInt {
                is_negative: !self.is_negative,
                data: self.data.clone()
            }
        }

    }

}

impl Sub<u32> for &BigInt {
    type Output = BigInt;

    fn sub(self, other: u32) -> BigInt {

        if self.is_zero() {
            let mut result = BigInt {
                is_negative: true,
                data: vec![other]
            };

            result.trim();

            result
        }

        else if self.is_negative {

            if other < BASE {
                let mut result = self.clone();
                result.data[0] += other;
                result.trim();

                result
            }

            else {
                self - &BigInt::from_u32(other)
            }

        }

        else if self.data[0] >= other {
            let mut result = self.clone();
            result.data[0] -= other;

            result.trim_zero();

            result
        }

        else {
            self - &BigInt::from_u32(other)
        }

    }

}

impl Sub<i32> for &BigInt {
    type Output = BigInt;

    fn sub(self, other: i32) -> BigInt {

        if other >= 0 {
            self - other as u32
        }

        else {
            self - &BigInt::from_i32(other)
        }

    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn sub_test() {
        use crate::big_int::{BigInt, BASE};

        for _ in 0..0x400 {
            let n1 = rand::random::<i128>() / 2;
            let n2 = rand::random::<i128>() / 2;
            let ans = n1 - n2;

            if &BigInt::from_i128(n1) - &BigInt::from_i128(n2) != BigInt::from_i128(ans) {
                panic!(
                    "n1: {} -> {:?}\nn2: {} -> {:?}\nn1 - n2: {:?}\nans: {} -> {:?}",
                    n1, BigInt::from_i128(n1),
                    n2, BigInt::from_i128(n2),
                    &BigInt::from_i128(n1) - &BigInt::from_i128(n2),
                    ans, BigInt::from_i128(ans)
                );
            }
        }

        let coeff = BASE as i128 / 4;

        for i in -32..32 {

            for j in -32..32 {
                assert_eq!(&BigInt::from_i128(i * coeff) - &BigInt::from_i128(j * coeff), BigInt::from_i128(i * coeff - j * coeff));
            }

        }

    }

    #[test]
    fn neg_test() {
        use crate::big_int::BigInt;

        for i in -12..12 {
            let i = i * i * i;
            assert_eq!(BigInt::from_i32(i), -&-&BigInt::from_i32(i));
        }

    }
}

use crate::Ratio;

impl Sub<&Ratio> for &BigInt {
    type Output = Ratio;

    fn sub(self, other: &Ratio) -> Ratio {
        Ratio {
            denom: other.denom.clone(),
            numer: &(self * &other.denom) - &other.numer
        }
    }

}