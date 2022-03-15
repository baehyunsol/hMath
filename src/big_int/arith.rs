mod add;
mod sub;
mod mul;
mod div;
mod rem;

use crate::BigInt;

impl BigInt {

    #[inline]
    pub fn abs(&self) -> BigInt {

        if self.is_negative {
            -self
        }

        else {
            self.clone()
        }

    }

    pub fn pow(&self, n: u32) -> BigInt {

        if *self == 1 {
            self.clone()
        }

        else if self.is_zero() {
            self.clone()
        }

        else if n < 8 {

            if n == 0 {
                BigInt::one()
            }

            else if n == 1 {
                self.clone()
            }

            else {
                let mut result = self.clone();

                for _ in 1..n {
                    result = &result * self;
                }

                result
            }

        }

        else {
            let mut curr = self.clone();
            let n_binary = _into_binary(n);
            let mut result = BigInt::one();

            for i in 0..n_binary.len() {

                if n_binary[i] {
                    result = &result * &curr;
                }

                curr = &curr * &curr;
            }

            result
        }

    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn pow_test() {
        use crate::BigInt;

        let ns = (-5..5i32).map(|n| BigInt::from_i32(n)).collect::<Vec<BigInt>>();

        assert_eq!(ns[0].pow(3), BigInt::from_i32(-125));
        assert_eq!(ns[1].pow(4), BigInt::from_i32(256));
        assert_eq!(ns[5].pow(1), ns[5]);
        assert_eq!(ns[5].pow(2), ns[5]);
        assert_eq!(ns[6].pow(0), ns[6]);
        assert_eq!(ns[6].pow(1), ns[6]);
        assert_eq!(ns[6].pow(2), ns[6]);
        assert_eq!(ns[7].pow(2), ns[9]);
        assert_eq!(ns[8].pow(4), BigInt::from_u32(81));
        assert_eq!(ns[9].pow(3), BigInt::from_u32(64));
        assert_eq!(ns[7].pow(15), BigInt::from_u32(32768));
        assert_eq!(ns[8].pow(15), BigInt::from_u32(14348907));
        assert_eq!(ns[1].pow(15), BigInt::from_i32(-0x40_000_000));
    }

}


fn _into_binary(mut n: u32) -> Vec<bool> {

    let mut binary = vec![];

    while n > 0 {
        binary.push(n % 2 == 1);
        n /= 2;
    }

    binary
}