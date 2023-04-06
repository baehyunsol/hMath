use crate::UBigInt;
use crate::ubigint::funcs::log2_u32;

impl UBigInt {

    /// 0^0 is 1
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn pow_u32(&self, mut exp: u32) -> Self {

        if exp < 3 {

            if exp == 0 {
                return UBigInt::one();
            }

            else if exp == 1 {
                return self.clone();
            }

            else {
                return self.mul_ubi(self);
            }

        }

        let mut powers = Vec::with_capacity(log2_u32(exp) as usize);

        let mut curr_exp = 1;
        powers.push(self.clone());

        while curr_exp * 2 <= exp {
            powers.push(powers[powers.len() - 1].mul_ubi(&powers[powers.len() - 1]));
            curr_exp *= 2;
        }

        let mut result = powers.pop().unwrap();
        exp -= curr_exp;
        curr_exp /= 2;

        while curr_exp > 0 {
            let l = powers.pop().unwrap();

            if curr_exp <= exp {
                result.mul_ubi_mut(&l);
                exp -= curr_exp;
            }

            curr_exp /= 2;
        }

        #[cfg(test)] assert!(result.is_valid());

        result
    }

    pub fn pow_u32_mut(&mut self, exp: u32) {
        let result = self.pow_u32(exp);
        *self = result;
    }

    /// returns 2^exp
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn pow2(exp: u32) -> Self {
        UBigInt::from_raw(
            vec![
                vec![0; (exp / 32) as usize],
                vec![1 << (exp % 32)]
            ].concat()
        )
    }

}

#[cfg(test)]
mod tests {
    use crate::UBigInt;
    use crate::consts::RUN_ALL_TESTS;

    #[test]
    fn pow_test2() {

        if !RUN_ALL_TESTS {
            return;
        }

        for i in 33..46 {
            let a = UBigInt::from_u32(i).pow_u32(1225);
            assert_eq!(a, UBigInt::from_u32(i).pow_u32(35).pow_u32(35));
            assert_eq!(a, UBigInt::from_u32(i).pow_u32(5).pow_u32(245));
            assert_eq!(a, UBigInt::from_u32(i).pow_u32(245).pow_u32(5));
        }

    }

    #[test]
    fn pow2_test() {
        assert_eq!(UBigInt::from_u32(2).pow_u32(25), UBigInt::from_u32(33554432));

        for i in 0..256 {
            let p = UBigInt::pow2(i);
            assert!(p.is_valid());
            assert_eq!(p.log2().to_u32().unwrap(), i);
            assert_eq!(UBigInt::from_u32(2).pow_u32(i), p);
        }

    }

}