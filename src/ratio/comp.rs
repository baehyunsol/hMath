use crate::Ratio;
use std::cmp::Ordering;

impl Ratio {

    /// self < other
    pub fn lt_rat(&self, other: &Ratio) -> bool {
        todo!()
    }

    /// self > other
    pub fn gt_rat(&self, other: &Ratio) -> bool {
        todo!()
    }

    pub fn eq_rat(&self, other: &Ratio) -> bool {
        self == other
    }

    pub fn neq_rat(&self, other: &Ratio) -> bool {
        self != other
    }

    /// self <= other
    pub fn leq_rat(&self, other: &Ratio) -> bool {
        !self.gt_rat(other)
    }

    /// self >= other
    pub fn geq_rat(&self, other: &Ratio) -> bool {
        !self.lt_rat(other)
    }

    pub fn comp_rat(&self, other: &Ratio) -> Ordering {
        todo!()
    }

    /// self < 1
    pub fn lt_one(&self) -> bool {
        self.is_neg() || self.numer.lt_bi(&self.denom)
    }

    /// self > 1
    pub fn gt_one(&self) -> bool {
        !self.is_neg() && self.numer.gt_bi(&self.denom)
    }

}