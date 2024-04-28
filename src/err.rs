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

    /// `std::num::TryFromIntError` is always converted to `ConversionError::TryFromIntError`.
    /// For example, if you try to convert a `BigInt` into a `u32`, it's first converted to `i64`, then to `u32`.
    /// If the latter one fails, Rust emits `std::num::TryFromIntError`, which hmath cannot understand.
    /// In this case hmath will throw this error.
    TryFromIntError,
}

impl From<TryFromIntError> for ConversionError {
    fn from(_: TryFromIntError) -> Self {
        ConversionError::TryFromIntError
    }
}
