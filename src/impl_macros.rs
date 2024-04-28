#[macro_export]
macro_rules! impl_from_for_ref {
    ($t: ty, $u: ty) => (
        impl From<&$u> for $t {
            fn from(n: &$u) -> Self {
                <$t>::from(*n)
            }
        }
    )
}

#[macro_export]
macro_rules! impl_tryfrom_for_ref {
    ($t: ty, $u: ty) => (
        impl TryFrom<&$u> for $t {
            type Error = ConversionError;

            fn try_from(n: &$u) -> Result<Self, ConversionError> {
                <$t>::try_from(*n)
            }
        }
    )
}

#[macro_export]
macro_rules! impl_trait_for_general {
    (From, $t: ty, $u: ty, $m2: ident) => (
        impl From<$t> for $u {
            fn from(n: $t) -> Self {
                <$u>::$m2(n.into())
            }
        }
    );
    (TryFrom, $t: ty, $u: ty, $m2: ident) => (
        impl TryFrom<$t> for $u {
            type Error = ConversionError;

            fn try_from(n: $t) -> Result<Self, Self::Error> {
                <$u>::$m2(n.into())
            }
        }
    );
}

#[macro_export]
macro_rules! impl_trivial_try_from {
    ($t: ty, $u: ty, $m: ident) => (
        impl TryFrom<$t> for $u {
            type Error = ConversionError;
        
            fn try_from(n: $t) -> Result<Self, Self::Error> {
                n.$m()
            }
        }
    );
    (Fallible, $t: ty, $u: ty, $m: ident) => (
        impl TryFrom<$t> for $u {
            type Error = ConversionError;

            fn try_from(n: $t) -> Result<Self, Self::Error> {
                Ok(n.$m()?.try_into()?)
            }
        }
    )
}
