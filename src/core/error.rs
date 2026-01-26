//! Error types for quantity operations.

use thiserror::Error;

/// Errors that can occur when working with quantities.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum QuantityError {
    /// Error parsing a quantity from a string.
    #[error("Failed to parse quantity: {0}")]
    ParseError(#[from] QuantityParseError),

    /// Error when attempting an invalid unit conversion.
    #[error("Invalid conversion: {0}")]
    ConversionError(String),

    /// Error when a range is invalid (e.g., lower >= upper).
    #[error("Invalid range: {0}")]
    RangeError(String),

    /// Error for unsupported operations.
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}

/// Error parsing a quantity from a string.
#[derive(Error, Debug, Clone, PartialEq)]
#[error("Unable to parse {dimension}: '{input}'")]
pub struct QuantityParseError {
    /// The dimension being parsed (e.g., "Length", "Mass").
    pub dimension: String,
    /// The input string that failed to parse.
    pub input: String,
}

impl QuantityParseError {
    /// Creates a new parse error.
    pub fn new(dimension: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            dimension: dimension.into(),
            input: input.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_display() {
        let err = QuantityParseError::new("Length", "invalid");
        assert_eq!(err.to_string(), "Unable to parse Length: 'invalid'");
    }

    #[test]
    fn test_quantity_error_from_parse_error() {
        let parse_err = QuantityParseError::new("Mass", "bad input");
        let quantity_err: QuantityError = parse_err.into();
        assert!(matches!(quantity_err, QuantityError::ParseError(_)));
    }
}
