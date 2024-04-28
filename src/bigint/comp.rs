use crate::BigInt;
use std::cmp::Ordering;

impl BigInt {

    /// self < other
    pub fn lt_bi(&self, other: &BigInt) -> bool {
        if self.is_neg() != other.is_neg() {
            self.is_neg()
        }

        else {
            match self.val.comp_ubi(&other.val) {
                Ordering::Equal => false,

                // -3 > -4
                Ordering::Less if self.is_neg() => false,

                // 3 < 4
                Ordering::Less => true,

                // -4 < -3
                Ordering::Greater if self.is_neg() => true,

                // 4 > 3
                Ordering::Greater => false,
            }
        }
    }

    /// self > other
    pub fn gt_bi(&self, other: &BigInt) -> bool {
        if self.is_neg() != other.is_neg() {
            other.is_neg()
        }

        else {
            match self.val.comp_ubi(&other.val) {
                Ordering::Equal => false,

                // -3 > -4
                Ordering::Less if self.is_neg() => true,

                // 3 < 4
                Ordering::Less => false,

                // -4 < -3
                Ordering::Greater if self.is_neg() => false,

                // 4 > 3
                Ordering::Greater => true,
            }
        }
    }

    /// Though `PartialEq` is implemented for `BigInt`, this method exists.
    /// That's for consistency.
    pub fn eq_bi(&self, other: &BigInt) -> bool {
        self == other
    }

    pub fn neq_bi(&self, other: &BigInt) -> bool {
        self != other
    }

    /// self <= other
    pub fn leq_bi(&self, other: &BigInt) -> bool {
        !self.gt_bi(other)
    }

    /// self >= other
    pub fn geq_bi(&self, other: &BigInt) -> bool {
        !self.lt_bi(other)
    }

    pub fn comp_bi(&self, other: &BigInt) -> Ordering {
        if self.is_neg() != other.is_neg() {
            if self.is_neg() {
                Ordering::Less
            }

            else {
                Ordering::Greater
            }
        }

        else {
            match self.val.comp_ubi(&other.val) {
                Ordering::Equal => Ordering::Equal,
                Ordering::Less if self.is_neg() => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Greater if self.is_neg() => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
            }
        }
    }

    /// self < other
    pub fn lt_i32(&self, other: i32) -> bool {
        if self.is_neg() != (other < 0) {
            self.is_neg()
        }

        else {
            match self.val.comp_u32(other.abs() as u32) {
                Ordering::Equal => false,

                // -3 > -4
                Ordering::Less if self.is_neg() => false,

                // 3 < 4
                Ordering::Less => true,

                // -4 < -3
                Ordering::Greater if self.is_neg() => true,

                // 4 > 3
                Ordering::Greater => false,
            }
        }
    }

    /// self > other
    pub fn gt_i32(&self, other: i32) -> bool {
        if self.is_neg() != (other < 0) {
            other < 0
        }

        else {
            match self.val.comp_u32(other.abs() as u32) {
                Ordering::Equal => false,

                // -3 > -4
                Ordering::Less if self.is_neg() => true,

                // 3 < 4
                Ordering::Less => false,

                // -4 < -3
                Ordering::Greater if self.is_neg() => false,

                // 4 > 3
                Ordering::Greater => true,
            }
        }
    }

    pub fn eq_i32(&self, other: i32) -> bool {
        if self.is_neg() != (other < 0) {
            false
        }

        else {
            self.val.to_u32().unwrap() == other.abs() as u32
        }
    }

    pub fn neq_i32(&self, other: i32) -> bool {
        !self.eq_i32(other)
    }

    /// self <= other
    pub fn leq_i32(&self, other: i32) -> bool {
        !self.gt_i32(other)
    }

    /// self >= other
    pub fn geq_i32(&self, other: i32) -> bool {
        !self.lt_i32(other)
    }

    pub fn comp_i32(&self, other: i32) -> Ordering {
        if self.is_neg() != (other < 0) {
            if self.is_neg() {
                Ordering::Less
            }

            else {
                Ordering::Greater
            }
        }

        else {
            match self.val.comp_u32(other.abs() as u32) {
                Ordering::Equal => Ordering::Equal,
                Ordering::Less if self.is_neg() => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Greater if self.is_neg() => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
            }
        }
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
        Some(self.comp_bi(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &BigInt) -> Ordering {
        self.comp_bi(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::BigInt;
    use std::cmp::Ordering;

    fn bi_comp(bi1: &BigInt, bi2: &BigInt) {
        match bi1.comp_bi(bi2) {
            Ordering::Equal => {
                assert!(!bi1.lt_bi(bi2));
                assert!(!bi1.gt_bi(bi2));
                assert!(bi1.eq_bi(bi2));
                assert!(bi1.leq_bi(bi2));
                assert!(bi1.geq_bi(bi2));
                assert!(!bi1.neq_bi(bi2));
            },
            Ordering::Less => {
                assert!(bi1.lt_bi(bi2));
                assert!(!bi1.gt_bi(bi2));
                assert!(!bi1.eq_bi(bi2));
                assert!(bi1.leq_bi(bi2));
                assert!(!bi1.geq_bi(bi2));
                assert!(bi1.neq_bi(bi2));
            },
            Ordering::Greater => {
                assert!(!bi1.lt_bi(bi2));
                assert!(bi1.gt_bi(bi2));
                assert!(!bi1.eq_bi(bi2));
                assert!(!bi1.leq_bi(bi2));
                assert!(bi1.geq_bi(bi2));
                assert!(bi1.neq_bi(bi2));
            },
        }
    }

    fn bi_i32_comp(bi: &BigInt, n: i32) {
        match bi.comp_i32(n) {
            Ordering::Equal => {
                assert!(!bi.lt_i32(n));
                assert!(!bi.gt_i32(n));
                assert!(bi.eq_i32(n));
                assert!(bi.leq_i32(n));
                assert!(bi.geq_i32(n));
                assert!(!bi.neq_i32(n));
            },
            Ordering::Less => {
                assert!(bi.lt_i32(n));
                assert!(!bi.gt_i32(n));
                assert!(!bi.eq_i32(n));
                assert!(bi.leq_i32(n));
                assert!(!bi.geq_i32(n));
                assert!(bi.neq_i32(n));
            },
            Ordering::Greater => {
                assert!(!bi.lt_i32(n));
                assert!(bi.gt_i32(n));
                assert!(!bi.eq_i32(n));
                assert!(!bi.leq_i32(n));
                assert!(bi.geq_i32(n));
                assert!(bi.neq_i32(n));
            },
        }
    }

    #[test]
    fn bi_comp_test() {
        for x in -7..8 {
            for y in -7..8 {
                let bix = BigInt::from_i32(x);
                let biy = BigInt::from_i32(y);
                assert_eq!(x.cmp(&y), bix.comp_bi(&biy));
                bi_comp(&bix, &biy);
                bi_i32_comp(&bix, y);
            }
        }
    }
}
