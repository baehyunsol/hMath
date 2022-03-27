use crate::big_int::{BigInt, BASE};


pub fn gcd(mut a: BigInt, mut b: BigInt) -> BigInt {

    if a.is_zero() || b.is_zero() {
        panic!("gcd of 0");
    }

    while !b.is_zero() {
        let r = &a % &b;
        a = b;
        b = r;
    }

    a
}


/// inclusive
/// panics if `from` > `to`
pub fn random(from: &BigInt, to: &BigInt) -> BigInt {

    if from > to {
        panic!("`from` > `to`");
    }

    let d = &(to - from) + 1u32;
    let rand_val = _random(d.len() + 1);

    from + &(&rand_val % &d)
}


fn _random(size: usize) -> BigInt {
    BigInt {
        is_negative: false,
        data: (0..size).map(|_| rand::random::<u32>() % BASE).collect()
    }
}


fn is_prime_u32(n: u32) -> bool {

    if n < 2 || (n % 2 == 0 && n != 2) {
        return false;
    }

    let mut div = 3;

    while div * div <= n {

        if n % div == 0 {
            return false;
        }

        div += 2;
    }

    return true;
}


impl BigInt {

    pub fn is_prime(&self) -> bool {

        if self.is_negative {
            return false;
        }

        match self.to_u32() {
            Ok(n) => {
                is_prime_u32(n)
            }
            Err(_) => {

                if self % 2 == 0 {
                    return false;
                }

                let mut div: u32 = 3;

                while self >= &(div * div) && div < 0x8_000 {

                    if self % div == 0 {
                        return false;
                    }

                    div += 2;
                }

                let mut div = BigInt::from_u32(div);

                while &(&div * &div) <= self {

                    if self % &div == 0 {
                        return false;
                    }

                    div = &div + 2u32;
                }

                true
            }
        }

    }

}


#[cfg(test)]
mod tests {

    fn raw_gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    #[test]
    fn gcd_test() {
        use crate::big_int::{BigInt, funcs::gcd};

        for _ in 0..0x400 {
            let a = rand::random::<i32>();
            let b = rand::random::<i32>();

            if a == 0 || b == 0 {
                continue;
            }

            assert_eq!(gcd(BigInt::from_i32(a), BigInt::from_i32(b)), BigInt::from_i32(raw_gcd(a, b)));
        }
    }

}
