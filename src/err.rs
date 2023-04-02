#[derive(Debug)]
pub enum ConversionError {
    NoData,
    InvalidChar(char),
    NotInRange
}