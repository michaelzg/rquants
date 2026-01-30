//! Luminous energy quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of luminous energy measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LuminousEnergyUnit {
    /// Lumen-seconds (lm·s) - SI unit
    LumenSeconds,
}

impl LuminousEnergyUnit {
    /// All available luminous energy units.
    pub const ALL: &'static [LuminousEnergyUnit] = &[LuminousEnergyUnit::LumenSeconds];
}

impl fmt::Display for LuminousEnergyUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for LuminousEnergyUnit {
    fn symbol(&self) -> &'static str {
        match self {
            LuminousEnergyUnit::LumenSeconds => "lm·s",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            LuminousEnergyUnit::LumenSeconds => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, LuminousEnergyUnit::LumenSeconds)
    }
}

/// A quantity of luminous energy.
///
/// Luminous energy represents the time integral of luminous flux.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let energy = LuminousEnergy::lumen_seconds(1000.0);
/// let time = Time::seconds(10.0);
///
/// // LuminousEnergy / Time = LuminousFlux
/// let flux = energy / time;
/// assert!((flux.to_lumens() - 100.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct LuminousEnergy {
    value: f64,
    unit: LuminousEnergyUnit,
}

impl LuminousEnergy {
    /// Creates a new LuminousEnergy quantity.
    pub const fn new_const(value: f64, unit: LuminousEnergyUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a LuminousEnergy in lumen-seconds.
    pub fn lumen_seconds(value: f64) -> Self {
        Self::new(value, LuminousEnergyUnit::LumenSeconds)
    }

    // Conversion methods
    /// Converts to lumen-seconds.
    pub fn to_lumen_seconds(&self) -> f64 {
        self.to(LuminousEnergyUnit::LumenSeconds)
    }
}

impl fmt::Display for LuminousEnergy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for LuminousEnergy {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for LuminousEnergy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for LuminousEnergy {
    type Unit = LuminousEnergyUnit;

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

impl Add for LuminousEnergy {
    type Output = LuminousEnergy;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        LuminousEnergy::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for LuminousEnergy {
    type Output = LuminousEnergy;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        LuminousEnergy::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for LuminousEnergy {
    type Output = LuminousEnergy;

    fn mul(self, rhs: f64) -> Self::Output {
        LuminousEnergy::new(self.value * rhs, self.unit)
    }
}

impl Mul<LuminousEnergy> for f64 {
    type Output = LuminousEnergy;

    fn mul(self, rhs: LuminousEnergy) -> Self::Output {
        LuminousEnergy::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for LuminousEnergy {
    type Output = LuminousEnergy;

    fn div(self, rhs: f64) -> Self::Output {
        LuminousEnergy::new(self.value / rhs, self.unit)
    }
}

impl Div<LuminousEnergy> for LuminousEnergy {
    type Output = f64;

    fn div(self, rhs: LuminousEnergy) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for LuminousEnergy {
    type Output = LuminousEnergy;

    fn neg(self) -> Self::Output {
        LuminousEnergy::new(-self.value, self.unit)
    }
}

// Cross-quantity operations
use super::luminous_flux::{LuminousFlux, LuminousFluxUnit};
use crate::time::{Time, TimeUnit};

// LuminousEnergy / Time = LuminousFlux
impl Div<Time> for LuminousEnergy {
    type Output = LuminousFlux;

    fn div(self, rhs: Time) -> Self::Output {
        let lumens = self.to_lumen_seconds() / rhs.to_seconds();
        LuminousFlux::new(lumens, LuminousFluxUnit::Lumens)
    }
}

// LuminousEnergy / LuminousFlux = Time
impl Div<LuminousFlux> for LuminousEnergy {
    type Output = Time;

    fn div(self, rhs: LuminousFlux) -> Self::Output {
        let seconds = self.to_lumen_seconds() / rhs.to_lumens();
        Time::new(seconds, TimeUnit::Seconds)
    }
}

/// Dimension for LuminousEnergy.
pub struct LuminousEnergyDimension;

impl Dimension for LuminousEnergyDimension {
    type Quantity = LuminousEnergy;
    type Unit = LuminousEnergyUnit;

    fn name() -> &'static str {
        "LuminousEnergy"
    }

    fn primary_unit() -> Self::Unit {
        LuminousEnergyUnit::LumenSeconds
    }

    fn si_unit() -> Self::Unit {
        LuminousEnergyUnit::LumenSeconds
    }

    fn units() -> &'static [Self::Unit] {
        LuminousEnergyUnit::ALL
    }
}

/// Extension trait for creating LuminousEnergy quantities from numeric types.
pub trait LuminousEnergyConversions {
    /// Creates a LuminousEnergy in lumen-seconds.
    fn lumen_seconds(self) -> LuminousEnergy;
}

impl LuminousEnergyConversions for f64 {
    fn lumen_seconds(self) -> LuminousEnergy {
        LuminousEnergy::lumen_seconds(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luminous_energy_creation() {
        let e = LuminousEnergy::lumen_seconds(500.0);
        assert_eq!(e.value(), 500.0);
        assert_eq!(e.unit(), LuminousEnergyUnit::LumenSeconds);
    }

    #[test]
    fn test_luminous_energy_divided_by_time() {
        let e = LuminousEnergy::lumen_seconds(1000.0);
        let t = Time::seconds(10.0);
        let f = e / t;
        // 1000 lm·s / 10 s = 100 lm
        assert!((f.to_lumens() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_luminous_flux_times_time() {
        let f = LuminousFlux::lumens(50.0);
        let t = Time::seconds(20.0);
        let e = f * t;
        // 50 lm * 20 s = 1000 lm·s
        assert!((e.to_lumen_seconds() - 1000.0).abs() < 1e-10);
    }
}
