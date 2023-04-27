#[derive(Clone, Debug, PartialEq)]
pub enum ConversionError {
    NoData,
    InvalidChar(char),
    NotInRange { permitted: String, error: String },

    UnexpectedEnd,

    // ieee754 numbers
    Infinity,
    NotANumber
}