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

            assert_eq!(gcd(BigInt::from_i32(a), BigInt::from_i32(b)), BigInt::from_i32(raw_gcd(a, b)));
        }
    }

}
