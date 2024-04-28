use crate::F192;
use std::cmp::Ordering;

impl PartialOrd for F192 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for F192 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_neg() != other.is_neg() {
            if self.is_neg() {
                Ordering::Less
            }

            else {
                Ordering::Greater
            }
        }

        else {
            match self.exp.cmp(&other.exp) {
                Ordering::Equal => match self.digits.cmp(&other.digits) {
                    Ordering::Equal => Ordering::Equal,
                    Ordering::Greater => if self.is_neg() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    },
                    Ordering::Less => if self.is_neg() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    },
                },
                Ordering::Greater => if self.is_neg() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                },
                Ordering::Less => if self.is_neg() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                },
            }
        }
    }
}
