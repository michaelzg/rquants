//! Energy density quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of energy density measurement (energy per unit volume).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnergyDensityUnit {
    /// Joules per cubic meter (J/m³) - SI unit
    JoulesPerCubicMeter,
}

impl EnergyDensityUnit {
    /// All available energy density units.
    pub const ALL: &'static [EnergyDensityUnit] = &[EnergyDensityUnit::JoulesPerCubicMeter];
}

impl fmt::Display for EnergyDensityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for EnergyDensityUnit {
    fn symbol(&self) -> &'static str {
        match self {
            EnergyDensityUnit::JoulesPerCubicMeter => "J/m³",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            EnergyDensityUnit::JoulesPerCubicMeter => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        true
    }
}

/// A quantity of energy density (energy per unit volume).
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let ed = EnergyDensity::joules_per_cubic_meter(1000.0);
/// let volume = Volume::cubic_meters(2.0);
///
/// // Energy = EnergyDensity * Volume
/// let energy = ed * volume;
/// assert!((energy.to_joules() - 2000.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct EnergyDensity {
    value: f64,
    unit: EnergyDensityUnit,
}

impl EnergyDensity {
    /// Creates a new EnergyDensity quantity.
    pub const fn new_const(value: f64, unit: EnergyDensityUnit) -> Self {
        Self { value, unit }
    }

    /// Creates an EnergyDensity in joules per cubic meter.
    pub fn joules_per_cubic_meter(value: f64) -> Self {
        Self::new(value, EnergyDensityUnit::JoulesPerCubicMeter)
    }

    /// Converts to joules per cubic meter.
    pub fn to_joules_per_cubic_meter(&self) -> f64 {
        self.to(EnergyDensityUnit::JoulesPerCubicMeter)
    }
}

impl fmt::Display for EnergyDensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for EnergyDensity {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for EnergyDensity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for EnergyDensity {
    type Unit = EnergyDensityUnit;

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

impl Add for EnergyDensity {
    type Output = EnergyDensity;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        EnergyDensity::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for EnergyDensity {
    type Output = EnergyDensity;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        EnergyDensity::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for EnergyDensity {
    type Output = EnergyDensity;

    fn mul(self, rhs: f64) -> Self::Output {
        EnergyDensity::new(self.value * rhs, self.unit)
    }
}

impl Mul<EnergyDensity> for f64 {
    type Output = EnergyDensity;

    fn mul(self, rhs: EnergyDensity) -> Self::Output {
        EnergyDensity::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for EnergyDensity {
    type Output = EnergyDensity;

    fn div(self, rhs: f64) -> Self::Output {
        EnergyDensity::new(self.value / rhs, self.unit)
    }
}

impl Div<EnergyDensity> for EnergyDensity {
    type Output = f64;

    fn div(self, rhs: EnergyDensity) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for EnergyDensity {
    type Output = EnergyDensity;

    fn neg(self) -> Self::Output {
        EnergyDensity::new(-self.value, self.unit)
    }
}

// Cross-quantity operations
use super::energy::{Energy, EnergyUnit};
use crate::space::Volume;

// EnergyDensity * Volume = Energy
impl Mul<Volume> for EnergyDensity {
    type Output = Energy;

    fn mul(self, rhs: Volume) -> Self::Output {
        let joules = self.to_joules_per_cubic_meter() * rhs.to_cubic_meters();
        Energy::new(joules, EnergyUnit::Joules)
    }
}

// Volume * EnergyDensity = Energy
impl Mul<EnergyDensity> for Volume {
    type Output = Energy;

    fn mul(self, rhs: EnergyDensity) -> Self::Output {
        let joules = rhs.to_joules_per_cubic_meter() * self.to_cubic_meters();
        Energy::new(joules, EnergyUnit::Joules)
    }
}

/// Dimension for EnergyDensity.
pub struct EnergyDensityDimension;

impl Dimension for EnergyDensityDimension {
    type Quantity = EnergyDensity;
    type Unit = EnergyDensityUnit;

    fn name() -> &'static str {
        "EnergyDensity"
    }

    fn primary_unit() -> Self::Unit {
        EnergyDensityUnit::JoulesPerCubicMeter
    }

    fn si_unit() -> Self::Unit {
        EnergyDensityUnit::JoulesPerCubicMeter
    }

    fn units() -> &'static [Self::Unit] {
        EnergyDensityUnit::ALL
    }
}

/// Extension trait for creating EnergyDensity quantities from numeric types.
pub trait EnergyDensityConversions {
    /// Creates an EnergyDensity in joules per cubic meter.
    fn joules_per_cubic_meter(self) -> EnergyDensity;
}

impl EnergyDensityConversions for f64 {
    fn joules_per_cubic_meter(self) -> EnergyDensity {
        EnergyDensity::joules_per_cubic_meter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_density_creation() {
        let ed = EnergyDensity::joules_per_cubic_meter(1000.0);
        assert_eq!(ed.value(), 1000.0);
        assert_eq!(ed.unit(), EnergyDensityUnit::JoulesPerCubicMeter);
    }

    #[test]
    fn test_energy_density_times_volume() {
        let ed = EnergyDensity::joules_per_cubic_meter(500.0);
        let v = Volume::cubic_meters(4.0);
        let e = ed * v;
        assert!((e.to_joules() - 2000.0).abs() < 1e-10);
    }
}
