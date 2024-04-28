use crate::UBigInt;

impl UBigInt {

    // self - self / other * other
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn rem(&self, other: &UBigInt) -> Self {
        let sdo = self.div(other).mul(other);

        #[cfg(test)] assert!(self.geq(&sdo));

        self.sub(&sdo)
    }

    pub fn rem_mut(&mut self, other: &UBigInt) {
        let sdo = self.div(other).mul(other);

        #[cfg(test)] assert!(self.geq(&sdo));

        self.sub_mut(&sdo);
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn rem_u32(&self, other: u32) -> Self {
        let other = other as u64;
        let unit = (1 << 32) % other;
        let mut curr_power = 1;
        let mut result: u64 = 0;

        for d in self.0.iter() {
            result += (*d as u64 % other) * curr_power % other;
            curr_power *= unit;
            curr_power %= other;
        }

        let result = UBigInt::from_u32((result % other) as u32);

        #[cfg(test)] {
            let t = self.rem(&UBigInt::from_u32(other as u32));
            assert_eq!(t, result);
            assert!(result.is_valid());
        }

        result
    }

    pub fn rem_u32_mut(&mut self, other: u32) {
        let unit = ((1 << 32) % other as u64) as u32;
        let mut curr_power = 1;
        let mut result = 0;

        for d in self.0.iter() {
            result += (d % other) * curr_power % other;
            curr_power *= unit;
            curr_power %= other;
        }

        *self = UBigInt::from_u32(result % other);

        #[cfg(test)] {
            let mut t = self.rem(&UBigInt::from_u32(other as u32));
            assert_eq!(&mut t, self);
            assert!(self.is_valid());
        }
    }

    /// `other` must be a power of 2
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn rem_pow2(&self, other: u32) -> Self {
        let result = UBigInt::from_u32(self.0[0] % other);

        #[cfg(test)] {
            assert!(is_pow2(other));
            assert_eq!(result, self.rem_u32(other));
        }

        result
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
