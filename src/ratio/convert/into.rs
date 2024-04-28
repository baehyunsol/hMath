use crate::{
    BigInt,
    ConversionError,
    Ratio,
    UBigInt,
    impl_trivial_try_from,
};

macro_rules! impl_ref_for_ratios {
    ($t: ty) => (
        impl TryFrom<Ratio> for $t {
            type Error = ConversionError;

            fn try_from(n: Ratio) -> Result<Self, Self::Error> {
                (&n).try_into()
            }
        }
    );
    ($t: ty, $($u: ty), +) => (
        impl_ref_for_ratios!($t);
        impl_ref_for_ratios!($($u),+);
    )
}

macro_rules! impl_ratio_to_int {
    ($t: ty) => (
        impl TryFrom<&Ratio> for $t {
            type Error = ConversionError;

            fn try_from(n: &Ratio) -> Result<Self, Self::Error> {
                n.truncate_bi().try_into()
            }
        }
    );
    ($t: ty, $($u: ty), +) => (
        impl_ratio_to_int!($t);
        impl_ratio_to_int!($($u),+);
    )
}

impl_trivial_try_from!(&Ratio, f32, to_ieee754_f32);
impl_trivial_try_from!(&Ratio, f64, to_ieee754_f64);

impl_ratio_to_int!(u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, isize, i128);

impl_ref_for_ratios!(
    u8, u16, u32, u64, usize, u128,
    i8, i16, i32, i64, isize, i128,
    f32, f64, UBigInt
);

impl From<Ratio> for BigInt {
    fn from(n: Ratio) -> Self {
        n.truncate_bi()
    }
}

impl<D, N> From<&Ratio> for (D, N) where BigInt: Into<D> + Into<N> {
    /// It destructures its denom and numer to types that can be constructed from `BigInt`.
    /// It's a bit expensive because it clones the numbers.
    fn from(n: &Ratio) -> (D, N) {
        let denom = n.get_denom();
        let numer = n.get_numer();

        (denom.into(), numer.into())
    }
}

impl<D, N> From<Ratio> for (D, N) where BigInt: Into<D> + Into<N> {
    /// It destructures its denom and numer to types that can be constructed from `BigInt`.
    /// It's a bit expensive because it clones the numbers.
    fn from(n: Ratio) -> (D, N) {
        (&n).into()
    }
}
