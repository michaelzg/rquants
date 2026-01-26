//! Unit of measure trait and supporting types.

use std::fmt::{Debug, Display};
use std::hash::Hash;

/// Trait for units of measurement.
///
/// A unit of measure defines a standard for measuring a particular quantity.
/// Each unit has a symbol and a conversion factor relative to the primary unit.
///
/// # Example
///
/// ```ignore
/// // Units are typically implemented as enums
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// pub enum LengthUnit {
///     Meters,
///     Kilometers,
///     Feet,
/// }
///
/// impl UnitOfMeasure for LengthUnit {
///     fn symbol(&self) -> &'static str {
///         match self {
///             LengthUnit::Meters => "m",
///             LengthUnit::Kilometers => "km",
///             LengthUnit::Feet => "ft",
///         }
///     }
///
///     fn conversion_factor(&self) -> f64 {
///         match self {
///             LengthUnit::Meters => 1.0,
///             LengthUnit::Kilometers => 1000.0,
///             LengthUnit::Feet => 0.3048,
///         }
///     }
/// }
/// ```
pub trait UnitOfMeasure: Debug + Clone + Copy + PartialEq + Eq + Hash + Display {
    /// Returns the symbol for this unit (e.g., "m", "kg", "s").
    fn symbol(&self) -> &'static str;

    /// Returns the conversion factor relative to the primary unit.
    ///
    /// The primary unit has a conversion factor of 1.0.
    /// For example, if meters is the primary unit for length:
    /// - Meters: 1.0
    /// - Kilometers: 1000.0
    /// - Centimeters: 0.01
    fn conversion_factor(&self) -> f64;

    /// Returns true if this is the primary (base) unit for its dimension.
    fn is_primary(&self) -> bool {
        (self.conversion_factor() - 1.0).abs() < f64::EPSILON
    }

    /// Returns true if this is an SI unit.
    fn is_si(&self) -> bool {
        false
    }

    /// Converts a value from the primary unit to this unit.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Convert 1000 meters to kilometers
    /// let km = LengthUnit::Kilometers;
    /// assert_eq!(km.convert_from_primary(1000.0), 1.0);
    /// ```
    fn convert_from_primary(&self, value: f64) -> f64 {
        value / self.conversion_factor()
    }

    /// Converts a value from this unit to the primary unit.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Convert 1 kilometer to meters
    /// let km = LengthUnit::Kilometers;
    /// assert_eq!(km.convert_to_primary(1.0), 1000.0);
    /// ```
    fn convert_to_primary(&self, value: f64) -> f64 {
        value * self.conversion_factor()
    }

    /// Converts a value from this unit to another unit.
    fn convert_to(&self, value: f64, target: &Self) -> f64 {
        target.convert_from_primary(self.convert_to_primary(value))
    }
}

/// Marker trait for SI base units.
pub trait SiBaseUnit: UnitOfMeasure {}

/// Marker trait for SI derived units.
pub trait SiUnit: UnitOfMeasure {}

/// Marker trait for the primary (reference) unit of a dimension.
pub trait PrimaryUnit: UnitOfMeasure {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestUnit {
        Primary,
        Double,
        Half,
    }

    impl Display for TestUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.symbol())
        }
    }

    impl UnitOfMeasure for TestUnit {
        fn symbol(&self) -> &'static str {
            match self {
                TestUnit::Primary => "p",
                TestUnit::Double => "d",
                TestUnit::Half => "h",
            }
        }

        fn conversion_factor(&self) -> f64 {
            match self {
                TestUnit::Primary => 1.0,
                TestUnit::Double => 2.0,
                TestUnit::Half => 0.5,
            }
        }
    }

    #[test]
    fn test_is_primary() {
        assert!(TestUnit::Primary.is_primary());
        assert!(!TestUnit::Double.is_primary());
        assert!(!TestUnit::Half.is_primary());
    }

    #[test]
    fn test_convert_to_primary() {
        assert_eq!(TestUnit::Primary.convert_to_primary(10.0), 10.0);
        assert_eq!(TestUnit::Double.convert_to_primary(10.0), 20.0);
        assert_eq!(TestUnit::Half.convert_to_primary(10.0), 5.0);
    }

    #[test]
    fn test_convert_from_primary() {
        assert_eq!(TestUnit::Primary.convert_from_primary(10.0), 10.0);
        assert_eq!(TestUnit::Double.convert_from_primary(10.0), 5.0);
        assert_eq!(TestUnit::Half.convert_from_primary(10.0), 20.0);
    }

    #[test]
    fn test_convert_to() {
        // Convert 10 double units to half units
        // 10 double = 20 primary = 40 half
        assert_eq!(TestUnit::Double.convert_to(10.0, &TestUnit::Half), 40.0);

        // Convert 10 half units to double units
        // 10 half = 5 primary = 2.5 double
        assert_eq!(TestUnit::Half.convert_to(10.0, &TestUnit::Double), 2.5);
    }
}
