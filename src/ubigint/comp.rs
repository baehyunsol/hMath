use super::UBigInt;
use std::cmp::Ordering;

impl UBigInt {

    /// self < other
    pub fn lt_ubi(&self, other: &UBigInt) -> bool {

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
    pub fn gt_ubi(&self, other: &UBigInt) -> bool {

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
    pub fn eq_ubi(&self, other: &UBigInt) -> bool {
        self == other
    }

    pub fn neq_ubi(&self, other: &UBigInt) -> bool {
        self != other
    }

    /// self <= other
    pub fn leq_ubi(&self, other: &UBigInt) -> bool {
        !self.gt_ubi(other)
    }

    /// self >= other
    pub fn geq_ubi(&self, other: &UBigInt) -> bool {
        !self.lt_ubi(other)
    }

    pub fn comp_ubi(&self, other: &UBigInt) -> Ordering {

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

// TODO: more tests