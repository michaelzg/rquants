//! Internal macros for generating quantity boilerplate.

#[doc(hidden)]
#[macro_export]
macro_rules! quantity {
    (
        $(#[$quantity_meta:meta])*
        pub quantity $quantity:ident {
            unit: $unit:ident;
            dimension: $dimension:ident;
            conversions: $conversions:ident;
            name: $name:expr;
            primary: $primary:ident;
            si: $si_unit:ident;

            units {
                $(
                    $(#[$unit_meta:meta])*
                    $unit_variant:ident {
                        symbol: $symbol:expr,
                        factor: $factor:expr,
                        ctor: $ctor:ident,
                        to: $to:ident,
                        si: $is_si:expr
                    }
                ),+ $(,)?
            }
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $unit {
            $(
                $(#[$unit_meta])*
                $unit_variant,
            )+
        }

        impl $unit {
            pub const ALL: &'static [$unit] = &[
                $($unit::$unit_variant,)+
            ];

            pub fn symbol(&self) -> &'static str {
                match self {
                    $($unit::$unit_variant => $symbol,)+
                }
            }

            pub fn conversion_factor(&self) -> f64 {
                match self {
                    $($unit::$unit_variant => $factor,)+
                }
            }

            pub fn is_primary(&self) -> bool {
                matches!(self, $unit::$primary)
            }

            pub fn is_si(&self) -> bool {
                match self {
                    $($unit::$unit_variant => $is_si,)+
                }
            }
        }

        impl ::std::fmt::Display for $unit {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", $crate::core::UnitOfMeasure::symbol(self))
            }
        }

        impl $crate::core::UnitOfMeasure for $unit {
            fn symbol(&self) -> &'static str {
                $unit::symbol(self)
            }

            fn conversion_factor(&self) -> f64 {
                $unit::conversion_factor(self)
            }

            fn is_primary(&self) -> bool {
                $unit::is_primary(self)
            }

            fn is_si(&self) -> bool {
                $unit::is_si(self)
            }
        }

        $(#[$quantity_meta])*
        #[derive(Debug, Clone, Copy)]
        pub struct $quantity {
            value: f64,
            unit: $unit,
        }

        impl $quantity {
            /// Creates a new quantity with the given value and unit.
            pub const fn new_const(value: f64, unit: $unit) -> Self {
                Self { value, unit }
            }

            $(
                $(#[$unit_meta])*
                pub fn $ctor(value: f64) -> Self {
                    <Self as $crate::core::Quantity>::new(value, $unit::$unit_variant)
                }

                $(#[$unit_meta])*
                pub fn $to(&self) -> f64 {
                    $crate::core::Quantity::to(self, $unit::$unit_variant)
                }
            )+
        }

        impl ::std::fmt::Display for $quantity {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    "{} {}",
                    self.value,
                    $crate::core::UnitOfMeasure::symbol(&self.unit)
                )
            }
        }

        impl ::std::cmp::PartialEq for $quantity {
            fn eq(&self, other: &Self) -> bool {
                $crate::core::Quantity::to_primary(self)
                    == $crate::core::Quantity::to_primary(other)
            }
        }

        impl ::std::cmp::PartialOrd for $quantity {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                $crate::core::Quantity::to_primary(self)
                    .partial_cmp(&$crate::core::Quantity::to_primary(other))
            }
        }

        impl $crate::core::Quantity for $quantity {
            type Unit = $unit;

            fn new(value: f64, unit: Self::Unit) -> Self {
                Self { value, unit }
            }

            fn value(&self) -> f64 {
                self.value
            }

            fn unit(&self) -> Self::Unit {
                self.unit
            }
        }

        impl ::std::ops::Add for $quantity {
            type Output = $quantity;

            fn add(self, rhs: Self) -> Self::Output {
                let rhs_value = $crate::core::Quantity::to(&rhs, self.unit);
                <$quantity as $crate::core::Quantity>::new(self.value + rhs_value, self.unit)
            }
        }

        impl ::std::ops::Sub for $quantity {
            type Output = $quantity;

            fn sub(self, rhs: Self) -> Self::Output {
                let rhs_value = $crate::core::Quantity::to(&rhs, self.unit);
                <$quantity as $crate::core::Quantity>::new(self.value - rhs_value, self.unit)
            }
        }

        impl ::std::ops::Mul<f64> for $quantity {
            type Output = $quantity;

            fn mul(self, rhs: f64) -> Self::Output {
                <$quantity as $crate::core::Quantity>::new(self.value * rhs, self.unit)
            }
        }

        impl ::std::ops::Mul<$quantity> for f64 {
            type Output = $quantity;

            fn mul(self, rhs: $quantity) -> Self::Output {
                <$quantity as $crate::core::Quantity>::new(self * rhs.value, rhs.unit)
            }
        }

        impl ::std::ops::Div<f64> for $quantity {
            type Output = $quantity;

            fn div(self, rhs: f64) -> Self::Output {
                <$quantity as $crate::core::Quantity>::new(self.value / rhs, self.unit)
            }
        }

        impl ::std::ops::Div<$quantity> for $quantity {
            type Output = f64;

            fn div(self, rhs: $quantity) -> Self::Output {
                $crate::core::Quantity::to_primary(&self)
                    / $crate::core::Quantity::to_primary(&rhs)
            }
        }

        impl ::std::ops::Neg for $quantity {
            type Output = $quantity;

            fn neg(self) -> Self::Output {
                <$quantity as $crate::core::Quantity>::new(-self.value, self.unit)
            }
        }

        pub struct $dimension;

        impl $crate::core::Dimension for $dimension {
            type Quantity = $quantity;
            type Unit = $unit;

            fn name() -> &'static str {
                $name
            }

            fn primary_unit() -> Self::Unit {
                $unit::$primary
            }

            fn si_unit() -> Self::Unit {
                $unit::$si_unit
            }

            fn units() -> &'static [Self::Unit] {
                $unit::ALL
            }
        }

        pub trait $conversions {
            $(
                $(#[$unit_meta])*
                fn $ctor(self) -> $quantity;
            )+
        }

        impl $conversions for f64 {
            $(
                fn $ctor(self) -> $quantity {
                    $quantity::$ctor(self)
                }
            )+
        }
    };
}
