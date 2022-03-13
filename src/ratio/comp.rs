use crate::{Ratio, BigInt};
use std::cmp::{Eq, PartialOrd, Ord, Ordering};


impl PartialOrd for Ratio {

    fn partial_cmp(&self, other: &Ratio) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}


impl Ord for Ratio {

    fn cmp(&self, other: &Ratio) -> Ordering {
        let sub = self - other;

        if sub.is_zero() {
            Ordering::Equal
        }

        else if sub.is_negative() {
            Ordering::Less
        }

        else {
            Ordering::Greater
        }

    }

}

impl Eq for Ratio {}


impl PartialOrd<BigInt> for Ratio {

    fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
        Some((self - other).cmp(&Ratio::zero()))
    }

}

impl PartialEq<BigInt> for Ratio {

    fn eq(&self, other: &BigInt) -> bool {
        self.denom == 1 && &self.numer == other
    }

}


impl PartialOrd<u32> for Ratio {

    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        Some((self - *other).cmp(&Ratio::zero()))
    }

}

impl PartialEq<u32> for Ratio {

    fn eq(&self, other: &u32) -> bool {
        self.denom == 1 && &self.numer == other
    }

}