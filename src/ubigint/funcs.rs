use super::UBigInt;

mod exp;
pub mod log;
mod shift;

pub use log::log2_u32;

impl UBigInt {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sqrt(&self) -> Self {
        if self.len() < 3 {
            return UBigInt::from_u64(sqrt_u64(self.to_u64().unwrap()));
        }

        let approx = sqrt_u64(self.shift_right(self.len() - 2).to_u64().unwrap());

        // result = sqrt(self) - alpha, where alpha << sqrt(self)
        // self - result^2 = 2 * alpha * sqrt(self) - alpha^2 ~= 2 * alpha * sqrt(self)
        let mut result = UBigInt::from_u64(approx);

        if self.len() % 2 != 0 {
            result.mul_u32_mut(65536);
        }

        result.shift_left_mut(self.len() / 2 - 1);

        let mut div = self.sub(&result.mul(&result)).div_u32(2).div(&result);
        result.add_mut(&div);

        loop {
            let result_sqr = result.mul(&result);

            if self.geq(&result_sqr) {
                div = self.sub(&result_sqr).div_u32(2).div(&result);
                result.add_mut(&div);
            }

            else {
                div = result_sqr.sub(&self).div_u32(2).div(&result);
                result.sub_mut(&div);
            }

            if div.len() < 2 {
                break;
            }
        }

        // Safety: div.len() < 2;
        let mut div = div.to_u32().unwrap();

        if div == 0 {
            div = 2;
        }

        loop {
            while result.mul(&result).lt(self) {
                result.add_u32_mut(div);
            }

            div /= 4;

            // it guarantees that the result is always `truncate(sqrt(self))`, not `round(sqrt(self))`
            if div < 4 {
                div = 1;
            }

            while result.mul(&result).gt(self) {
                result.sub_u32_mut(div);
            }

            div /= 4;

            if div == 0 {
                return result;
            }
        }
    }

    pub fn factorial(n: u32) -> UBigInt {
        if n < 21 {
            let mut result: u64 = 1;

            for i in 2..(n as u64 + 1) {
                result *= i;
            }

            UBigInt::from_u64(result)
        }

        else if n < 129 {
            let mut result = UBigInt::from_raw(vec![2192834560, 566454140]);  // factorial(20)
            let mut int_buffer = 1;

            for i in 21..(n as u64 + 1) {
                int_buffer *= i as u32;

                if int_buffer > 0x1_000_000 {
                    result.mul_u32_mut(int_buffer);
                    int_buffer = 1;
                }
            }

            if int_buffer > 1 {
                result.mul_u32_mut(int_buffer);
            }

            result
        }

        else {
            let mut result = UBigInt::from_raw(vec![
                0, 0, 0, 2147483648, 1653232837,
                595720861, 948844160, 1991672462,
                2500910141, 2421394908, 1199558731,
                684006397, 4097118094, 3861115933,
                3624737256, 703871983, 1875727135,
                2498653150, 380736459, 2256750694,
                3845178240, 3753984225, 4581,
            ]);  // factorial(128)
            let mut buffer = UBigInt::one();

            for i in 129..(n as u64 + 1) {
                buffer.mul_u32_mut(i as u32);

                if buffer.len() > 6 {
                    result.mul_mut(&buffer);
                    buffer = UBigInt::one();
                }
            }

            result.mul_mut(&buffer);
            result
        }
    }

    pub fn fibonacci(n: u32) -> UBigInt {
        if n < 14 {
            UBigInt::from_u32([
                0, 1, 1, 2, 3, 5, 8, 13,
                21, 34, 55, 89, 144, 233,
            ][n as usize])
        }

        else if n < 187 {
            let mut last = 233;
            let mut llast = 144;
            let mut result = 0;

            for _ in 0..(n - 13) {
                result = last + llast;
                llast = last;
                last = result;
            }

            UBigInt::from_u128(result)
        }

        else {
            let mut last = UBigInt::from_raw(vec![858943736, 3366396219, 4196493967, 4200843481]);  // fibonacci(186)
            let mut llast = UBigInt::from_raw(vec![3883453837, 806826839, 1203823756, 2596264053]);  // fibonacci(185)
            let mut result = UBigInt::zero();

            for _ in 0..(n - 186) {
                result = last.add(&llast);
                llast = last;
                last = result.clone();
            }

            result
        }
    }

