use super::UBigInt;
use std::cmp::Ordering;

impl UBigInt {

    /// self < other
    pub fn lt(&self, other: &UBigInt) -> bool {
        if self.len() > other.len() {
            false
        }

        else if self.len() < other.len() {
            true
        }

        else {
            let self_len = self.len();

            for i in 1..(self_len + 1) {
                if self.0[self_len - i] < other.0[self_len - i] {
                    return true;
                }

                else if self.0[self_len - i] > other.0[self_len - i] {
                    return false;
                }
            }

            false  // equal
        }
    }

    /// self > other
    pub fn gt(&self, other: &UBigInt) -> bool {
        if self.len() > other.len() {
            true
        }

        else if self.len() < other.len() {
            false
        }

        else {
            let self_len = self.len();

            for i in 1..(self_len + 1) {
                if self.0[self_len - i] < other.0[self_len - i] {
                    return false;
                }

                else if self.0[self_len - i] > other.0[self_len - i] {
                    return true;
                }
            }

            false  // equal
        }
    }

    /// Though `PartialEq` is implemented for `UBigInt`, this method exists.
    /// That's for consistency.
    pub fn eq(&self, other: &UBigInt) -> bool {
        self == other
    }

    pub fn neq(&self, other: &UBigInt) -> bool {
        self != other
    }

    /// self <= other
    pub fn leq(&self, other: &UBigInt) -> bool {
        !self.gt(other)
    }

    /// self >= other
    pub fn geq(&self, other: &UBigInt) -> bool {
        !self.lt(other)
    }

    pub fn comp(&self, other: &UBigInt) -> Ordering {
        if self.len() > other.len() {
            Ordering::Greater
        }

        else if self.len() < other.len() {
            Ordering::Less
        }

        else {
            let self_len = self.len();

            for i in 1..(self_len + 1) {

                if self.0[self_len - i] > other.0[self_len - i] {
                    return Ordering::Greater;
                }

                else if self.0[self_len - i] < other.0[self_len - i] {
                    return Ordering::Less;
                }
            }

            Ordering::Equal
        }
    }

    /// self < other
    pub fn lt_u32(&self, other: u32) -> bool {
        if self.len() > 1 {
            false
        }

        else {
            self.0[0] < other
        }
    }

    /// self > other
    pub fn gt_u32(&self, other: u32) -> bool {
        if self.len() > 1 {
            true
        }

        else {
            self.0[0] > other
        }
    }

    pub fn eq_u32(&self, other: u32) -> bool {
        if self.len() > 1 {
            false
        }

        else {
            self.0[0] == other
        }
    }

    pub fn neq_u32(&self, other: u32) -> bool {
        !self.eq_u32(other)
    }

    /// self <= other
    pub fn leq_u32(&self, other: u32) -> bool {
        !self.gt_u32(other)
    }

    /// self >= other
    pub fn geq_u32(&self, other: u32) -> bool {
        !self.lt_u32(other)
    }

    pub fn comp_u32(&self, other: u32) -> Ordering {
        if self.len() > 1 {
            Ordering::Greater
        }

        else {
            self.0[0].cmp(&other)
        }
    }
}

impl PartialOrd for UBigInt {
    fn partial_cmp(&self, other: &UBigInt) -> Option<Ordering> {
        Some(self.comp(other))
    }
}

impl Ord for UBigInt {
    fn cmp(&self, other: &UBigInt) -> Ordering {
        self.comp(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::UBigInt;
    use std::cmp::Ordering;

    #[test]
    fn ubi_cmp_test() {
        let numbers = vec![
            UBigInt::zero(), UBigInt::one(), UBigInt::from_u32(2), UBigInt::from_u32(3),
            UBigInt::from_u32(u32::MAX),
            UBigInt::from_u64(u64::MAX),
            UBigInt::from_u128(u128::MAX),
            UBigInt::exp2(130),
            UBigInt::exp2(131),
            UBigInt::exp2(132),
            UBigInt::from_u32(19).pow_u32(23),
            UBigInt::from_u32(23).pow_u32(19),
            UBigInt::from_u32(100).pow_u32(99),
            UBigInt::from_u32(100).pow_u32(100),
            UBigInt::from_u32(100).pow_u32(101),
            UBigInt::from_u32(100).pow_u32(102),
            UBigInt::from_u32(100).pow_u32(103),
            UBigInt::from_u32(100).pow_u32(104),
            UBigInt::from_u32(100).pow_u32(105),
            UBigInt::from_u32(100).pow_u32(106),
        ];

        for i in 0..numbers.len() {
            for j in (i + 1)..numbers.len() {
                match numbers[i].comp(&numbers[j]) {
                    Ordering::Greater => {
                        assert!(numbers[i].gt(&numbers[j]));
                        assert!(!numbers[i].eq(&numbers[j]));
                        assert!(!numbers[i].lt(&numbers[j]));

                        if let Ok(n) = numbers[j].to_u32() {
                            assert!(numbers[i].gt_u32(n));
                        }
                    },
                    Ordering::Equal => panic!("No same numbers"),
                    Ordering::Less => {
                        assert!(!numbers[i].gt(&numbers[j]));
                        assert!(!numbers[i].eq(&numbers[j]));
                        assert!(numbers[i].lt(&numbers[j]));

                        if let Ok(n) = numbers[i].to_u32() {
                            assert!(numbers[j].gt_u32(n));
                        }
                    },
                }
            }
        }
    }
}
