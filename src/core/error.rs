//! Error types for quantity operations.

use std::fmt;

/// Errors that can occur when working with quantities.
#[derive(Debug, Clone, PartialEq)]
pub enum QuantityError {
    /// Error parsing a quantity from a string.
    ParseError(QuantityParseError),

    /// Error when attempting an invalid unit conversion.
    ConversionError(String),

    /// Error when a range is invalid (e.g., lower >= upper).
    RangeError(String),

    /// Error for unsupported operations.
    UnsupportedOperation(String),
}

impl fmt::Display for QuantityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuantityError::ParseError(e) => write!(f, "Failed to parse quantity: {e}"),
            QuantityError::ConversionError(msg) => write!(f, "Invalid conversion: {msg}"),
            QuantityError::RangeError(msg) => write!(f, "Invalid range: {msg}"),
            QuantityError::UnsupportedOperation(msg) => {
                write!(f, "Unsupported operation: {msg}")
            }
        }
    }
}

impl std::error::Error for QuantityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            QuantityError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<QuantityParseError> for QuantityError {
    fn from(err: QuantityParseError) -> Self {
        QuantityError::ParseError(err)
    }
}

/// Error parsing a quantity from a string.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantityParseError {
    /// The dimension being parsed (e.g., "Length", "Mass").
    pub dimension: String,
    /// The input string that failed to parse.
    pub input: String,
}

impl fmt::Display for QuantityParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse {}: '{}'", self.dimension, self.input)
    }
}

impl std::error::Error for QuantityParseError {}

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