    pub fn is_prime(&self) -> bool {
        if self.0[0] % 2 == 0 {
            if self.0[0] == 2 && self.0.len() == 1 {
                true
            }

            // According to ChatGPT, 0 is not a prime number :)
            else {
                false
            }
        }

        else if self.is_one() {
            false
        }

        else {
            let iter_until = self.sqrt().add_u32(1);
            let mut div = 3;

            match iter_until.to_u32() {
                Ok(n) => {

                    // Safety: if (sqrt(self) + 1) is smaller than (2^32 - 1), self is smaller than (2^64 - 1)
                    let self_ = self.to_u64().unwrap();

                    while div < n {
                        if self_ % div as u64 == 0 {
                            return false;
                        }

                        div += 2;
                    }

                    true
                },
                _ => {
                    for _ in 0..(u32::MAX / 2 - 1) {

                        if self.rem_u32(div).is_zero() {
                            return false;
                        }

                        div += 2;
                    }

                    let mut div = u32::MAX as u64;
                    div = div + div % 2 + 1;

                    let mut div = UBigInt::from_u64(div);

                    // TODO: this loop is not tested
                    while div.mul(&div).leq(self) {
                        if self.rem(&div).is_zero() {
                            return false;
                        }

                        div.add_u32_mut(2);
                    }

                    true
                },
            }
        }
    }

    pub fn prime_factorial(&self) -> Vec<Self> {
        let mut self_clone = self.clone();
        let mut result = vec![];

        while self_clone.gt_u32(1) && self_clone.rem_pow2(2).is_zero() {
            self_clone.div_u32_mut(2);
            result.push(UBigInt::from_u32(2));
        }

        let mut div = 3;

        while self_clone.geq(&UBigInt::from_u64(div as u64 * div as u64)) {
            while self_clone.rem_u32(div).is_zero() {
                self_clone.div_u32_mut(div);
                result.push(UBigInt::from_u32(div));
            }

            div += 2;
        }

        let mut div = UBigInt::from_u32(u32::MAX);

        // TODO: this loop is not tested
        while self_clone.geq(&div.mul(&div)) {
            while self_clone.rem(&div).is_zero() {
                self_clone.div_mut(&div);
                result.push(div.clone());
            }

            div.add_u32_mut(2);
        }

        if self_clone.gt_u32(1) || result.len() == 0 {
            result.push(self_clone);
        }

        result
    }

    /// It returns a random number between 1..2^(32 * scale).\
    /// If `scale` is 0, it returns 0.
    #[cfg(feature = "rand")]
    pub fn random(scale: usize) -> Self {
        if scale == 0 {
            UBigInt::zero()
        }

        else {
            UBigInt::from_raw(
                (0..scale).map(|_| rand::random::<u32>().max(1)).collect()
            )
        }
    }
}

fn sqrt_u64(n: u64) -> u64 {
    if n == 0 { return 0; }

    let l2 = n.ilog2();

    let mut frac = if l2 > 28 {
        n >> (l2 - 28)
    }

    else {
        n << (28 - l2)
    };

    #[cfg(test)] assert!((1 << 28) <= frac && frac < (1 << 29));

    // n = 2^l2 * frac / 2^28
    // frac = 1.xxx
    // sqrt(1 + x) = 1 + x/2 - x^2/8
    frac -= 1 << 28;
    let mut s = (1 << 28) + frac / 2 - frac * frac / (1 << 31);  // sqrt(frac / 2^28) * 2^28

    if l2 % 2 == 1 {
        s *= 379625062;  // 2^28.5
        s /= 1 << 28;
    }

    let d = (1 << (l2 / 2)) * s;

    let mut result = d / (1 << 28);

    // see the comments in `UBigInt::sqrt`
    let mut div = (n.abs_diff(result * result) / (2 * result)).max(1);

    if n > result * result {
        result += div;
    }

    else {
        result -= div;
    }

    if result == 0 {
        div = 2;
    }

    else {
        div = n.abs_diff(result * result) / (2 * result);

        if div == 0 {
            div = 2;
        }
    }

    loop {
        while result * result < n {
            result += div;
        }

        div /= 4;

        if div < 4 { div = 1; }

        while result * result > n {
            result -= div;
        }

        div /= 4;

        if div == 0 {
            #[cfg(test)] assert!(result * result <= n && n <= (result + 1) * (result + 1));

            return result;
        }
    }
}

pub fn gcd_ubi(a: &UBigInt, b: &UBigInt) -> UBigInt {
    if a.is_zero() {
        return b.clone();
    }

    let _a = a.clone();
    let mut a = b.rem(a);
    let mut b = _a;

    while !a.is_zero() {
        let _a = a.clone();
        a = b.rem(&a);
        b = _a;
    }

    b
}

#[cfg(test)]
mod tests {
    use crate::UBigInt;
    use crate::consts::RUN_ALL_TESTS;
    use super::sqrt_u64;

    #[test]
    fn factorial_test() {
        if !RUN_ALL_TESTS { return; }

        let mut acc = UBigInt::one();

        for n in 2..256 {
            acc.mul_u32_mut(n);
            assert_eq!(UBigInt::factorial(n), acc);
        }
    }

