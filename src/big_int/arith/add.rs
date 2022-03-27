use crate::big_int::{BigInt, BASE};
use std::ops::Add;


impl Add for &BigInt {
    type Output = BigInt;

    fn add(self, other: &BigInt) -> BigInt {

        if self.is_zero() {
            return other.clone();
        }

        if other.is_zero() {
            return self.clone();
        }

        if self.is_negative != other.is_negative {
            return self - &(-other);
        }

        let mut index = 0;
        let min_len = self.len().min(other.len());
        let mut result = Vec::with_capacity(self.len().max(other.len()));

        while index < min_len {
            result.push(self.data[index] + other.data[index]);
            index += 1;
        }

        while index < self.len() {
            result.push(self.data[index]);
            index += 1;
        }

        while index < other.len() {
            result.push(other.data[index]);
            index += 1;
        }

        let mut result = BigInt {
            is_negative: self.is_negative,
            data: result
        };

        result.trim();

        result
    }

}


impl Add<u32> for &BigInt {
    type Output = BigInt;

    fn add(self, other: u32) -> BigInt {

        if self.is_zero() {
            BigInt::from_u32(other)
        }

        else if other < BASE && !self.is_negative {
            let mut result = self.clone();
            result.data[0] += other;
            result.trim();

            result
        }

        else {
            self + &BigInt::from_u32(other)
        }

    }

}


impl Add<u8> for &BigInt {
    type Output = BigInt;

    fn add(self, other: u8) -> BigInt {
        self + other as u32
    }

}


impl Add<i32> for &BigInt {
    type Output = BigInt;

    fn add(self, other: i32) -> BigInt {

        if self.is_zero() {
            BigInt::from_i32(other)
        }

        else if other >= 0 {
            self + other as u32
        }

        else {
            self + &BigInt::from_i32(other)
        }

    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn add_test() {
        use crate::big_int::{BigInt, BASE};

        for _ in 0..0x400 {
            let n1 = rand::random::<i128>() / 2;
            let n2 = rand::random::<i128>() / 2;
            let ans = n1 + n2;

            if &BigInt::from_i128(n1) + &BigInt::from_i128(n2) != BigInt::from_i128(ans) {
                panic!(
                    "n1: {} -> {:?}\nn2: {} -> {:?}\nn1 + n2: {:?}\nans: {} -> {:?}",
                    n1, BigInt::from_i128(n1),
                    n2, BigInt::from_i128(n2),
                    &BigInt::from_i128(n1) + &BigInt::from_i128(n2),
                    ans, BigInt::from_i128(ans)
                );
            }

            let n3 = rand::random::<u32>();

            if &BigInt::from_i128(n1) + n3 != &BigInt::from_i128(n1) + &BigInt::from_u32(n3) {
                panic!(
                    "n1: {} -> {:?}\nn3: {} -> {:?}\nMul<u32>: {:?}\nMul<BigInt>: {:?}",
                    n1, BigInt::from_i128(n1),
                    n3, BigInt::from_u32(n3),
                    &BigInt::from_i128(n1) + n3,
                    &BigInt::from_i128(n1) + &BigInt::from_u32(n3),
                );
            }

        }

        let coeff = (BASE as i128 / 16).max(1);

        for i in -32..32 {
            let i: i128 = i * i * i;

            for j in -32..32 {
                let j: i128 = j * j * j;
                assert_eq!(&BigInt::from_i128(i * coeff) + &BigInt::from_i128(j * coeff), BigInt::from_i128(i * coeff + j * coeff));
                assert_eq!(&BigInt::from_i128(i * coeff) + j.abs() as u32, &BigInt::from_i128(i * coeff) + &BigInt::from_u32(j.abs() as u32));
            }

        }

    }
}