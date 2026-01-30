//! Spectral irradiance quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of spectral irradiance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpectralIrradianceUnit {
    /// Watts per cubic meter (W/m³) - SI unit
    WattsPerCubicMeter,
}

impl SpectralIrradianceUnit {
    /// All available spectral irradiance units.
    pub const ALL: &'static [SpectralIrradianceUnit] = &[SpectralIrradianceUnit::WattsPerCubicMeter];
}

impl fmt::Display for SpectralIrradianceUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for SpectralIrradianceUnit {
    fn symbol(&self) -> &'static str {
        match self {
            SpectralIrradianceUnit::WattsPerCubicMeter => "W/m³",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            SpectralIrradianceUnit::WattsPerCubicMeter => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, SpectralIrradianceUnit::WattsPerCubicMeter)
    }
}

/// A quantity of spectral irradiance.
///
/// Spectral irradiance represents power per unit area per unit wavelength.
/// SI unit: W/m³
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let si = SpectralIrradiance::watts_per_cubic_meter(100.0);
/// assert_eq!(si.to_watts_per_cubic_meter(), 100.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SpectralIrradiance {
    value: f64,
    unit: SpectralIrradianceUnit,
}

impl SpectralIrradiance {
    /// Creates a new SpectralIrradiance quantity.
    pub const fn new_const(value: f64, unit: SpectralIrradianceUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a SpectralIrradiance in watts per cubic meter.
    pub fn watts_per_cubic_meter(value: f64) -> Self {
        Self::new(value, SpectralIrradianceUnit::WattsPerCubicMeter)
    }

    // Conversion methods
    /// Converts to watts per cubic meter.
    pub fn to_watts_per_cubic_meter(&self) -> f64 {
        self.to(SpectralIrradianceUnit::WattsPerCubicMeter)
    }
}

impl fmt::Display for SpectralIrradiance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for SpectralIrradiance {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for SpectralIrradiance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for SpectralIrradiance {
    type Unit = SpectralIrradianceUnit;

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

impl Add for SpectralIrradiance {
    type Output = SpectralIrradiance;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        SpectralIrradiance::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for SpectralIrradiance {
    type Output = SpectralIrradiance;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        SpectralIrradiance::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for SpectralIrradiance {
    type Output = SpectralIrradiance;

    fn mul(self, rhs: f64) -> Self::Output {
        SpectralIrradiance::new(self.value * rhs, self.unit)
    }
}

impl Mul<SpectralIrradiance> for f64 {
    type Output = SpectralIrradiance;

    fn mul(self, rhs: SpectralIrradiance) -> Self::Output {
        SpectralIrradiance::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for SpectralIrradiance {
    type Output = SpectralIrradiance;

    fn div(self, rhs: f64) -> Self::Output {
        SpectralIrradiance::new(self.value / rhs, self.unit)
    }
}

impl Div<SpectralIrradiance> for SpectralIrradiance {
    type Output = f64;

    fn div(self, rhs: SpectralIrradiance) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for SpectralIrradiance {
    type Output = SpectralIrradiance;

    fn neg(self) -> Self::Output {
        SpectralIrradiance::new(-self.value, self.unit)
    }
}

/// Dimension for SpectralIrradiance.
pub struct SpectralIrradianceDimension;

impl Dimension for SpectralIrradianceDimension {
    type Quantity = SpectralIrradiance;
    type Unit = SpectralIrradianceUnit;

    fn name() -> &'static str {
        "SpectralIrradiance"
    }

    fn primary_unit() -> Self::Unit {
        SpectralIrradianceUnit::WattsPerCubicMeter
    }

    fn si_unit() -> Self::Unit {
        SpectralIrradianceUnit::WattsPerCubicMeter
    }

    fn units() -> &'static [Self::Unit] {
        SpectralIrradianceUnit::ALL
    }
}

/// Extension trait for creating SpectralIrradiance quantities from numeric types.
pub trait SpectralIrradianceConversions {
    /// Creates a SpectralIrradiance in watts per cubic meter.
    fn watts_per_cubic_meter(self) -> SpectralIrradiance;
}

impl SpectralIrradianceConversions for f64 {
    fn watts_per_cubic_meter(self) -> SpectralIrradiance {
        SpectralIrradiance::watts_per_cubic_meter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_irradiance_creation() {
        let si = SpectralIrradiance::watts_per_cubic_meter(100.0);
        assert_eq!(si.value(), 100.0);
        assert_eq!(si.unit(), SpectralIrradianceUnit::WattsPerCubicMeter);
    }

    #[test]
    fn test_spectral_irradiance_conversions() {
        let si = SpectralIrradiance::watts_per_cubic_meter(100.0);
        assert_eq!(si.to_watts_per_cubic_meter(), 100.0);
    }

    #[test]
    fn test_spectral_irradiance_arithmetic() {
        let si1 = SpectralIrradiance::watts_per_cubic_meter(100.0);
        let si2 = SpectralIrradiance::watts_per_cubic_meter(50.0);
        let sum = si1 + si2;
        assert_eq!(sum.to_watts_per_cubic_meter(), 150.0);
    }
}
