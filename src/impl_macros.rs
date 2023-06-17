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
macro_rules! impl_ops_for_refs {
    (deref_rhs, $trait: ident, $rhs: ty, $lhs: ty, $trait_func: ident, $impl_func: ident) => (
        impl $trait<$rhs> for $lhs {
            type Output = Self;

            fn $trait_func(self, other: $rhs) -> Self::Output {
                self.$impl_func(other)
            }

        }
        impl $trait<&$rhs> for $lhs {
            type Output = Self;

            fn $trait_func(self, other: &$rhs) -> Self::Output {
                self.$impl_func(*other)
            }

        }
        impl $trait<$rhs> for &$lhs {
            type Output = $lhs;

            fn $trait_func(self, other: $rhs) -> Self::Output {
                self.$impl_func(other)
            }

        }
        impl $trait<&$rhs> for &$lhs {
            type Output = $lhs;

            fn $trait_func(self, other: &$rhs) -> Self::Output {
                self.$impl_func(*other)
            }

        }
    );
    (ref_rhs, $trait: ident, $rhs: ty, $lhs: ty, $trait_func: ident, $impl_func: ident) => (
        impl $trait<$rhs> for $lhs {
            type Output = Self;

            fn $trait_func(self, other: $rhs) -> Self::Output {
                self.$impl_func(&other)
            }

        }
        impl $trait<&$rhs> for $lhs {
            type Output = Self;

            fn $trait_func(self, other: &$rhs) -> Self::Output {
                self.$impl_func(other)
            }

        }
        impl $trait<$rhs> for &$lhs {
            type Output = $lhs;

            fn $trait_func(self, other: $rhs) -> Self::Output {
                self.$impl_func(&other)
            }

        }
        impl $trait<&$rhs> for &$lhs {
            type Output = $lhs;

            fn $trait_func(self, other: &$rhs) -> Self::Output {
                self.$impl_func(other)
            }

        }
    )
}

#[macro_export]
macro_rules! impl_ops_general {
    ($trait: ident, $rhs: ty, $lhs: ty, $output: ty, $trait_func: ident, $impl_func: ident) => (
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            fn $trait_func(self, other: $rhs) -> Self::Output {
                <$lhs as TryInto<$output>>::try_into(self).unwrap().$impl_func(&other.try_into().unwrap())
            }
        }
    )
}