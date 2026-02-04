//! Macros to reduce boilerplate in quantity implementations.
//!
//! These macros generate the repetitive trait implementations that are
//! identical across all standard quantity types.

/// Implements `Display` for a unit enum by delegating to `symbol()`.
///
/// Every unit type displays as its symbol, so this eliminates the
/// identical `Display` impl repeated across all unit types.
macro_rules! impl_unit_display {
    ($unit_type:ty) => {
        impl std::fmt::Display for $unit_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.symbol())
            }
        }
    };
}

/// Implements the standard set of traits for a quantity type.
///
/// This covers: `Display`, `PartialEq`, `PartialOrd`, `Quantity`,
/// `Add`, `Sub`, `Mul<f64>`, `f64 * Quantity`, `Div<f64>`,
/// `Div<Self> -> f64`, and `Neg`.
///
/// Every standard quantity (those with `value: f64` and `unit: XUnit`)
/// has identical implementations for all of these.
macro_rules! impl_quantity {
    ($qty:ident, $unit:ident) => {
        impl std::fmt::Display for $qty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {}", self.value, self.unit.symbol())
            }
        }

        impl PartialEq for $qty {
            fn eq(&self, other: &Self) -> bool {
                (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
            }
        }

        impl PartialOrd for $qty {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.compare(other))
            }
        }

        impl $crate::core::Quantity for $qty {
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

        impl std::ops::Add for $qty {
            type Output = $qty;

            fn add(self, rhs: Self) -> Self::Output {
                use $crate::core::UnitOfMeasure;
                let sum = self.to_primary() + rhs.to_primary();
                $qty::new(self.unit.convert_from_primary(sum), self.unit)
            }
        }

        impl std::ops::Sub for $qty {
            type Output = $qty;

            fn sub(self, rhs: Self) -> Self::Output {
                use $crate::core::UnitOfMeasure;
                let diff = self.to_primary() - rhs.to_primary();
                $qty::new(self.unit.convert_from_primary(diff), self.unit)
            }
        }

        impl std::ops::Mul<f64> for $qty {
            type Output = $qty;

            fn mul(self, rhs: f64) -> Self::Output {
                $qty::new(self.value * rhs, self.unit)
            }
        }

        impl std::ops::Mul<$qty> for f64 {
            type Output = $qty;

            fn mul(self, rhs: $qty) -> Self::Output {
                $qty::new(self * rhs.value, rhs.unit)
            }
        }

        impl std::ops::Div<f64> for $qty {
            type Output = $qty;

            fn div(self, rhs: f64) -> Self::Output {
                $qty::new(self.value / rhs, self.unit)
            }
        }

        impl std::ops::Div<$qty> for $qty {
            type Output = f64;

            fn div(self, rhs: $qty) -> Self::Output {
                self.to_primary() / rhs.to_primary()
            }
        }

        impl std::ops::Neg for $qty {
            type Output = $qty;

            fn neg(self) -> Self::Output {
                $qty::new(-self.value, self.unit)
            }
        }
    };
}

/// Implements the `Dimension` trait for a quantity's dimension struct.
///
/// Every dimension struct follows the same pattern: a unit struct with
/// four static methods.
macro_rules! impl_dimension {
    ($dim:ident, $qty:ident, $unit:ident, $name:expr, $primary:expr, $si:expr) => {
        pub struct $dim;

        impl $crate::core::Dimension for $dim {
            type Quantity = $qty;
            type Unit = $unit;

            fn name() -> &'static str {
                $name
            }

            fn primary_unit() -> Self::Unit {
                $primary
            }

            fn si_unit() -> Self::Unit {
                $si
            }

            fn units() -> &'static [Self::Unit] {
                $unit::ALL
            }
        }
    };
}

pub(crate) use impl_dimension;
pub(crate) use impl_quantity;
pub(crate) use impl_unit_display;
