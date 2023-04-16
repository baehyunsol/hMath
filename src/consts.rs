// TODO: remove these constants and use shift operators instead
// eg: 3 * U64_32 -> 3 << 32
pub const U64_32: u64 = 1 << 32;

pub const U128_32: u128 = 1 << 32;
pub const U128_64: u128 = 1 << 64;

#[cfg(test)]
pub const RUN_ALL_TESTS: bool = false;