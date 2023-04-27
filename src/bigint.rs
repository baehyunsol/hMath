use crate::UBigInt;

mod arith;
mod comp;
mod convert;
pub mod funcs;

// 0 is not negative
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BigInt {
    val: UBigInt,
    _is_neg: bool
}

impl BigInt {

    /// size of its internal vector
    #[inline]
    pub fn len(&self) -> usize {
        self.val.len()
    }

    #[inline]
    pub fn zero() -> Self {
        BigInt { val: UBigInt::zero(), _is_neg: false }
    }

    #[inline]
    pub fn one() -> Self {
        BigInt { val: UBigInt::one(), _is_neg: false }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.val.is_zero()
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        self.val.is_one() && !self.is_neg()
    }

    #[inline]
    pub fn is_neg(&self) -> bool {
        self._is_neg
    }

    #[inline]
    /// `vec` is that of `UBigInt::from_raw`
    pub fn from_raw(vec: Vec<u32>, is_neg: bool) -> Self {
        BigInt { val: UBigInt::from_raw(vec), _is_neg: is_neg }
    }

    #[inline]
    pub fn into_raw(self) -> (Vec<u32>, bool) {
        (self.val.0, self._is_neg)
    }

    #[cfg(test)]
    pub fn is_valid(&self) -> bool {
        self.val.is_valid() && (!self.val.is_zero() || !self.is_neg())
    }

}