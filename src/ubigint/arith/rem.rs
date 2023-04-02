use crate::UBigInt;
use crate::consts::U32_OVER;

impl UBigInt {

    // self - self / other * other
    #[must_use]
    pub fn rem_ubi(&self, other: &UBigInt) -> Self {
        let mut sdo = self.div_ubi(other);
        sdo.mul_ubi_mut(other);

        self.sub_ubi(&sdo)
    }

    pub fn rem_ubi_mut(&mut self, other: &UBigInt) {
        let mut sdo = self.div_ubi(other);
        sdo.mul_ubi_mut(other);
        self.sub_ubi_mut(&sdo);
    }

    #[must_use]
    pub fn rem_u32(&self, other: u32) -> Self {
        let other = other as u64;
        let unit = U32_OVER % other;
        let mut curr_power = 1;
        let mut result: u64 = 0;

        for d in self.0.iter() {
            result += (*d as u64 % other) * curr_power % other;
            curr_power *= unit;
            curr_power %= other;
        }

        let result = UBigInt::from_u32((result % other) as u32);

        #[cfg(test)] {
            let t = self.rem_ubi(&UBigInt::from_u32(other as u32));
            assert_eq!(t, result);
            assert!(result.is_valid());
        }

        result
    }

    pub fn rem_u32_mut(&mut self, other: u32) {
        let unit = (U32_OVER % other as u64) as u32;
        let mut curr_power = 1;
        let mut result = 0;

        for d in self.0.iter() {
            result += (d % other) * curr_power % other;
            curr_power *= unit;
            curr_power %= other;
        }

        *self = UBigInt::from_u32(result % other);

        #[cfg(test)] {
            let mut t = self.rem_ubi(&UBigInt::from_u32(other as u32));
            assert_eq!(&mut t, self);
            assert!(self.is_valid());
        }
    }

    /// `other` must be a power of 2
    #[must_use]
    pub fn rem_pow2(&self, other: u32) -> Self {
        #[cfg(test)] assert!(is_pow2(other));

        UBigInt::from_u32(self.0[0] % other)
    }

}

#[cfg(test)]
fn is_pow2(mut n: u32) -> bool {

    while n > 4 {

        if n % 2 == 1 {
            return false;
        }

        n /= 2;
    }

    n == 1 || n == 2 || n == 4
}