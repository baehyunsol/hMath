use crate::big_int::{BigInt, BASE, trim_u64_to_u32};
use std::ops::Div;


impl Div for &BigInt {
    type Output = BigInt;

    fn div(self, other: &BigInt) -> BigInt {

        let mut result = if self.is_negative {

            if other.is_negative {
                _pos_div(&-self, &-other)
            }

            else {
                let mut result = _pos_div(&-self, other);
                result.is_negative = true;

                result
            }
        
        }

        else {

            if other.is_negative {
                let mut result = _pos_div(self, &-other);
                result.is_negative = true;

                result
            }

            else {
                _pos_div(self, other)
            }

        };

        if result.is_zero() {
            result.is_negative = false;
        }

        result
    }

}


// a and b are both positive
fn _pos_div(a: &BigInt, b: &BigInt) -> BigInt {

    if a < b || a.is_zero() {
        BigInt::zero()
    }

    else if b.is_zero() {
        panic!("Zero Division Error!")
    }

    else if b.len() == 1 {

        if a.len() > 1 {
            a / b.data[0]
        }

        else {
            let data = vec![a.data[0] / b.data[0]];

            BigInt {
                data, is_negative: false
            }
        }

    }

    else if a.len() > b.len() {
        let base_u64 = BASE as u64;
        let a_approx = a.data[a.len() - 1] as u64 * base_u64 + a.data[a.len() - 2] as u64;
        let b_approx = b.data[b.len() - 1] as u64 * base_u64 + b.data[b.len() - 2] as u64;

        let mut data = vec![0;a.len() - b.len()];

        if a_approx / (b_approx + 1) == 0 {
            let a_approx = a.data[a.len() - 1] as u64 * base_u64 + a.data[a.len() - 2] as u64;
            let b_approx = b.data[b.len() - 1] as u64;

            data[a.len() - b.len() - 1] = a_approx / (b_approx + 1);
        }

        else {
            data.push(a_approx / (b_approx + 1));
        }

        let result = BigInt {
            is_negative: false,
            data: trim_u64_to_u32(data)
        };

        &result + &_pos_div(&(a - &(b * &result)), b)
    }

    else {
        let base_u64 = BASE as u64;
        let a_approx = a.data[a.len() - 1] as u64 * base_u64 + a.data[a.len() - 2] as u64;
        let b_approx = b.data[b.len() - 1] as u64 * base_u64 + b.data[b.len() - 2] as u64;

        if a_approx / (b_approx + 1) == 0 {
            BigInt::one()
        }

        else {
            let result = BigInt::from_u64(a_approx / (b_approx + 1));

            &result + &_pos_div(&(a - &(b * &result)), b)
        }

    }

}


impl Div<u32> for &BigInt {
    type Output = BigInt;

    fn div(self, other: u32) -> BigInt {

        if other < 2 {

            if other == 1 {
                self.clone()
            }

            else {
                panic!("Zero Division Error!")
            }

        }

        else {
            let mut data = vec![];
            let mut carry: u64 = 0;
            let base_u64 = BASE as u64;

            for n in self.data.iter().rev() {
                data.push((*n as u64 + carry) / other as u64);
                carry = (*n as u64 + carry) % other as u64 * base_u64;
            }

            data.reverse();

            let data = trim_u64_to_u32(data);

            let mut result = BigInt {
                is_negative: self.is_negative,
                data
            };

            result.trim();
            result.trim_zero();

            result
        }

    }

}

impl Div<i32> for &BigInt {
    type Output = BigInt;

    fn div(self, other: i32) -> BigInt {

        if other > 0 {
            self / other as u32
        }

        else {
            self / &BigInt::from_i32(other)
        }

    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn div_test() {
        use crate::big_int::{BigInt, BASE};

        for _ in 0..0x400 {
            let n1 = rand::random::<i128>() % 100_000_000;
            let n2 = rand::random::<i128>() % 100_000_000;

            if n2 == 0 {
                continue;
            }

            let ans = n1 / n2;

            if &BigInt::from_i128(n1) / &BigInt::from_i128(n2) != BigInt::from_i128(ans) {
                panic!(
                    "n1: {} -> {:?}\nn2: {} -> {:?}\nn1 / n2: {:?}\nans: {} -> {:?}",
                    n1, BigInt::from_i128(n1),
                    n2, BigInt::from_i128(n2),
                    &BigInt::from_i128(n1) / &BigInt::from_i128(n2),
                    ans, BigInt::from_i128(ans)
                );
            }

            let n3 = rand::random::<u32>();

            if n3 == 0 {
                continue;
            }

            if &BigInt::from_i128(n1) / n3 != &BigInt::from_i128(n1) / &BigInt::from_u32(n3) {
                panic!(
                    "n1: {} -> {:?}\nn3: {} -> {:?}\nDiv<u32>: {:?}\nDiv<BigInt>: {:?}",
                    n1, BigInt::from_i128(n1),
                    n3, BigInt::from_u32(n3),
                    &BigInt::from_i128(n1) / n3,
                    &BigInt::from_i128(n1) / &BigInt::from_u32(n3),
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

                assert_eq!(&BigInt::from_i128(i * coeff) / &BigInt::from_i128(j * coeff), BigInt::from_i128((i * coeff) / (j * coeff)));
                assert_eq!(&BigInt::from_i128(i * coeff) / j.abs() as u32, &BigInt::from_i128(i * coeff) / &BigInt::from_u32(j.abs() as u32));
            }

        }

    }

}


use crate::Ratio;

impl Div<&Ratio> for &BigInt {
    type Output = Ratio;

    fn div(self, other: &Ratio) -> Ratio {
        &other.reci() * self
    }

}