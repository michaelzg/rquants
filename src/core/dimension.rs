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
/// ```ignore
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
        let (value_str, unit_str) = parse_value_and_unit(s)?;

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
fn parse_value_and_unit(s: &str) -> Result<(&str, &str), QuantityParseError> {
    // Find where the numeric part ends
    let mut value_end = 0;
    let chars: Vec<char> = s.chars().collect();

    // Skip leading whitespace
    while value_end < chars.len() && chars[value_end].is_whitespace() {
        value_end += 1;
    }

    let start = value_end;

    // Handle optional sign
    if value_end < chars.len() && (chars[value_end] == '-' || chars[value_end] == '+') {
        value_end += 1;
    }

    // Parse digits and decimal point
    let mut has_decimal = false;
    let mut has_exponent = false;

    while value_end < chars.len() {
        let c = chars[value_end];
        if c.is_ascii_digit() {
            value_end += 1;
        } else if c == '.' && !has_decimal && !has_exponent {
            has_decimal = true;
            value_end += 1;
        } else if (c == 'e' || c == 'E') && !has_exponent {
            has_exponent = true;
            value_end += 1;
            // Handle optional sign after exponent
            if value_end < chars.len()
                && (chars[value_end] == '-' || chars[value_end] == '+')
            {
                value_end += 1;
            }
        } else {
            break;
        }
    }

    if value_end == start {
        return Err(QuantityParseError {
            dimension: "Unknown".to_string(),
            input: s.to_string(),
        });
    }

    let value_str = &s[start..value_end];
    let unit_str = s[value_end..].trim();

    if unit_str.is_empty() {
        return Err(QuantityParseError {
            dimension: "Unknown".to_string(),
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
        assert_eq!(parse_value_and_unit("10 m").unwrap(), ("10", "m"));
        assert_eq!(parse_value_and_unit("10m").unwrap(), ("10", "m"));
        assert_eq!(parse_value_and_unit("-10.5 kg").unwrap(), ("-10.5", "kg"));
        assert_eq!(parse_value_and_unit("1.5e10 m").unwrap(), ("1.5e10", "m"));
        assert_eq!(parse_value_and_unit("1.5E-10 m").unwrap(), ("1.5E-10", "m"));
    }

    #[test]
    fn test_parse_value_and_unit_errors() {
        assert!(parse_value_and_unit("m").is_err()); // No value
        assert!(parse_value_and_unit("10").is_err()); // No unit
        assert!(parse_value_and_unit("").is_err()); // Empty
    }
}
