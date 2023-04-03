#[derive(Clone, Debug, PartialEq)]
pub enum ConversionError {
    NoData,
    InvalidChar(char),
    NotInRange,

    UnexpectedEnd,

    // ieee754 numbers
    Infinity,
    NotANumber
}