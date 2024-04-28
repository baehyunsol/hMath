mod arith;
mod comp;
mod consts;
mod convert;
mod funcs;

#[cfg(test)]
mod testbench;

/// `F192` is a type that represent real numbers. It's less accurate than `Ratio`, but more performant.
/// You can think of it as a mid-point between f64 and `Ratio`.
///
/// If there's an overflow, it would either panic or return an arbitrary result. Don't worry though, f192 covers very wide range.
// N = D * 2^E  (2^127 <= D < 2^128)
// D = 0, E = 0 when N = 0
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct F192 {
    // first bit is sign bit (1 if neg)
    // D = SIGN_MASK | digits  // 2^127 + (digits % 2^127)
    digits: u128,

    // E = exp - EXP_COEFF
    exp: u64,
}

pub(crate) const SIGN_MASK: u128 = 1 << 127;
pub(crate) const EXP_COEFF: u64 = 1 << 48;

impl F192 {
    #[inline]
    const fn is_zero(&self) -> bool {
        self.exp == 0 && self.digits == 0
    }

    #[inline]
    const fn is_neg(&self) -> bool {
        self.digits >> 127 == 1
    }

    /// It's pointless if the number is very big (eg: greater than 2^127).
    pub fn is_integer(&self) -> bool {
        if self.is_zero() {
            true
        }

        else {
            let e = self.exp as i64 - EXP_COEFF as i64;

            if e < -127 {
                false
            }

            else if e >= 0 {
                true
            }

            else {
                let e = -e as u32;

                ((self.digits >> e) << e) == self.digits
            }
        }
    }

    pub fn abs(&self) -> Self {
        F192 {
            digits: self.digits & !SIGN_MASK,
            ..*self
        }
    }

    pub fn neg(&self) -> Self {
        if self.is_zero() {
            F192::ZERO
        }

        else {
            F192 {
                digits: self.digits ^ SIGN_MASK,
                ..*self
            }
        }
    }

    pub fn shl(&self, other: u64) -> Self {
        if self.is_zero() {
            F192::ZERO
        }

        else {
            F192 {
                exp: self.exp + other,
                ..*self
            }
        }
    }

    pub fn shr(&self, other: u64) -> Self {
        if self.exp < other || self.is_zero() {
            F192::ZERO
        }

        else {
            F192 {
                exp: self.exp - other,
                ..*self
            }
        }
    }

    pub fn step_epsilon(&self) -> Self {
        if self.digits & !SIGN_MASK == !SIGN_MASK {
            F192 {
                digits: self.digits & SIGN_MASK,
                exp: self.exp + 1,
            }
        }

        else {
            F192 {
                digits: self.digits + 1,
                exp: self.exp,
            }
        }
    }
}

