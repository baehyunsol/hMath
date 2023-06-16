use std::num::TryFromIntError;

#[derive(Clone, Debug, PartialEq)]
pub enum ConversionError {
    NoData,
    InvalidChar(char),
    NotInRange { permitted: String, error: String },

    UnexpectedEnd,

    /// f32::INFINITY, f64::INFINITY
    Infinity,

    /// f32::NEG_INFINITY, f64::NEG_INFINITY
    NegInfinity,

    /// f32::NAN, f64::NAN
    NotANumber,

    /// `std::num::TryFromIntError` is always converted to `ConversionError::TryFromIntError`
    TryFromIntError
}

impl From<TryFromIntError> for ConversionError {

    fn from(_: TryFromIntError) -> Self {
        ConversionError::TryFromIntError
    }

}