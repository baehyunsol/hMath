mod utils;
mod bigint;
mod consts;
mod err;
mod ratio;
mod ubigint;

pub use ubigint::{UBigInt, funcs::gcd_ubi};
pub use bigint::{BigInt, funcs::gcd_bi};
pub use ratio::Ratio;

/*
TODO: impl pi iterator

pi = sigma{k = 0}{inf}{1/16^k * f(k)}
f(k) = (120k^2 + 151k + 47) / (512k^4 + 1024k^3 + 712k^2 + 194k + 15)

f(0) = 47 / 15
f(1) = 106 / 819
f(2) = 829 / 19635
f(3) = 316 / 15225
f(4) = 857 / 69597
f(5) = 3802 / 466785
f(6) = 5273 / 911547
f(7) = 776 / 179645
*/