impl Default for F192 {
    fn default() -> Self {
        F192::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::testbench::{assert_f64_close, assert_very_close};
    use crate::Ratio;

    #[test]
    fn float_div_test() {
        let a: F192 = (3.125).try_into().unwrap();
        assert_eq!(a, F192::from(25).div(&8.into()));
        assert_eq!(a, F192::from(25).div_i32(8));

        let a: F192 = (2.5).try_into().unwrap();
        assert_eq!(a, F192::from(5).div(&2.into()));
        assert_eq!(a, F192::from(5).div_i32(2));
    }

    #[test]
    fn some_random_exprs() {
        assert!(F192::MIN < F192::MAX);
        assert_eq!(F192::ZERO, F192::MAX.add(&F192::MIN));
        assert_eq!(F192::ONE, F192::MAX.div(&F192::MAX));
        assert_eq!(F192::ONE, F192::MIN.div(&F192::MIN));
    }

    fn general_test(start: i128, end: i128) {
        for a in start..end {
            let a_f192: F192 = a.into();

            assert_eq!(a_f192.neg(), (-a).into());
            assert_eq!(a_f192.abs(), (a.abs()).into());
            assert_eq!(a, a_f192.try_into().unwrap());
            assert_eq!(a_f192, F192::ONE.mul(&a_f192));
            assert!(a_f192.is_integer());

            for i in 0..5 {
                assert_eq!(a_f192.shl(i), (a << i).into());

                if (a >> i) << i == a {
                    assert_eq!(a_f192.shr(i), (a >> i).into());
                }
            }

            let a_f32 = a as f32;
            let a_f64 = a as f64;
            assert_eq!(a_f192, a_f32.try_into().unwrap());
            assert_eq!(a_f192, a_f64.try_into().unwrap());

            let rat1: Ratio = a.into();
            let rat2: Ratio = a_f192.into();
            assert_eq!(rat1, rat2);
            assert_eq!(a_f192, rat2.into());

            if a > 0 {
                assert_eq!(a.ilog2(), a_f192.ilog2().try_into().unwrap());

                if a < 48 {
                    assert_eq!(F192::from_pow2(a as i64), F192::from(1u64 << a));
                }
            }

            let code13 = F192::from_code13(&a_f192.into_code13().unwrap()).unwrap();
            assert_eq!(code13, a_f192);

            assert_eq!(a_f192.to_string(), a.to_string());

            for b in (-512..512).chain(((1 << 96) - 2)..((1 << 96) + 4)).chain(-((1 << 96) + 2)..-((1 << 96) - 4)) {
                // println!("a: {a}, b: {b}");
                let b_f192 = b.into();
                let c_f192 = a_f192.mul(&b_f192);
                let d_f192 = a_f192.add(&b_f192);
                let e_f192 = a_f192.sub(&b_f192);

                assert_eq!(c_f192, (a * b).into());
                assert_eq!(d_f192, (a + b).into());
                assert_eq!(e_f192, (a - b).into());
                assert_eq!(a_f192.cmp(&b_f192), a.cmp(&b));

                if let Ok(b) = i32::try_from(b) {
                    assert_eq!(c_f192, a_f192.mul_i32(b));
                }

                if b != 0 {
                    if a % b == 0 {
                        let f_f192 = a_f192.div(&b_f192);
                        assert_eq!(f_f192, (a / b).into());
                    }

                    else {
                        let f_f192 = a_f192.div(&b_f192);
                        assert!(!f_f192.is_integer());
                    }

                    if let Ok(b) = i32::try_from(b) {
                        assert_very_close(a_f192.div_i32(b), a_f192.div(&b_f192));
                    }
                }
            }
        }
    }

    macro_rules! test_runner {
        ($name: ident, $start: expr, $end: expr) => {
            #[test]
            fn $name() {
                general_test($start, $end);
            }
        }
    }

    // for parallel execution
    test_runner!(general_test_1, -512, -480);
    test_runner!(general_test_2, -480, -448);
    test_runner!(general_test_3, -448, -416);
    test_runner!(general_test_4, -416, -384);
    test_runner!(general_test_5, -384, -352);
    test_runner!(general_test_6, -352, -320);
    test_runner!(general_test_7, -320, -288);
    test_runner!(general_test_8, -288, -256);
    test_runner!(general_test_9, -256, -224);
    test_runner!(general_test_a, -224, -192);
    test_runner!(general_test_b, -192, -160);
    test_runner!(general_test_c, -160, -128);
    test_runner!(general_test_d, -128, -96);
    test_runner!(general_test_e, -96, -64);
    test_runner!(general_test_f, -64, -32);
    test_runner!(general_test_g, -32, 0);
    test_runner!(general_test_h, 0, 32);
    test_runner!(general_test_i, 32, 64);
    test_runner!(general_test_j, 64, 96);
    test_runner!(general_test_k, 96, 128);
    test_runner!(general_test_l, 128, 160);
    test_runner!(general_test_m, 160, 192);
    test_runner!(general_test_n, 192, 224);
    test_runner!(general_test_o, 224, 256);
    test_runner!(general_test_p, 256, 288);
    test_runner!(general_test_q, 288, 320);
    test_runner!(general_test_r, 320, 352);
    test_runner!(general_test_s, 352, 384);
    test_runner!(general_test_t, 384, 416);
    test_runner!(general_test_u, 416, 448);
    test_runner!(general_test_v, 448, 480);
    test_runner!(general_test_w, 480, 512);
}
