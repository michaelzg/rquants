//! Dimension trait for quantity metadata and parsing.

use super::error::QuantityParseError;
use super::quantity::Quantity;
use super::unit::UnitOfMeasure;

/// Trait for dimension metadata and factory operations.
///
/// A dimension represents a type of physical quantity (e.g., Length, Mass, Time).
/// It provides metadata about the dimension and methods for parsing quantities
/// from strings.
///
/// # Example
///
/// ```rust
/// use rquants::space::{Length, LengthUnit};
/// use rquants::Dimension;
///
/// pub struct LengthDimension;
///
/// impl Dimension for LengthDimension {
///     type Quantity = Length;
///     type Unit = LengthUnit;
///
///     fn name() -> &'static str {
///         "Length"
///     }
///
///     fn primary_unit() -> Self::Unit {
///         LengthUnit::Meters
///     }
///
///     fn si_unit() -> Self::Unit {
///         LengthUnit::Meters
///     }
///
///     fn units() -> &'static [Self::Unit] {
///         &[LengthUnit::Meters, LengthUnit::Kilometers, LengthUnit::Feet]
///     }
/// }
/// ```
pub trait Dimension {
    /// The quantity type for this dimension.
    type Quantity: Quantity<Unit = Self::Unit>;

    /// The unit type for this dimension.
    type Unit: UnitOfMeasure + 'static;

    /// Returns the name of this dimension (e.g., "Length", "Mass").
    fn name() -> &'static str;

    /// Returns the primary (reference) unit for this dimension.
    ///
    /// The primary unit has a conversion factor of 1.0.
    fn primary_unit() -> Self::Unit;

    /// Returns the SI unit for this dimension.
    ///
    /// For base SI dimensions, this is typically the same as the primary unit.
    fn si_unit() -> Self::Unit;

    /// Returns all available units for this dimension.
    fn units() -> &'static [Self::Unit];

    /// Attempts to find a unit by its symbol.
    fn unit_by_symbol(symbol: &str) -> Option<Self::Unit> {
        Self::units().iter().find(|u| u.symbol() == symbol).copied()
    }

    /// Parses a string into a quantity.
    ///
    /// The string should be in the format "value unit" (e.g., "10 m", "5.5 kg").
    ///
    /// # Errors
    ///
    /// Returns a `QuantityParseError` if the string cannot be parsed.
    fn parse(s: &str) -> Result<Self::Quantity, QuantityParseError> {
        let s = s.trim();

        // Try to find the split between value and unit
        // Handle formats like "10m", "10 m", "10.5 m", "-10.5 m"
        let (value_str, unit_str) = parse_value_and_unit(s, Self::name())?;

        let value: f64 = value_str.parse().map_err(|_| QuantityParseError {
            dimension: Self::name().to_string(),
            input: s.to_string(),
        })?;

        let unit = Self::unit_by_symbol(unit_str.trim()).ok_or_else(|| QuantityParseError {
            dimension: Self::name().to_string(),
            input: s.to_string(),
        })?;

        Ok(Self::Quantity::new(value, unit))
    }
}

/// Helper function to parse a value and unit from a string.
fn parse_value_and_unit<'a>(
    s: &'a str,
    dimension: &str,
) -> Result<(&'a str, &'a str), QuantityParseError> {
    let s = s.trim_start();
    let value_end = s
        .find(|c: char| !(c.is_ascii_digit() || matches!(c, '+' | '-' | '.' | 'e' | 'E')))
        .unwrap_or(s.len());

    if value_end == 0 {
        return Err(QuantityParseError {
            dimension: dimension.to_string(),
            input: s.to_string(),
        });
    }

    let (value_str, unit_str) = s.split_at(value_end);
    let unit_str = unit_str.trim();

    if unit_str.is_empty() {
        return Err(QuantityParseError {
            dimension: dimension.to_string(),
            input: s.to_string(),
        });
    }

    Ok((value_str, unit_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value_and_unit() {
        assert_eq!(parse_value_and_unit("10 m", "Test").unwrap(), ("10", "m"));
        assert_eq!(parse_value_and_unit("10m", "Test").unwrap(), ("10", "m"));
        assert_eq!(
            parse_value_and_unit("-10.5 kg", "Test").unwrap(),
            ("-10.5", "kg")
        );
        assert_eq!(
            parse_value_and_unit("1.5e10 m", "Test").unwrap(),
            ("1.5e10", "m")
        );
        assert_eq!(
            parse_value_and_unit("1.5E-10 m", "Test").unwrap(),
            ("1.5E-10", "m")
        );
        assert_eq!(parse_value_and_unit("10µm", "Test").unwrap(), ("10", "µm"));
    }

    #[test]
    fn test_parse_value_and_unit_errors() {
        let err = parse_value_and_unit("m", "Length").unwrap_err();
        assert_eq!(err.dimension, "Length");

        assert!(parse_value_and_unit("10", "Length").is_err()); // No unit
        assert!(parse_value_and_unit("", "Length").is_err()); // Empty
    }
}
