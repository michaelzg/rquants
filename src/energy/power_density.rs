//! Power density quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Mul};

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

impl_unit_display!(PowerDensityUnit);

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

impl_quantity!(PowerDensity, PowerDensityUnit);

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

impl_dimension!(
    PowerDensityDimension,
    PowerDensity,
    PowerDensityUnit,
    "PowerDensity",
    PowerDensityUnit::WattsPerCubicMeter,
    PowerDensityUnit::WattsPerCubicMeter
);

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
