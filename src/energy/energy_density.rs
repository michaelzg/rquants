//! Energy density quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Mul};

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

impl_unit_display!(EnergyDensityUnit);

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

impl_quantity!(EnergyDensity, EnergyDensityUnit);

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

impl_dimension!(
    EnergyDensityDimension,
    EnergyDensity,
    EnergyDensityUnit,
    "EnergyDensity",
    EnergyDensityUnit::JoulesPerCubicMeter,
    EnergyDensityUnit::JoulesPerCubicMeter
);

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
