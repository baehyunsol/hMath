use crate::big_int::{BigInt, BASE};
use std::ops::Rem;

impl Rem for &BigInt {
    type Output = BigInt;

    fn rem(self, other: &BigInt) -> BigInt {
        self - &(&(self / other) * other)
    }

}

impl Rem<u32> for &BigInt {
    type Output = BigInt;

    fn rem(self, other: u32) -> BigInt {

        if !self.is_negative && other < BASE {
            let unit = BASE % other;
            let mut curr_power = 1;
            let mut result = 0;

            for d in self.data.iter() {
                result += (d % other) * curr_power % other;
                curr_power *= unit;
                curr_power %= other;
            }

            BigInt::from_u32(result % other)
        }

        else {
            self - &(&(self / other) * other)
        }

    }

}

impl Rem<i32> for &BigInt {
    type Output = BigInt;

    fn rem(self, other: i32) -> BigInt {

        if other > 0 {
            self % other as u32
        }

        else {
            self - &(&(self / other) * other)
        }

    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn rem_test() {
        use crate::big_int::{BigInt, BASE};

        for _ in 0..0x400 {
            let n1 = rand::random::<i128>() % 100_000_000;
            let n2 = rand::random::<i128>() % 100_000_000;

            if n2 == 0 {
                continue;
            }

            let ans = n1 % n2;

            if &BigInt::from_i128(n1) % &BigInt::from_i128(n2) != BigInt::from_i128(ans) {
                panic!(
                    "n1: {} -> {:?}\nn2: {} -> {:?}\nn1 % n2: {:?}\nans: {} -> {:?}",
                    n1, BigInt::from_i128(n1),
                    n2, BigInt::from_i128(n2),
                    &BigInt::from_i128(n1) % &BigInt::from_i128(n2),
                    ans, BigInt::from_i128(ans)
                );
            }

            let n3 = rand::random::<u32>();

            if n3 == 0 {
                continue;
            }

            if &BigInt::from_i128(n1) % n3 != &BigInt::from_i128(n1) % &BigInt::from_u32(n3) {
                panic!(
                    "n1: {} -> {:?}\nn3: {} -> {:?}\nRem<u32>: {:?}\nRem<BigInt>: {:?}",
                    n1, BigInt::from_i128(n1),
                    n3, BigInt::from_u32(n3),
                    &BigInt::from_i128(n1) % n3,
                    &BigInt::from_i128(n1) % &BigInt::from_u32(n3),
                );
            }

        }

        let coeff = (BASE as i128 / 16).max(1);

        for i in -32..32 {
            let i: i128 = i * i * i;

            for j in -32..32 {

                if j == 0 {
                    continue;
                }

                let j: i128 = j * j * j;

                assert_eq!(&BigInt::from_i128(i * coeff) % &BigInt::from_i128(j * coeff), BigInt::from_i128((i * coeff) % (j * coeff)));
                assert_eq!(&BigInt::from_i128(i * coeff) % j.abs() as u32, &BigInt::from_i128(i * coeff) % &BigInt::from_u32(j.abs() as u32));
            }

        }

    }

}

use crate::Ratio;

impl Rem<&Ratio> for &BigInt {
    type Output = Ratio;

    fn rem(self, other: &Ratio) -> Ratio {
        self - &(&((self / other).floor()) * other)
    }

}