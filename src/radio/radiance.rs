//! Radiance quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Div, Mul};

/// Units of radiance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RadianceUnit {
    /// Watts per steradian per square meter (W/(sr·m²)) - SI unit
    WattsPerSteradianPerSquareMeter,
}

impl RadianceUnit {
    /// All available radiance units.
    pub const ALL: &'static [RadianceUnit] = &[RadianceUnit::WattsPerSteradianPerSquareMeter];
}

impl_unit_display!(RadianceUnit);

impl UnitOfMeasure for RadianceUnit {
    fn symbol(&self) -> &'static str {
        match self {
            RadianceUnit::WattsPerSteradianPerSquareMeter => "W/(sr·m²)",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            RadianceUnit::WattsPerSteradianPerSquareMeter => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, RadianceUnit::WattsPerSteradianPerSquareMeter)
    }
}

/// A quantity of radiance.
///
/// Radiance represents power per unit solid angle per unit area.
/// SI unit: W/(sr·m²)
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
/// assert_eq!(rad.to_watts_per_steradian_per_square_meter(), 100.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Radiance {
    value: f64,
    unit: RadianceUnit,
}

impl Radiance {
    /// Creates a new Radiance quantity.
    pub const fn new_const(value: f64, unit: RadianceUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a Radiance in watts per steradian per square meter.
    pub fn watts_per_steradian_per_square_meter(value: f64) -> Self {
        Self::new(value, RadianceUnit::WattsPerSteradianPerSquareMeter)
    }

    // Conversion methods
    /// Converts to watts per steradian per square meter.
    pub fn to_watts_per_steradian_per_square_meter(&self) -> f64 {
        self.to(RadianceUnit::WattsPerSteradianPerSquareMeter)
    }
}

impl_quantity!(Radiance, RadianceUnit);

// Cross-quantity operations
use super::radiant_intensity::{RadiantIntensity, RadiantIntensityUnit};
use crate::space::{Area, AreaUnit};

// Radiance * Area = RadiantIntensity
impl Mul<Area> for Radiance {
    type Output = RadiantIntensity;

    fn mul(self, rhs: Area) -> Self::Output {
        let wsr = self.to_watts_per_steradian_per_square_meter() * rhs.to_square_meters();
        RadiantIntensity::new(wsr, RadiantIntensityUnit::WattsPerSteradian)
    }
}

// Area * Radiance = RadiantIntensity
impl Mul<Radiance> for Area {
    type Output = RadiantIntensity;

    fn mul(self, rhs: Radiance) -> Self::Output {
        let wsr = rhs.to_watts_per_steradian_per_square_meter() * self.to_square_meters();
        RadiantIntensity::new(wsr, RadiantIntensityUnit::WattsPerSteradian)
    }
}

// RadiantIntensity / Radiance = Area
impl Div<Radiance> for RadiantIntensity {
    type Output = Area;

    fn div(self, rhs: Radiance) -> Self::Output {
        let m2 = self.to_watts_per_steradian() / rhs.to_watts_per_steradian_per_square_meter();
        Area::new(m2, AreaUnit::SquareMeters)
    }
}

impl_dimension!(
    RadianceDimension,
    Radiance,
    RadianceUnit,
    "Radiance",
    RadianceUnit::WattsPerSteradianPerSquareMeter,
    RadianceUnit::WattsPerSteradianPerSquareMeter
);

/// Extension trait for creating Radiance quantities from numeric types.
pub trait RadianceConversions {
    /// Creates a Radiance in watts per steradian per square meter.
    fn watts_per_steradian_per_square_meter(self) -> Radiance;
}

impl RadianceConversions for f64 {
    fn watts_per_steradian_per_square_meter(self) -> Radiance {
        Radiance::watts_per_steradian_per_square_meter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radiance_creation() {
        let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
        assert_eq!(rad.value(), 100.0);
        assert_eq!(rad.unit(), RadianceUnit::WattsPerSteradianPerSquareMeter);
    }

    #[test]
    fn test_radiance_conversions() {
        let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
        assert_eq!(rad.to_watts_per_steradian_per_square_meter(), 100.0);
    }

    #[test]
    fn test_radiance_times_area() {
        let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
        let area = Area::square_meters(2.0);
        let ri = rad * area;
        assert_eq!(ri.to_watts_per_steradian(), 200.0);
    }

    #[test]
    fn test_radiant_intensity_divided_by_radiance() {
        let ri = RadiantIntensity::watts_per_steradian(200.0);
        let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
        let area = ri / rad;
        assert_eq!(area.to_square_meters(), 2.0);
    }
}
