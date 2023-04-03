#[derive(Debug)]
pub enum ConversionError {
    NoData,
    InvalidChar(char),
    NotInRange,

    // ieee754 numbers
    Infinity,
    NotANumber
}