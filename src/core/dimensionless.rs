//! Dimensionless quantity and units.
//!
//! Represents quantities with no physical dimension, such as counts,
//! percentages, and ratios between like quantities.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Add, Mul, Sub};

/// Units of dimensionless measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DimensionlessUnit {
    /// Each - single units
    Each,
    /// Percent - hundredths (0.01)
    Percent,
    /// Dozen - 12 units
    Dozen,
    /// Score - 20 units
    Score,
    /// Gross - 144 units (12 dozen)
    Gross,
}

impl DimensionlessUnit {
    /// All available dimensionless units.
    pub const ALL: &'static [DimensionlessUnit] = &[
        DimensionlessUnit::Each,
        DimensionlessUnit::Percent,
        DimensionlessUnit::Dozen,
        DimensionlessUnit::Score,
        DimensionlessUnit::Gross,
    ];
}

impl_unit_display!(DimensionlessUnit);

impl UnitOfMeasure for DimensionlessUnit {
    fn symbol(&self) -> &'static str {
        match self {
            DimensionlessUnit::Each => "ea",
            DimensionlessUnit::Percent => "%",
            DimensionlessUnit::Dozen => "dz",
            DimensionlessUnit::Score => "score",
            DimensionlessUnit::Gross => "gr",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            DimensionlessUnit::Each => 1.0,
            DimensionlessUnit::Percent => 0.01,
            DimensionlessUnit::Dozen => 12.0,
            DimensionlessUnit::Score => 20.0,
            DimensionlessUnit::Gross => 144.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, DimensionlessUnit::Each)
    }
}

/// A dimensionless quantity (counts, percentages, ratios).
///
/// This may represent counts or other discrete amounts,
/// or ratios between like quantities where units cancel out.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let percentage = Dimensionless::percent(50.0);
/// let count = Dimensionless::each(100.0);
///
/// // 50% of 100 = 50
/// let result = count.to_each() * percentage.to_each();
/// assert!((result - 50.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Dimensionless {
    value: f64,
    unit: DimensionlessUnit,
}

impl Dimensionless {
    /// Creates a new Dimensionless quantity.
    pub const fn new_const(value: f64, unit: DimensionlessUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a Dimensionless quantity in "each" units.
    pub fn each(value: f64) -> Self {
        Self::new(value, DimensionlessUnit::Each)
    }

    /// Creates a Dimensionless quantity in percent.
    pub fn percent(value: f64) -> Self {
        Self::new(value, DimensionlessUnit::Percent)
    }

    /// Creates a Dimensionless quantity in dozens.
    pub fn dozen(value: f64) -> Self {
        Self::new(value, DimensionlessUnit::Dozen)
    }

    /// Creates a Dimensionless quantity in score (20 units).
    pub fn score(value: f64) -> Self {
        Self::new(value, DimensionlessUnit::Score)
    }

    /// Creates a Dimensionless quantity in gross (144 units).
    pub fn gross(value: f64) -> Self {
        Self::new(value, DimensionlessUnit::Gross)
    }

    /// Creates a Dimensionless quantity of 100 "each".
    pub fn hundred(value: f64) -> Self {
        Self::each(value * 100.0)
    }

    /// Creates a Dimensionless quantity of 1000 "each".
    pub fn thousand(value: f64) -> Self {
        Self::each(value * 1000.0)
    }

    /// Creates a Dimensionless quantity of 1,000,000 "each".
    pub fn million(value: f64) -> Self {
        Self::each(value * 1_000_000.0)
    }

    // Conversion methods
    /// Converts to "each" units.
    pub fn to_each(&self) -> f64 {
        self.to(DimensionlessUnit::Each)
    }

    /// Converts to percent.
    pub fn to_percent(&self) -> f64 {
        self.to(DimensionlessUnit::Percent)
    }

    /// Converts to dozens.
    pub fn to_dozen(&self) -> f64 {
        self.to(DimensionlessUnit::Dozen)
    }

    /// Converts to score.
    pub fn to_score(&self) -> f64 {
        self.to(DimensionlessUnit::Score)
    }

    /// Converts to gross.
    pub fn to_gross(&self) -> f64 {
        self.to(DimensionlessUnit::Gross)
    }
}

impl_quantity!(Dimensionless, DimensionlessUnit);

// Extra arithmetic operations specific to Dimensionless

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

// Dimensionless * Dimensionless = Dimensionless
impl Mul<Dimensionless> for Dimensionless {
    type Output = Dimensionless;

