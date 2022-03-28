use crate::{BigInt, Ratio};
use std::cmp::{Eq, PartialOrd, Ord, Ordering};


impl PartialOrd for BigInt {

    fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}


impl Ord for BigInt {

    fn cmp(&self, other: &BigInt) -> Ordering {

        if self.is_negative != other.is_negative {

            if self.is_negative {
                Ordering::Less
            }

            else {
                Ordering::Greater
            }

        }

        else {

            if self.len() > other.len() {

                if self.is_negative {
                    Ordering::Less
                }

                else {
                    Ordering::Greater
                }

            }

            else if self.len() < other.len() {

                if self.is_negative {
                    Ordering::Greater
                }

                else {
                    Ordering::Less
                }

            }

            else {
                let length = self.len();

                for i in 1..self.len() + 1 {

                    if self.data[length - i] > other.data[length - i] {
                        return if self.is_negative {Ordering::Less} else {Ordering::Greater};
                    }

                    else if self.data[length - i] < other.data[length - i] {
                        return if self.is_negative {Ordering::Greater} else {Ordering::Less};
                    }

                }

                Ordering::Equal
            }

        }

    }

}


impl Eq for BigInt {}


impl PartialOrd<u32> for BigInt {

    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        Some((self - *other).cmp(&BigInt::zero()))
    }

}


impl PartialEq<u32> for BigInt {

    fn eq(&self, other: &u32) -> bool {

        if self.is_negative {
            false
        }

        else {
            (self - *other).is_zero()
        }

    }

}


impl PartialEq<i32> for BigInt {

    fn eq(&self, other: &i32) -> bool {
        (self - *other).is_zero()
    }

}


impl PartialOrd<Ratio> for BigInt {

    fn partial_cmp(&self, other: &Ratio) -> Option<Ordering> {
        Some((self - other).cmp(&Ratio::zero()))
    }

}


impl PartialEq<Ratio> for BigInt {

    fn eq(&self, other: &Ratio) -> bool {
        other.denom == 1u32 && &other.numer == self
    }

}


#[cfg(test)]
mod tests {

    #[test]
    fn comp_test() {
        use crate::big_int::{BigInt, BASE};

        let coeff = (BASE as i128 / 16).max(1);

        for i in -32..32 {
            let i: i128 = i * i * i * coeff;

            for j in -32..32 {
                let j: i128 = j * j * j * coeff;
                assert_eq!(i.cmp(&j), BigInt::from_i128(i).cmp(&BigInt::from_i128(j)));
            }

        }

    }

    #[test]
    fn partial_cmp_test() {
        use crate::big_int::BigInt;

        for _ in 0..0x400 {
            let n1 = rand::random::<u32>() % 512;
            let n2 = rand::random::<u32>() % 512;

            assert_eq!(n1.partial_cmp(&n2), BigInt::from_u32(n1).partial_cmp(&n2));
        }
    }

}