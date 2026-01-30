//! Spectral power quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of spectral power measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpectralPowerUnit {
    /// Watts per meter (W/m) - SI unit
    WattsPerMeter,
}

impl SpectralPowerUnit {
    /// All available spectral power units.
    pub const ALL: &'static [SpectralPowerUnit] = &[SpectralPowerUnit::WattsPerMeter];
}

impl fmt::Display for SpectralPowerUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for SpectralPowerUnit {
    fn symbol(&self) -> &'static str {
        match self {
            SpectralPowerUnit::WattsPerMeter => "W/m",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            SpectralPowerUnit::WattsPerMeter => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, SpectralPowerUnit::WattsPerMeter)
    }
}

/// A quantity of spectral power.
///
/// Spectral power represents power per unit wavelength.
/// SI unit: W/m
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let sp = SpectralPower::watts_per_meter(50.0);
/// let length = Length::meters(2.0);
///
/// // SpectralPower * Length = Power
/// let power = sp * length;
/// assert_eq!(power.to_watts(), 100.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SpectralPower {
    value: f64,
    unit: SpectralPowerUnit,
}

impl SpectralPower {
    /// Creates a new SpectralPower quantity.
    pub const fn new_const(value: f64, unit: SpectralPowerUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a SpectralPower in watts per meter.
    pub fn watts_per_meter(value: f64) -> Self {
        Self::new(value, SpectralPowerUnit::WattsPerMeter)
    }

    // Conversion methods
    /// Converts to watts per meter.
    pub fn to_watts_per_meter(&self) -> f64 {
        self.to(SpectralPowerUnit::WattsPerMeter)
    }
}

impl fmt::Display for SpectralPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for SpectralPower {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for SpectralPower {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for SpectralPower {
    type Unit = SpectralPowerUnit;

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

impl Add for SpectralPower {
    type Output = SpectralPower;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        SpectralPower::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for SpectralPower {
    type Output = SpectralPower;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        SpectralPower::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for SpectralPower {
    type Output = SpectralPower;

    fn mul(self, rhs: f64) -> Self::Output {
        SpectralPower::new(self.value * rhs, self.unit)
    }
}

impl Mul<SpectralPower> for f64 {
    type Output = SpectralPower;

    fn mul(self, rhs: SpectralPower) -> Self::Output {
        SpectralPower::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for SpectralPower {
    type Output = SpectralPower;

    fn div(self, rhs: f64) -> Self::Output {
        SpectralPower::new(self.value / rhs, self.unit)
    }
}

impl Div<SpectralPower> for SpectralPower {
    type Output = f64;

    fn div(self, rhs: SpectralPower) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for SpectralPower {
    type Output = SpectralPower;

    fn neg(self) -> Self::Output {
        SpectralPower::new(-self.value, self.unit)
    }
}

// Cross-quantity operations
use crate::energy::{Power, PowerUnit};
use crate::space::{Length, LengthUnit};

// SpectralPower * Length = Power
impl Mul<Length> for SpectralPower {
    type Output = Power;

    fn mul(self, rhs: Length) -> Self::Output {
        let watts = self.to_watts_per_meter() * rhs.to_meters();
        Power::new(watts, PowerUnit::Watts)
    }
}

// Length * SpectralPower = Power
impl Mul<SpectralPower> for Length {
    type Output = Power;

    fn mul(self, rhs: SpectralPower) -> Self::Output {
        let watts = rhs.to_watts_per_meter() * self.to_meters();
        Power::new(watts, PowerUnit::Watts)
    }
}

// SpectralPower / Power = 1/Length
// Power / SpectralPower = Length
impl Div<SpectralPower> for Power {
    type Output = Length;

    fn div(self, rhs: SpectralPower) -> Self::Output {
        let meters = self.to_watts() / rhs.to_watts_per_meter();
        Length::new(meters, LengthUnit::Meters)
    }
}

/// Dimension for SpectralPower.
pub struct SpectralPowerDimension;

impl Dimension for SpectralPowerDimension {
    type Quantity = SpectralPower;
    type Unit = SpectralPowerUnit;

    fn name() -> &'static str {
        "SpectralPower"
    }

    fn primary_unit() -> Self::Unit {
        SpectralPowerUnit::WattsPerMeter
    }

    fn si_unit() -> Self::Unit {
        SpectralPowerUnit::WattsPerMeter
    }

    fn units() -> &'static [Self::Unit] {
        SpectralPowerUnit::ALL
    }
}

/// Extension trait for creating SpectralPower quantities from numeric types.
pub trait SpectralPowerConversions {
    /// Creates a SpectralPower in watts per meter.
    fn watts_per_meter(self) -> SpectralPower;
}

impl SpectralPowerConversions for f64 {
    fn watts_per_meter(self) -> SpectralPower {
        SpectralPower::watts_per_meter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_power_creation() {
        let sp = SpectralPower::watts_per_meter(50.0);
        assert_eq!(sp.value(), 50.0);
        assert_eq!(sp.unit(), SpectralPowerUnit::WattsPerMeter);
    }

    #[test]
    fn test_spectral_power_conversions() {
        let sp = SpectralPower::watts_per_meter(50.0);
        assert_eq!(sp.to_watts_per_meter(), 50.0);
    }

    #[test]
    fn test_spectral_power_times_length() {
        let sp = SpectralPower::watts_per_meter(50.0);
        let length = Length::meters(2.0);
        let power = sp * length;
        assert_eq!(power.to_watts(), 100.0);
    }

    #[test]
    fn test_power_divided_by_spectral_power() {
        let power = Power::watts(100.0);
        let sp = SpectralPower::watts_per_meter(50.0);
        let length = power / sp;
        assert_eq!(length.to_meters(), 2.0);
    }
}
