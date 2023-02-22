use crate::Number;
use std::cmp::{Eq, PartialOrd, Ord, Ordering};

impl PartialOrd for Number {

    fn partial_cmp(&self, other: &Number) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}

impl Eq for Number {}

impl Ord for Number {

    fn cmp(&self, other: &Number) -> Ordering {
        match self {
            Number::Integer(i) => match other {
                Number::Integer(ii) => i.cmp(ii),
                Number::Ratio(rr) => i.partial_cmp(rr).unwrap()
            }
            Number::Ratio(r) => match other {
                Number::Integer(ii) => r.partial_cmp(ii).unwrap(),
                Number::Ratio(rr) => r.cmp(rr)
            }
        }
    }

}

impl PartialOrd<i32> for Number {

    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        Some((self - *other).cmp(&Number::zero()))
    }

}

impl PartialEq<i32> for Number {

    fn eq(&self, other: &i32) -> bool {
        match self {
            Number::Integer(i) => i == other,
            _ => false
        }
    }

}