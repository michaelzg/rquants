//! Dimensionless quantity and units.
//!
//! Represents quantities with no physical dimension, such as counts,
//! percentages, and ratios between like quantities.

use std::ops::{Add, Mul, Sub};

crate::quantity! {
    /// A dimensionless quantity such as a count, percentage, or ratio.
    ///
    /// Dividing quantities of the same dimension returns a bare `f64` for
    /// ergonomic ratio arithmetic. Use `Dimensionless` when the result should
    /// retain an explicit unit such as percent or dozen.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let percentage = Dimensionless::percent(50.0);
    /// let count = Dimensionless::each(100.0);
    /// let result = count.to_each() * percentage.to_each();
    ///
    /// assert!((result - 50.0).abs() < 1e-10);
    /// ```
    pub quantity Dimensionless {
        /// Units of dimensionless measurement.
        unit: DimensionlessUnit;
        /// Dimension marker for dimensionless quantities.
        dimension: DimensionlessDimension;
        /// Extension trait for creating dimensionless quantities from `f64`.
        conversions: DimensionlessConversions;
        name: "Dimensionless";
        primary: Each;
        si: Each;

        units {
            /// Single units.
            Each {
                symbol: "ea",
                factor: 1.0,
                ctor: each,
                to: to_each,
                si: true
            },
            /// Hundredths.
            Percent {
                symbol: "%",
                factor: 0.01,
                ctor: percent,
                to: to_percent,
                si: false
            },
            /// Groups of 12.
            Dozen {
                symbol: "dz",
                factor: 12.0,
                ctor: dozen,
                to: to_dozen,
                si: false
            },
            /// Groups of 20.
            Score {
                symbol: "score",
                factor: 20.0,
                ctor: score,
                to: to_score,
                si: false
            },
            /// Groups of 144.
            Gross {
                symbol: "gr",
                factor: 144.0,
                ctor: gross,
                to: to_gross,
                si: false
            }
        }
    }
}

impl Dimensionless {
    /// Creates a dimensionless quantity of 100 each.
    pub fn hundred(value: f64) -> Self {
        Self::each(value * 100.0)
    }

    /// Creates a dimensionless quantity of 1,000 each.
    pub fn thousand(value: f64) -> Self {
        Self::each(value * 1_000.0)
    }

    /// Creates a dimensionless quantity of 1,000,000 each.
    pub fn million(value: f64) -> Self {
        Self::each(value * 1_000_000.0)
    }
}

impl Add<f64> for Dimensionless {
    type Output = Dimensionless;

    fn add(self, rhs: f64) -> Self::Output {
        self + Dimensionless::each(rhs)
    }
}

impl Sub<f64> for Dimensionless {
    type Output = Dimensionless;

    fn sub(self, rhs: f64) -> Self::Output {
        self - Dimensionless::each(rhs)
    }
}

impl Mul<Dimensionless> for Dimensionless {
    type Output = Dimensionless;

    fn mul(self, rhs: Dimensionless) -> Self::Output {
        Dimensionless::each(self.to_each() * rhs.to_each())
    }
}

impl From<Dimensionless> for f64 {
    fn from(dimensionless: Dimensionless) -> Self {
        dimensionless.to_each()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_dimensionless_creation() {
        let dimensionless = Dimensionless::each(10.0);
        assert_eq!(dimensionless.value(), 10.0);
        assert_eq!(dimensionless.unit(), DimensionlessUnit::Each);
    }

    #[test]
    fn test_percent_conversion() {
        assert_eq!(Dimensionless::percent(50.0).to_each(), 0.5);
        assert_eq!(Dimensionless::each(0.25).to_percent(), 25.0);
    }

    #[test]
    fn test_group_conversions() {
        assert_eq!(Dimensionless::dozen(2.0).to_each(), 24.0);
        assert_eq!(Dimensionless::gross(1.0).to_each(), 144.0);
        assert_eq!(Dimensionless::gross(1.0).to_dozen(), 12.0);
    }

    #[test]
    fn test_scalar_arithmetic_retains_left_unit() {
        let sum = Dimensionless::percent(50.0) + 1.0;
        assert_eq!(sum.unit(), DimensionlessUnit::Percent);
        assert_eq!(sum.to_percent(), 150.0);

        let difference = sum - 0.5;
        assert_eq!(difference.unit(), DimensionlessUnit::Percent);
        assert_eq!(difference.to_percent(), 100.0);
    }

    #[test]
    fn test_dimensionless_multiplication_normalizes_to_each() {
        let product = Dimensionless::percent(50.0) * Dimensionless::dozen(2.0);
        assert_eq!(product.unit(), DimensionlessUnit::Each);
        assert_eq!(product.to_each(), 12.0);
    }

    #[test]
    fn test_dimensionless_division_returns_scalar() {
        let ratio: f64 = Dimensionless::gross(1.0) / Dimensionless::dozen(1.0);
        assert_eq!(ratio, 12.0);
    }

    #[test]
    fn test_hundred_thousand_million() {
        let values = [
            (Dimensionless::hundred(1.0), 100.0),
            (Dimensionless::thousand(1.0), 1_000.0),
            (Dimensionless::million(1.0), 1_000_000.0),
        ];

        for (dimensionless, expected) in values {
            assert_eq!(dimensionless.unit(), DimensionlessUnit::Each);
            assert_eq!(dimensionless.to_each(), expected);
        }
    }

    #[test]
    fn test_conversion_to_f64_uses_primary_unit() {
        let value: f64 = Dimensionless::dozen(2.0).into();
        assert_eq!(value, 24.0);
    }
}
