//! Radiant intensity quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Div, Mul};

/// Units of radiant intensity measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RadiantIntensityUnit {
    /// Watts per steradian (W/sr) - SI unit
    WattsPerSteradian,
}

impl RadiantIntensityUnit {
    /// All available radiant intensity units.
    pub const ALL: &'static [RadiantIntensityUnit] = &[RadiantIntensityUnit::WattsPerSteradian];
}

impl_unit_display!(RadiantIntensityUnit);

impl UnitOfMeasure for RadiantIntensityUnit {
    fn symbol(&self) -> &'static str {
        match self {
            RadiantIntensityUnit::WattsPerSteradian => "W/sr",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            RadiantIntensityUnit::WattsPerSteradian => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, RadiantIntensityUnit::WattsPerSteradian)
    }
}

/// A quantity of radiant intensity.
///
/// Radiant intensity represents power per unit solid angle.
/// SI unit: W/sr
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let ri = RadiantIntensity::watts_per_steradian(100.0);
/// let angle = SolidAngle::steradians(2.0);
///
/// // RadiantIntensity * SolidAngle = Power
/// let power = ri * angle;
/// assert_eq!(power.to_watts(), 200.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RadiantIntensity {
    value: f64,
    unit: RadiantIntensityUnit,
}

impl RadiantIntensity {
    /// Creates a new RadiantIntensity quantity.
    pub const fn new_const(value: f64, unit: RadiantIntensityUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a RadiantIntensity in watts per steradian.
    pub fn watts_per_steradian(value: f64) -> Self {
        Self::new(value, RadiantIntensityUnit::WattsPerSteradian)
    }

    // Conversion methods
    /// Converts to watts per steradian.
    pub fn to_watts_per_steradian(&self) -> f64 {
        self.to(RadiantIntensityUnit::WattsPerSteradian)
    }
}

impl_quantity!(RadiantIntensity, RadiantIntensityUnit);

// Cross-quantity operations
use crate::energy::{Power, PowerUnit};
use crate::space::{SolidAngle, SolidAngleUnit};

// RadiantIntensity * SolidAngle = Power
impl Mul<SolidAngle> for RadiantIntensity {
    type Output = Power;

    fn mul(self, rhs: SolidAngle) -> Self::Output {
        let watts = self.to_watts_per_steradian() * rhs.to_steradians();
        Power::new(watts, PowerUnit::Watts)
    }
}

// SolidAngle * RadiantIntensity = Power
impl Mul<RadiantIntensity> for SolidAngle {
    type Output = Power;

    fn mul(self, rhs: RadiantIntensity) -> Self::Output {
        let watts = rhs.to_watts_per_steradian() * self.to_steradians();
        Power::new(watts, PowerUnit::Watts)
    }
}

// Power / RadiantIntensity = SolidAngle
impl Div<RadiantIntensity> for Power {
    type Output = SolidAngle;

    fn div(self, rhs: RadiantIntensity) -> Self::Output {
        let sr = self.to_watts() / rhs.to_watts_per_steradian();
        SolidAngle::new(sr, SolidAngleUnit::Steradians)
    }
}

impl_dimension!(
    RadiantIntensityDimension,
    RadiantIntensity,
    RadiantIntensityUnit,
    "RadiantIntensity",
    RadiantIntensityUnit::WattsPerSteradian,
    RadiantIntensityUnit::WattsPerSteradian
);

/// Extension trait for creating RadiantIntensity quantities from numeric types.
pub trait RadiantIntensityConversions {
    /// Creates a RadiantIntensity in watts per steradian.
    fn watts_per_steradian(self) -> RadiantIntensity;
}

impl RadiantIntensityConversions for f64 {
    fn watts_per_steradian(self) -> RadiantIntensity {
        RadiantIntensity::watts_per_steradian(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radiant_intensity_creation() {
        let ri = RadiantIntensity::watts_per_steradian(100.0);
        assert_eq!(ri.value(), 100.0);
        assert_eq!(ri.unit(), RadiantIntensityUnit::WattsPerSteradian);
    }

    #[test]
    fn test_radiant_intensity_conversions() {
        let ri = RadiantIntensity::watts_per_steradian(100.0);
        assert_eq!(ri.to_watts_per_steradian(), 100.0);
    }

    #[test]
    fn test_radiant_intensity_times_solid_angle() {
        let ri = RadiantIntensity::watts_per_steradian(100.0);
        let angle = SolidAngle::steradians(2.0);
        let power = ri * angle;
        assert_eq!(power.to_watts(), 200.0);
    }

    #[test]
    fn test_power_divided_by_radiant_intensity() {
        let power = Power::watts(200.0);
        let ri = RadiantIntensity::watts_per_steradian(100.0);
        let angle = power / ri;
        assert_eq!(angle.to_steradians(), 2.0);
    }
}
