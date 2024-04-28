use crate::Ratio;
use std::cmp::Ordering;

impl Ratio {

    /// self < other
    pub fn lt(&self, other: &Ratio) -> bool {
        // self - other < 0
        self.sub(other).is_neg()
    }

    /// self > other
    pub fn gt(&self, other: &Ratio) -> bool {
        // other - self < 0
        other.sub(self).is_neg()
    }

    /// self == other
    pub fn eq(&self, other: &Ratio) -> bool {
        self == other
    }

    /// self != other
    pub fn neq(&self, other: &Ratio) -> bool {
        self != other
    }

    /// self <= other
    pub fn leq(&self, other: &Ratio) -> bool {
        !self.gt(other)
    }

    /// self >= other
    pub fn geq(&self, other: &Ratio) -> bool {
        !self.lt(other)
    }

    pub fn comp(&self, other: &Ratio) -> Ordering {
        self.numer.mul(&other.denom).comp(&other.numer.mul(&self.denom))
    }

    /// self < 1
    pub fn lt_one(&self) -> bool {
        self.is_neg() || self.numer.lt(&self.denom)
    }

    /// self > 1
    pub fn gt_one(&self) -> bool {
        !self.is_neg() && self.numer.gt(&self.denom)
    }

    /// self < other
    pub fn lt_i32(&self, other: i32) -> bool {
        self.numer.lt(&self.denom.mul_i32(other))
    }

    /// self > other
    pub fn gt_i32(&self, other: i32) -> bool {
        self.numer.gt(&self.denom.mul_i32(other))
    }

    /// self == other
    pub fn eq_i32(&self, other: i32) -> bool {
        self.denom.is_one() && self.numer.eq_i32(other)
    }

    /// self != other
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
        self.numer.comp(&self.denom.mul_i32(other))
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Ratio) -> Option<Ordering> {
        Some(self.comp(other))
    }
}

impl Ord for Ratio {
    fn cmp(&self, other: &Ratio) -> Ordering {
        self.comp(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::Ratio;
    use std::cmp::Ordering;

    #[test]
    fn rat_comp_test() {
        for d1 in 1..6 {
            for d2 in 1..6 {
                for n1 in -7..7 {
                    for n2 in -7..7 {
                        comp_test_worker(d1, d2, n1, n2);
                    }
                }
            }
        }
    }

    fn comp_test_worker(d1: i32, d2: i32, n1: i32, n2: i32) {
        let a = Ratio::from_denom_and_numer_i32(d1, n1);
        let b = Ratio::from_denom_and_numer_i32(d2, n2);

        match a.comp(&b) {
            Ordering::Greater => {
                assert!(a.gt(&b));
                assert!(!a.eq(&b));
                assert!(!a.lt(&b));
            }
            Ordering::Equal => {
                assert!(!a.gt(&b));
                assert!(a.eq(&b));
                assert!(!a.lt(&b));
            }
            Ordering::Less => {
                assert!(!a.gt(&b));
                assert!(!a.eq(&b));
                assert!(a.lt(&b));
            }
        }

        match a.comp_i32(d2 * n2) {
            Ordering::Greater => {
                assert!(a.gt_i32(d2 * n2));
                assert!(!a.eq_i32(d2 * n2));
                assert!(!a.lt_i32(d2 * n2));
            }
            Ordering::Equal => {
                assert!(!a.gt_i32(d2 * n2));
                assert!(a.eq_i32(d2 * n2));
                assert!(!a.lt_i32(d2 * n2));
            }
            Ordering::Less => {
                assert!(!a.gt_i32(d2 * n2));
                assert!(!a.eq_i32(d2 * n2));
                assert!(a.lt_i32(d2 * n2));
            }
        }

        assert!(a.lt_one() && a.lt_i32(1) || a.gt_one() && a.gt_i32(1) || a.eq_i32(1));
    }
}