    #[test]
    fn fibonacci_test() {
        if !RUN_ALL_TESTS { return; }

        let mut fibos = vec![
            UBigInt::zero(), UBigInt::one(), UBigInt::one()
        ];

        for i in 3..256 {
            fibos.push(
                fibos[i - 1].add(&fibos[i - 2])
            );
        }

        for i in 0..256 {
            assert_eq!(fibos[i], UBigInt::fibonacci(i as u32));
        }
    }

    #[test]
    fn sqrt_test() {
        assert_eq!(sqrt_u64(10), 3);
        assert_eq!(sqrt_u64(100), 10);
        assert_eq!(sqrt_u64(1000), 31);
        assert_eq!(sqrt_u64(10000), 100);
        assert_eq!(sqrt_u64(100000), 316);
        assert_eq!(sqrt_u64(1000000), 1000);
        assert_eq!(sqrt_u64(10000000), 3162);
        assert_eq!(sqrt_u64(100000000), 10000);
        assert_eq!(sqrt_u64(1000000000), 31622);
        assert_eq!(sqrt_u64(10000000000), 100000);
        assert_eq!(sqrt_u64(100000000000), 316227);
        assert_eq!(sqrt_u64(1000000000000), 1000000);

        if !RUN_ALL_TESTS { return; }

        for n in [
            "0", "1", "2", "3",
            "20", "300", "4000",
            "50000", "600000",
            "123456789",
        ] {
            let n = UBigInt::from_string(n).unwrap();
            let nn = n.mul(&n);

            assert_eq!(nn.sqrt(), n);
        }

        assert_eq!(
            UBigInt::from_string("1000").unwrap().sqrt(),
            UBigInt::from_string("31").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("1000_0000").unwrap().sqrt(),
            UBigInt::from_string("3162").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("1000_0000_0000").unwrap().sqrt(),
            UBigInt::from_string("316227").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("1000_0000_0000_0000").unwrap().sqrt(),
            UBigInt::from_string("31622776").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("1000_0000_0000_0000_0000").unwrap().sqrt(),
            UBigInt::from_string("3162277660").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("1000_0000_0000_0000_0000_0000").unwrap().sqrt(),
            UBigInt::from_string("316227766016").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("1000_0000_0000_0000_0000_0000_0000").unwrap().sqrt(),
            UBigInt::from_string("31622776601683").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("1000_0000_0000_0000_0000_0000_0000_0000").unwrap().sqrt(),
            UBigInt::from_string("3162277660168379").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("1000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap().sqrt(),
            UBigInt::from_string("316227766016837933").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("1000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap().sqrt(),
            UBigInt::from_string("31622776601683793319").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap().sqrt(),
            UBigInt::from_string("3162277660168379331998").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("400_0000_0000_0000_0000_0000_0000_0000_0000").unwrap().sqrt(),
            UBigInt::from_string("20_0000_0000_0000_0000").unwrap(),
        );
        assert_eq!(
            UBigInt::from_raw(vec![0, 0x10_000_000]).sqrt(),
            UBigInt::from_u32(2).pow_u32(30),
        );
        assert_eq!(
            UBigInt::from_raw(vec![0, 0x4_000_000]).sqrt(),
            UBigInt::from_u32(2).pow_u32(29),
        );
        assert_eq!(
            UBigInt::from_string("0x100").unwrap().sqrt(),
            UBigInt::from_string("0x10").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("0x101").unwrap().sqrt(),
            UBigInt::from_string("0x10").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("0xff").unwrap().sqrt(),
            UBigInt::from_string("0xf").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("0").unwrap().sqrt(),
            UBigInt::from_string("0").unwrap(),
        );
        assert_eq!(
            UBigInt::from_string("1").unwrap().sqrt(),
            UBigInt::from_string("1").unwrap(),
        );
    }

    #[test]
    fn prime_factorial_test() {
        if !RUN_ALL_TESTS { return; }

        for i in 1..100000 {
            prime_factorial_test_unit(&UBigInt::from_u32(i));
        }

        let pns = vec![
            2, 3, 5, 7, 11, 13, 17, 19,
            23, 29, 31, 37, 41, 43, 47,
            53, 59, 61, 67, 71, 73, 79,
            83, 89, 97, 101, 103, 107, 109,
        ];

        for pn in pns.into_iter() {
            assert!(UBigInt::from_u32(pn).is_prime());
        }
    }

    fn prime_factorial_test_unit(number: &UBigInt) {
        let result = number.prime_factorial();
        let mut answer = UBigInt::one();

        for pf in result.iter() {
            answer.mul_mut(pf);

            if !pf.is_prime() && !(pf.is_one() && number.is_one()) {
                panic!("{pf}, {number}, {result:?}");
            }
        }

        assert_eq!(&answer, number);

        if ((result.len() == 1) != number.is_prime()) && !number.is_one() {
            panic!("{number:?}, {result:?}");
        }
    }
}