    fn mul(self, rhs: Dimensionless) -> Self::Output {
        Dimensionless::each(self.to_each() * rhs.to_each())
    }
}

/// Implicit conversion from Dimensionless to f64.
impl From<Dimensionless> for f64 {
    fn from(d: Dimensionless) -> Self {
        d.to_each()
    }
}

impl_dimension!(
    DimensionlessDimension,
    Dimensionless,
    DimensionlessUnit,
    "Dimensionless",
    DimensionlessUnit::Each,
    DimensionlessUnit::Each
);

/// Extension trait for creating Dimensionless quantities from numeric types.
pub trait DimensionlessConversions {
    /// Creates a Dimensionless quantity in "each" units.
    fn each(self) -> Dimensionless;
    /// Creates a Dimensionless quantity in percent.
    fn percent(self) -> Dimensionless;
    /// Creates a Dimensionless quantity in dozens.
    fn dozen(self) -> Dimensionless;
    /// Creates a Dimensionless quantity in score.
    fn score(self) -> Dimensionless;
    /// Creates a Dimensionless quantity in gross.
    fn gross(self) -> Dimensionless;
}

impl DimensionlessConversions for f64 {
    fn each(self) -> Dimensionless {
        Dimensionless::each(self)
    }
    fn percent(self) -> Dimensionless {
        Dimensionless::percent(self)
    }
    fn dozen(self) -> Dimensionless {
        Dimensionless::dozen(self)
    }
    fn score(self) -> Dimensionless {
        Dimensionless::score(self)
    }
    fn gross(self) -> Dimensionless {
        Dimensionless::gross(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensionless_creation() {
        let d = Dimensionless::each(10.0);
        assert_eq!(d.value(), 10.0);
        assert_eq!(d.unit(), DimensionlessUnit::Each);
    }

    #[test]
    fn test_percent_conversion() {
        let d = Dimensionless::percent(50.0);
        assert_eq!(d.to_each(), 0.5);

        let d2 = Dimensionless::each(0.25);
        assert_eq!(d2.to_percent(), 25.0);
    }

    #[test]
    fn test_dozen_conversion() {
        let d = Dimensionless::dozen(2.0);
        assert_eq!(d.to_each(), 24.0);
    }

    #[test]
    fn test_gross_conversion() {
        let d = Dimensionless::gross(1.0);
        assert_eq!(d.to_each(), 144.0);
        assert_eq!(d.to_dozen(), 12.0);
    }

    #[test]
    fn test_dimensionless_multiplication() {
        let d1 = Dimensionless::each(3.0);
        let d2 = Dimensionless::each(4.0);
        let result = d1 * d2;
        assert_eq!(result.to_each(), 12.0);
    }

    #[test]
    fn test_add_scalar() {
        let d = Dimensionless::each(5.0);
        let result = d + 3.0;
        assert_eq!(result.to_each(), 8.0);
    }

    #[test]
    fn test_hundred_thousand_million() {
        let h = Dimensionless::hundred(1.0);
        assert_eq!(h.to_each(), 100.0);

        let t = Dimensionless::thousand(1.0);
        assert_eq!(t.to_each(), 1000.0);

        let m = Dimensionless::million(1.0);
        assert_eq!(m.to_each(), 1_000_000.0);
    }

    #[test]
    fn test_conversion_to_f64() {
        let d = Dimensionless::dozen(2.0);
        let value: f64 = d.into();
        assert_eq!(value, 24.0);
    }
}
