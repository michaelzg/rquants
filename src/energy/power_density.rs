//! Power density quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of power density measurement (power per unit volume).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PowerDensityUnit {
    /// Watts per cubic meter (W/m³) - SI unit
    WattsPerCubicMeter,
}

impl PowerDensityUnit {
    /// All available power density units.
    pub const ALL: &'static [PowerDensityUnit] = &[PowerDensityUnit::WattsPerCubicMeter];
}

impl fmt::Display for PowerDensityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for PowerDensityUnit {
    fn symbol(&self) -> &'static str {
        match self {
            PowerDensityUnit::WattsPerCubicMeter => "W/m³",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            PowerDensityUnit::WattsPerCubicMeter => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        true
    }
}

/// A quantity of power density (power per unit volume).
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let pd = PowerDensity::watts_per_cubic_meter(500.0);
/// let volume = Volume::cubic_meters(2.0);
///
/// // Power = PowerDensity * Volume
/// let power = pd * volume;
/// assert!((power.to_watts() - 1000.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PowerDensity {
    value: f64,
    unit: PowerDensityUnit,
}

impl PowerDensity {
    /// Creates a new PowerDensity quantity.
    pub const fn new_const(value: f64, unit: PowerDensityUnit) -> Self {
        Self { value, unit }
    }

    /// Creates a PowerDensity in watts per cubic meter.
    pub fn watts_per_cubic_meter(value: f64) -> Self {
        Self::new(value, PowerDensityUnit::WattsPerCubicMeter)
    }

    /// Converts to watts per cubic meter.
    pub fn to_watts_per_cubic_meter(&self) -> f64 {
        self.to(PowerDensityUnit::WattsPerCubicMeter)
    }
}

impl fmt::Display for PowerDensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for PowerDensity {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for PowerDensity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for PowerDensity {
    type Unit = PowerDensityUnit;

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

impl Add for PowerDensity {
    type Output = PowerDensity;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        PowerDensity::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for PowerDensity {
    type Output = PowerDensity;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        PowerDensity::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for PowerDensity {
    type Output = PowerDensity;

    fn mul(self, rhs: f64) -> Self::Output {
        PowerDensity::new(self.value * rhs, self.unit)
    }
}

impl Mul<PowerDensity> for f64 {
    type Output = PowerDensity;

    fn mul(self, rhs: PowerDensity) -> Self::Output {
        PowerDensity::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for PowerDensity {
    type Output = PowerDensity;

    fn div(self, rhs: f64) -> Self::Output {
        PowerDensity::new(self.value / rhs, self.unit)
    }
}

impl Div<PowerDensity> for PowerDensity {
    type Output = f64;

    fn div(self, rhs: PowerDensity) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for PowerDensity {
    type Output = PowerDensity;

    fn neg(self) -> Self::Output {
        PowerDensity::new(-self.value, self.unit)
    }
}

// Cross-quantity operations
use super::power::{Power, PowerUnit};
use crate::space::Volume;

// PowerDensity * Volume = Power
impl Mul<Volume> for PowerDensity {
    type Output = Power;

    fn mul(self, rhs: Volume) -> Self::Output {
        let watts = self.to_watts_per_cubic_meter() * rhs.to_cubic_meters();
        Power::new(watts, PowerUnit::Watts)
    }
}

// Volume * PowerDensity = Power
impl Mul<PowerDensity> for Volume {
    type Output = Power;

    fn mul(self, rhs: PowerDensity) -> Self::Output {
        let watts = rhs.to_watts_per_cubic_meter() * self.to_cubic_meters();
        Power::new(watts, PowerUnit::Watts)
    }
}

/// Dimension for PowerDensity.
pub struct PowerDensityDimension;

impl Dimension for PowerDensityDimension {
    type Quantity = PowerDensity;
    type Unit = PowerDensityUnit;

    fn name() -> &'static str {
        "PowerDensity"
    }

    fn primary_unit() -> Self::Unit {
        PowerDensityUnit::WattsPerCubicMeter
    }

    fn si_unit() -> Self::Unit {
        PowerDensityUnit::WattsPerCubicMeter
    }

    fn units() -> &'static [Self::Unit] {
        PowerDensityUnit::ALL
    }
}

/// Extension trait for creating PowerDensity quantities from numeric types.
pub trait PowerDensityConversions {
    /// Creates a PowerDensity in watts per cubic meter.
    fn watts_per_cubic_meter(self) -> PowerDensity;
}

impl PowerDensityConversions for f64 {
    fn watts_per_cubic_meter(self) -> PowerDensity {
        PowerDensity::watts_per_cubic_meter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_density_creation() {
        let pd = PowerDensity::watts_per_cubic_meter(100.0);
        assert_eq!(pd.value(), 100.0);
        assert_eq!(pd.unit(), PowerDensityUnit::WattsPerCubicMeter);
    }

    #[test]
    fn test_power_density_times_volume() {
        let pd = PowerDensity::watts_per_cubic_meter(500.0);
        let v = Volume::cubic_meters(2.0);
        let p = pd * v;
        assert!((p.to_watts() - 1000.0).abs() < 1e-10);
    }
}
