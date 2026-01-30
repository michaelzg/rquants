//! Dose quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of radiation dose measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DoseUnit {
    /// Sieverts (Sv) - SI unit
    Sieverts,
    /// Rems (rem)
    Rems,
}

impl DoseUnit {
    /// All available dose units.
    pub const ALL: &'static [DoseUnit] = &[DoseUnit::Sieverts, DoseUnit::Rems];
}

// Conversion factor
const SIEVERT_TO_REM: f64 = 100.0;

impl fmt::Display for DoseUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for DoseUnit {
    fn symbol(&self) -> &'static str {
        match self {
            DoseUnit::Sieverts => "Sv",
            DoseUnit::Rems => "rem",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            DoseUnit::Sieverts => 1.0,
            DoseUnit::Rems => 1.0 / SIEVERT_TO_REM,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, DoseUnit::Sieverts)
    }
}

/// A quantity of radiation dose.
///
/// Dose represents the equivalent or effective radiation dose to biological tissue.
/// SI unit: Sievert (Sv)
/// 1 Sv = 100 rem
///
/// Note: This is different from absorbed dose (SpecificEnergy). Dose measures
/// the biological effect of radiation, while absorbed dose measures the energy
/// deposited per unit mass.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let dose = Dose::sieverts(1.0);
/// assert_eq!(dose.to_rems(), 100.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Dose {
    value: f64,
    unit: DoseUnit,
}

impl Dose {
    /// Creates a new Dose quantity.
    pub const fn new_const(value: f64, unit: DoseUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a Dose in sieverts.
    pub fn sieverts(value: f64) -> Self {
        Self::new(value, DoseUnit::Sieverts)
    }

    /// Creates a Dose in rems.
    pub fn rems(value: f64) -> Self {
        Self::new(value, DoseUnit::Rems)
    }

    // Conversion methods
    /// Converts to sieverts.
    pub fn to_sieverts(&self) -> f64 {
        self.to(DoseUnit::Sieverts)
    }

    /// Converts to rems.
    pub fn to_rems(&self) -> f64 {
        self.to(DoseUnit::Rems)
    }
}

impl fmt::Display for Dose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Dose {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Dose {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Dose {
    type Unit = DoseUnit;

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

// Arithmetic operations

impl Add for Dose {
    type Output = Dose;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Dose::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Dose {
    type Output = Dose;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Dose::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Dose {
    type Output = Dose;

    fn mul(self, rhs: f64) -> Self::Output {
        Dose::new(self.value * rhs, self.unit)
    }
}

impl Mul<Dose> for f64 {
    type Output = Dose;

    fn mul(self, rhs: Dose) -> Self::Output {
        Dose::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for Dose {
    type Output = Dose;

    fn div(self, rhs: f64) -> Self::Output {
        Dose::new(self.value / rhs, self.unit)
    }
}

impl Div<Dose> for Dose {
    type Output = f64;

    fn div(self, rhs: Dose) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Dose {
    type Output = Dose;

    fn neg(self) -> Self::Output {
        Dose::new(-self.value, self.unit)
    }
}

/// Dimension for Dose.
pub struct DoseDimension;

impl Dimension for DoseDimension {
    type Quantity = Dose;
    type Unit = DoseUnit;

    fn name() -> &'static str {
        "Dose"
    }

    fn primary_unit() -> Self::Unit {
        DoseUnit::Sieverts
    }

    fn si_unit() -> Self::Unit {
        DoseUnit::Sieverts
    }

    fn units() -> &'static [Self::Unit] {
        DoseUnit::ALL
    }
}

/// Extension trait for creating Dose quantities from numeric types.
pub trait DoseConversions {
    /// Creates a Dose in sieverts.
    fn sieverts(self) -> Dose;
    /// Creates a Dose in rems.
    fn rems(self) -> Dose;
}

impl DoseConversions for f64 {
    fn sieverts(self) -> Dose {
        Dose::sieverts(self)
    }
    fn rems(self) -> Dose {
        Dose::rems(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dose_creation() {
        let d = Dose::sieverts(1.0);
        assert_eq!(d.value(), 1.0);
        assert_eq!(d.unit(), DoseUnit::Sieverts);
    }

    #[test]
    fn test_dose_conversions() {
        let d = Dose::sieverts(1.0);
        assert_eq!(d.to_rems(), 100.0);

        let d2 = Dose::rems(100.0);
        assert!((d2.to_sieverts() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_dose_arithmetic() {
        let d1 = Dose::sieverts(1.0);
        let d2 = Dose::sieverts(0.5);
        let sum = d1 + d2;
        assert_eq!(sum.to_sieverts(), 1.5);
    }
}
