use super::UBigInt;

impl UBigInt {

    /// It returns 0 when `self` is 0.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2(&self) -> Self {
        // It assumes that `self` is less than 2^(2^64)
        UBigInt::from_u64((self.len() as u64 - 1) * 32 + log2_u32(self.0[self.len() - 1]) as u64)
    }

    /// It returns `truncate(log2(self) * 16777216)`.
    /// Warning: This function is very expensive.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2_accurate(&self) -> Self {
        let mut result = UBigInt::zero();
        let mut self_clone = if self.len() > 3 {
            result.add_ubi_mut(&UBigInt::from_u64((self.len() - 3) as u64).mul_u32(32));
            self.shift_right(self.len() - 3)
        } else {
            self.clone()
        };

        // self = self^256
        for _ in 0..8 {
            self_clone = self_clone.mul_ubi(&self_clone);
        }

        result.mul_u32_mut(256);

        for _ in 0..2 {

            if self_clone.len() > 3 {
                result.add_ubi_mut(&UBigInt::from_u64((self_clone.len() - 3) as u64).mul_u32(32));
                self_clone.shift_right_mut(self_clone.len() - 3);
            }

            // self = self^256
            for _ in 0..8 {
                self_clone = self_clone.mul_ubi(&self_clone);
            }

            result.mul_u32_mut(256);
        }

        result.add_ubi_mut(&self_clone.log2());
        result
    }

    /// divide by 2^32
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn shift_right(&self, n: usize) -> Self {
        UBigInt::from_raw(self.0[n..].to_vec())
    }

    /// divide by 2^32
    pub fn shift_right_mut(&mut self, n: usize) {
        self.0 = self.0[n..].to_vec();
    }

    /// multiply 2^32
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn shift_left(&self, n: usize) -> Self {
        UBigInt::from_raw(vec![
            vec![0; n],
            self.0.clone()
        ].concat())
    }

    /// multiply 2^32
    pub fn shift_left_mut(&mut self, n: usize) {
        self.0 = vec![
            vec![0; n],
            self.0.clone()
        ].concat();
    }

    /// modulo 2^32
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn slice_right(&self, n: usize) -> Self {
        UBigInt::from_raw(self.0[0..n].to_vec())
    }

    /// modulo 2^32
    pub fn slice_right_mut(&mut self, n: usize) {
        self.0 = self.0[0..n].to_vec();
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sqrt(&self) -> Self {
        let mut div = if self.len() > 2 {
            self.shift_right(1)
        } else {
            UBigInt::from_u32(1 << 30)
        };
        let mut result = UBigInt::zero();

        loop {

            while result.mul_ubi(&result).lt_ubi(self) {
                result.add_ubi_mut(&div);
            }

            div.div_u32_mut(4);

            if div.lt_u32(4) {
                div = UBigInt::one();
            }

            while result.mul_ubi(&result).gt_ubi(self) {
                result.sub_ubi_mut(&div);
            }

            div.div_u32_mut(4);

            if div.is_zero() {
                break;
            }

        }

        result
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
                3845178240, 3753984225, 4581
            ]);  // factorial(128)
            let mut buffer = UBigInt::one();

            for i in 129..(n as u64 + 1) {
                buffer.mul_u32_mut(i as u32);

                if buffer.len() > 3 {
                    result.mul_ubi_mut(&buffer);
                    buffer = UBigInt::one();
                }
            }

            result.mul_ubi_mut(&buffer);
            result
        }

    }

    pub fn fibonacci(n: u32) -> UBigInt {

        if n < 14 {
            UBigInt::from_u32([
                0, 1, 1, 2, 3, 5, 8, 13,
                21, 34, 55, 89, 144, 233
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
                result = last.add_ubi(&llast);
                llast = last;
                last = result.clone();
            }

            result
        }

    }

}

pub fn gcd_ubi(a: &UBigInt, b: &UBigInt) -> UBigInt {

    if a.is_zero() {
        return b.clone();
    }

    let _a = a.clone();
    let mut a = b.rem_ubi(a);
    let mut b = _a;

    while !a.is_zero() {
        let _a = a.clone();
        a = b.rem_ubi(&a);
        b = _a;
    }

    b
}

// floor(log2(n))
pub fn log2_u32(mut n: u32) -> u32 {
    let mut result = 0;

    while n > 1024 {
        n /= 1024;
        result += 10;
    }

    while n > 32 {
        n /= 32;
        result += 5;
    }

    while n > 1 {
        n /= 2;
        result += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::UBigInt;
    use crate::consts::RUN_ALL_TESTS;

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
                fibos[i - 1].add_ubi(&fibos[i - 2])
            );
        }

        for i in 0..256 {
            assert_eq!(fibos[i], UBigInt::fibonacci(i as u32));
        }

    }

    #[test]
    fn sqrt_test() {
        if !RUN_ALL_TESTS { return; }
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
    fn log_test() {

        if !RUN_ALL_TESTS { return; }

        let mut n = UBigInt::from_u32(2);
        let mut i = 1;

        for _ in 0..256 {
            assert_eq!(UBigInt::from_u32(i), n.log2());
            assert_eq!(UBigInt::from_u32(i), n.add_u32(1).log2());
            assert_eq!(UBigInt::from_u32(i - 1), n.sub_u32(1).log2());
            n.mul_u32_mut(2);
            i += 1;
        }

        use crate::{Ratio, BigInt};
        let denom = BigInt::from_i32(16777216);

        assert_eq!(
            Ratio::from_denom_and_numer(
                denom.clone(),
                BigInt::from_i32(3).log2_accurate()
            ).to_approx_string(6),
            "1.5849"
        );
        assert_eq!(
            Ratio::from_denom_and_numer(
                denom.clone(),
                BigInt::from_i32(9900).log2_accurate()
            ).to_approx_string(6),
            "13.273"
        );
    }
}