//! Thermal capacity (entropy) quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of thermal capacity measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThermalCapacityUnit {
    /// Joules per kelvin (J/K) - SI unit
    JoulesPerKelvin,
}

impl ThermalCapacityUnit {
    /// All available thermal capacity units.
    pub const ALL: &'static [ThermalCapacityUnit] = &[ThermalCapacityUnit::JoulesPerKelvin];
}

impl fmt::Display for ThermalCapacityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for ThermalCapacityUnit {
    fn symbol(&self) -> &'static str {
        match self {
            ThermalCapacityUnit::JoulesPerKelvin => "J/K",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            ThermalCapacityUnit::JoulesPerKelvin => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        true
    }
}

/// A quantity of thermal capacity (also represents entropy).
///
/// Thermal capacity represents the ability of a substance to store thermal energy
/// per unit of temperature change.
///
/// # Example
///
/// ```rust
/// use rquants::thermal::thermal_capacity::{ThermalCapacity, ThermalCapacityUnit};
/// use rquants::thermal::temperature::Temperature;
/// use rquants::core::Quantity;
///
/// let tc = ThermalCapacity::joules_per_kelvin(4186.0); // water, ~1 kg
/// let temp = Temperature::kelvin(300.0);
///
/// // Energy = ThermalCapacity * Temperature
/// let energy = tc * temp;
/// assert!((energy.to_joules() - 1_255_800.0).abs() < 1.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ThermalCapacity {
    value: f64,
    unit: ThermalCapacityUnit,
}

impl ThermalCapacity {
    /// Creates a new ThermalCapacity quantity.
    pub const fn new_const(value: f64, unit: ThermalCapacityUnit) -> Self {
        Self { value, unit }
    }

    /// Creates a ThermalCapacity in joules per kelvin.
    pub fn joules_per_kelvin(value: f64) -> Self {
        Self::new(value, ThermalCapacityUnit::JoulesPerKelvin)
    }

    /// Converts to joules per kelvin.
    pub fn to_joules_per_kelvin(&self) -> f64 {
        self.to(ThermalCapacityUnit::JoulesPerKelvin)
    }
}

impl fmt::Display for ThermalCapacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for ThermalCapacity {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for ThermalCapacity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for ThermalCapacity {
    type Unit = ThermalCapacityUnit;

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

impl Add for ThermalCapacity {
    type Output = ThermalCapacity;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        ThermalCapacity::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for ThermalCapacity {
    type Output = ThermalCapacity;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        ThermalCapacity::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for ThermalCapacity {
    type Output = ThermalCapacity;

    fn mul(self, rhs: f64) -> Self::Output {
        ThermalCapacity::new(self.value * rhs, self.unit)
    }
}

impl Mul<ThermalCapacity> for f64 {
    type Output = ThermalCapacity;

    fn mul(self, rhs: ThermalCapacity) -> Self::Output {
        ThermalCapacity::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for ThermalCapacity {
    type Output = ThermalCapacity;

    fn div(self, rhs: f64) -> Self::Output {
        ThermalCapacity::new(self.value / rhs, self.unit)
    }
}

impl Div<ThermalCapacity> for ThermalCapacity {
    type Output = f64;

    fn div(self, rhs: ThermalCapacity) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for ThermalCapacity {
    type Output = ThermalCapacity;

    fn neg(self) -> Self::Output {
        ThermalCapacity::new(-self.value, self.unit)
    }
}

// Cross-quantity: ThermalCapacity * Temperature = Energy
use super::temperature::Temperature;
use crate::energy::{Energy, EnergyUnit};

impl Mul<Temperature> for ThermalCapacity {
    type Output = Energy;

    fn mul(self, rhs: Temperature) -> Self::Output {
        let joules = self.to_joules_per_kelvin() * rhs.to_kelvin_scale();
        Energy::new(joules, EnergyUnit::Joules)
    }
}

/// Dimension for ThermalCapacity.
pub struct ThermalCapacityDimension;

impl Dimension for ThermalCapacityDimension {
    type Quantity = ThermalCapacity;
    type Unit = ThermalCapacityUnit;

    fn name() -> &'static str {
        "ThermalCapacity"
    }

    fn primary_unit() -> Self::Unit {
        ThermalCapacityUnit::JoulesPerKelvin
    }

    fn si_unit() -> Self::Unit {
        ThermalCapacityUnit::JoulesPerKelvin
    }

    fn units() -> &'static [Self::Unit] {
        ThermalCapacityUnit::ALL
    }
}

/// Extension trait for creating ThermalCapacity quantities from numeric types.
pub trait ThermalCapacityConversions {
    /// Creates a ThermalCapacity in joules per kelvin.
    fn joules_per_kelvin(self) -> ThermalCapacity;
}

impl ThermalCapacityConversions for f64 {
    fn joules_per_kelvin(self) -> ThermalCapacity {
        ThermalCapacity::joules_per_kelvin(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_thermal_capacity_creation() {
        let tc = ThermalCapacity::joules_per_kelvin(100.0);
        assert_eq!(tc.value(), 100.0);
        assert_eq!(tc.unit(), ThermalCapacityUnit::JoulesPerKelvin);
    }

    #[test]
    fn test_thermal_capacity_times_temperature() {
        let tc = ThermalCapacity::joules_per_kelvin(100.0);
        let t = Temperature::kelvin(300.0);
        let e = tc * t;
        assert!((e.to_joules() - 30000.0).abs() < 1e-10);
    }

    #[test]
    fn test_temperature_times_thermal_capacity() {
        let t = Temperature::kelvin(300.0);
        let tc = ThermalCapacity::joules_per_kelvin(100.0);
        let e = t * tc;
        assert!((e.to_joules() - 30000.0).abs() < 1e-10);
    }
}
