use crate::big_int::BigInt;
use std::ops::Rem;


impl Rem for &BigInt {
    type Output = BigInt;

    fn rem(self, other: &BigInt) -> BigInt {
        self - &(&(self / other) * other)
    }

}


impl Rem<u32> for &BigInt {
    type Output = BigInt;

    fn rem(self, other: u32) -> BigInt {
        self - &(&(self / other) * other)
    }

}
