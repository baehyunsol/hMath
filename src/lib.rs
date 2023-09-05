mod bigint;
mod consts;
mod err;
mod matrix;
mod poly;
mod ratio;
mod ubigint;
pub mod utils;

mod impl_macros;

pub use ubigint::{UBigInt, funcs::gcd_ubi};
pub use bigint::{BigInt, funcs::gcd_bi};
pub use ratio::{
    Ratio,
    funcs::exp_iter, funcs::ln_iter, funcs::pow_iter, funcs::log_iter,
    funcs::sqrt_iter, funcs::cbrt_iter,
    funcs::sin_iter, funcs::cos_iter, funcs::tan_iter,
    funcs::common_denom,
    e::e_iter, ln2::ln2_iter, pi::pi_iter,
    inspect_ieee754_f32, inspect_ieee754_f64
};
pub use matrix::{Matrix, MatrixError};
pub use poly::{
    Polynomial,
    from_points, from_points_generic,
    cubic_2_points, quadratic_3_points, linear_2_points,
};
pub use consts::{pi_const, e_const, ln2_const};
pub use err::ConversionError;