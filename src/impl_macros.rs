#[macro_export]
macro_rules! impl_from_for_ref {
    ($name: ty, $t: ty) => (
        impl From<&$t> for $name {
            fn from(n: &$t) -> Self {
                <$name>::from(*n)
            }
        }
    )
}

#[macro_export]
macro_rules! impl_tryfrom_for_ref {
    ($name: ty, $t: ty) => (
        impl TryFrom<&$t> for $name {
            type Error = ConversionError;

            fn try_from(n: &$t) -> Result<Self, ConversionError> {
                <$name>::try_from(*n)
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