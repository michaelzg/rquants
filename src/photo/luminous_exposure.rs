//! Luminous exposure quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of luminous exposure measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LuminousExposureUnit {
    /// Lux-seconds (lx·s) - SI unit
    LuxSeconds,
}

impl LuminousExposureUnit {
    /// All available luminous exposure units.
    pub const ALL: &'static [LuminousExposureUnit] = &[LuminousExposureUnit::LuxSeconds];
}

impl fmt::Display for LuminousExposureUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for LuminousExposureUnit {
    fn symbol(&self) -> &'static str {
        match self {
            LuminousExposureUnit::LuxSeconds => "lx·s",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            LuminousExposureUnit::LuxSeconds => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, LuminousExposureUnit::LuxSeconds)
    }
}

/// A quantity of luminous exposure.
///
/// Luminous exposure represents the time integral of illuminance.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let exposure = LuminousExposure::lux_seconds(600.0);
/// let time = Time::seconds(3.0);
///
/// // LuminousExposure / Time = Illuminance
/// let illuminance = exposure / time;
/// assert!((illuminance.to_lux() - 200.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct LuminousExposure {
    value: f64,
    unit: LuminousExposureUnit,
}

impl LuminousExposure {
    /// Creates a new LuminousExposure quantity.
    pub const fn new_const(value: f64, unit: LuminousExposureUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a LuminousExposure in lux-seconds.
    pub fn lux_seconds(value: f64) -> Self {
        Self::new(value, LuminousExposureUnit::LuxSeconds)
    }

    // Conversion methods
    /// Converts to lux-seconds.
    pub fn to_lux_seconds(&self) -> f64 {
        self.to(LuminousExposureUnit::LuxSeconds)
    }
}

impl fmt::Display for LuminousExposure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for LuminousExposure {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for LuminousExposure {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for LuminousExposure {
    type Unit = LuminousExposureUnit;

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

impl Add for LuminousExposure {
    type Output = LuminousExposure;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        LuminousExposure::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for LuminousExposure {
    type Output = LuminousExposure;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        LuminousExposure::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for LuminousExposure {
    type Output = LuminousExposure;

    fn mul(self, rhs: f64) -> Self::Output {
        LuminousExposure::new(self.value * rhs, self.unit)
    }
}

impl Mul<LuminousExposure> for f64 {
    type Output = LuminousExposure;

    fn mul(self, rhs: LuminousExposure) -> Self::Output {
        LuminousExposure::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for LuminousExposure {
    type Output = LuminousExposure;

    fn div(self, rhs: f64) -> Self::Output {
        LuminousExposure::new(self.value / rhs, self.unit)
    }
}

impl Div<LuminousExposure> for LuminousExposure {
    type Output = f64;

    fn div(self, rhs: LuminousExposure) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for LuminousExposure {
    type Output = LuminousExposure;

    fn neg(self) -> Self::Output {
        LuminousExposure::new(-self.value, self.unit)
    }
}

// Cross-quantity operations
use super::illuminance::{Illuminance, IlluminanceUnit};
use crate::time::{Time, TimeUnit};

// LuminousExposure / Time = Illuminance
impl Div<Time> for LuminousExposure {
    type Output = Illuminance;

    fn div(self, rhs: Time) -> Self::Output {
        let lux = self.to_lux_seconds() / rhs.to_seconds();
        Illuminance::new(lux, IlluminanceUnit::Lux)
    }
}

// LuminousExposure / Illuminance = Time
impl Div<Illuminance> for LuminousExposure {
    type Output = Time;

    fn div(self, rhs: Illuminance) -> Self::Output {
        let seconds = self.to_lux_seconds() / rhs.to_lux();
        Time::new(seconds, TimeUnit::Seconds)
    }
}

/// Dimension for LuminousExposure.
pub struct LuminousExposureDimension;

impl Dimension for LuminousExposureDimension {
    type Quantity = LuminousExposure;
    type Unit = LuminousExposureUnit;

    fn name() -> &'static str {
        "LuminousExposure"
    }

    fn primary_unit() -> Self::Unit {
        LuminousExposureUnit::LuxSeconds
    }

    fn si_unit() -> Self::Unit {
        LuminousExposureUnit::LuxSeconds
    }

    fn units() -> &'static [Self::Unit] {
        LuminousExposureUnit::ALL
    }
}

/// Extension trait for creating LuminousExposure quantities from numeric types.
pub trait LuminousExposureConversions {
    /// Creates a LuminousExposure in lux-seconds.
    fn lux_seconds(self) -> LuminousExposure;
}

impl LuminousExposureConversions for f64 {
    fn lux_seconds(self) -> LuminousExposure {
        LuminousExposure::lux_seconds(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luminous_exposure_creation() {
        let e = LuminousExposure::lux_seconds(600.0);
        assert_eq!(e.value(), 600.0);
        assert_eq!(e.unit(), LuminousExposureUnit::LuxSeconds);
    }

    #[test]
    fn test_luminous_exposure_divided_by_time() {
        let e = LuminousExposure::lux_seconds(900.0);
        let t = Time::seconds(3.0);
        let i = e / t;
        // 900 lx·s / 3 s = 300 lx
        assert!((i.to_lux() - 300.0).abs() < 1e-10);
    }

    #[test]
    fn test_illuminance_times_time() {
        let i = Illuminance::lux(150.0);
        let t = Time::seconds(4.0);
        let e = i * t;
        // 150 lx * 4 s = 600 lx·s
        assert!((e.to_lux_seconds() - 600.0).abs() < 1e-10);
    }
}
