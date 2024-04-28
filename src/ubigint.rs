mod arith;
mod comp;
pub mod convert;
pub mod funcs;

// data is always in the most reduced form
// [0] is the least-significant number
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UBigInt(pub(crate) Vec<u32>);

impl UBigInt {

    /// size of its internal vector
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn zero() -> Self {
        UBigInt(vec![0])
    }

    #[inline]
    pub fn one() -> Self {
        UBigInt(vec![1])
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.len() == 1 && self.0[0] == 0
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        self.len() == 1 && self.0[0] == 1
    }

    #[cfg(test)]
    pub fn is_valid(&self) -> bool {
        if self.len() == 0 {
            println!("length 0 ubigint");
            return false;
        }

        if self.len() > 1 && self.0[self.len() - 1] == 0 {
            println!("{:?}", self.0);
            return false;
        }

        true
    }

    #[inline]
    /// `vec[0]` is the least significant number
    pub fn from_raw(vec: Vec<u32>) -> Self {
        UBigInt(vec)
    }

    #[inline]
    pub fn into_raw(self) -> Vec<u32> {
        self.0
    }
}

impl Default for UBigInt {
    fn default() -> Self { UBigInt::zero() }
}
