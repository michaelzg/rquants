//! Spectral irradiance quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};

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

impl_unit_display!(SpectralIrradianceUnit);

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

impl_quantity!(SpectralIrradiance, SpectralIrradianceUnit);

impl_dimension!(
    SpectralIrradianceDimension,
    SpectralIrradiance,
    SpectralIrradianceUnit,
    "SpectralIrradiance",
    SpectralIrradianceUnit::WattsPerCubicMeter,
    SpectralIrradianceUnit::WattsPerCubicMeter
);

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
