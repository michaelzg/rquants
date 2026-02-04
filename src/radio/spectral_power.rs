//! Spectral power quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Div, Mul};

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

impl_unit_display!(SpectralPowerUnit);

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

impl_quantity!(SpectralPower, SpectralPowerUnit);

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

impl_dimension!(
    SpectralPowerDimension,
    SpectralPower,
    SpectralPowerUnit,
    "SpectralPower",
    SpectralPowerUnit::WattsPerMeter,
    SpectralPowerUnit::WattsPerMeter
);

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
