//! Solid angle quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::f64::consts::PI;

/// Units of solid angle measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SolidAngleUnit {
    /// Steradians (sr) - SI unit
    Steradians,
    /// Square degrees
    SquareDegrees,
    /// Spheres (complete sphere = 4π steradians)
    Spheres,
}

impl SolidAngleUnit {
    /// All available solid angle units.
    pub const ALL: &'static [SolidAngleUnit] = &[
        SolidAngleUnit::Steradians,
        SolidAngleUnit::SquareDegrees,
        SolidAngleUnit::Spheres,
    ];
}

impl_unit_display!(SolidAngleUnit);

impl UnitOfMeasure for SolidAngleUnit {
    fn symbol(&self) -> &'static str {
        match self {
            SolidAngleUnit::Steradians => "sr",
            SolidAngleUnit::SquareDegrees => "deg²",
            SolidAngleUnit::Spheres => "sphere",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            SolidAngleUnit::Steradians => 1.0,
            SolidAngleUnit::SquareDegrees => (PI / 180.0) * (PI / 180.0),
            SolidAngleUnit::Spheres => 4.0 * PI,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, SolidAngleUnit::Steradians)
    }
}

/// A quantity of solid angle.
///
/// Solid angle represents a two-dimensional angle subtended at a point.
/// A full sphere subtends 4π steradians.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
/// use std::f64::consts::PI;
///
/// let sa = SolidAngle::spheres(1.0);
/// assert!((sa.to_steradians() - 4.0 * PI).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SolidAngle {
    value: f64,
    unit: SolidAngleUnit,
}

impl SolidAngle {
    /// Creates a new SolidAngle quantity.
    pub const fn new_const(value: f64, unit: SolidAngleUnit) -> Self {
        Self { value, unit }
    }

    /// Creates a SolidAngle in steradians.
    pub fn steradians(value: f64) -> Self {
        Self::new(value, SolidAngleUnit::Steradians)
    }

    /// Creates a SolidAngle in square degrees.
    pub fn square_degrees(value: f64) -> Self {
        Self::new(value, SolidAngleUnit::SquareDegrees)
    }

    /// Creates a SolidAngle in spheres (4π steradians).
    pub fn spheres(value: f64) -> Self {
        Self::new(value, SolidAngleUnit::Spheres)
    }

    // Conversion methods
    /// Converts to steradians.
    pub fn to_steradians(&self) -> f64 {
        self.to(SolidAngleUnit::Steradians)
    }

    /// Converts to square degrees.
    pub fn to_square_degrees(&self) -> f64 {
        self.to(SolidAngleUnit::SquareDegrees)
    }

    /// Converts to spheres.
    pub fn to_spheres(&self) -> f64 {
        self.to(SolidAngleUnit::Spheres)
    }
}

impl_quantity!(SolidAngle, SolidAngleUnit);

impl_dimension!(
    SolidAngleDimension,
    SolidAngle,
    SolidAngleUnit,
    "SolidAngle",
    SolidAngleUnit::Steradians,
    SolidAngleUnit::Steradians
);

/// Extension trait for creating SolidAngle quantities from numeric types.
pub trait SolidAngleConversions {
    /// Creates a SolidAngle in steradians.
    fn steradians(self) -> SolidAngle;
    /// Creates a SolidAngle in square degrees.
    fn square_degrees(self) -> SolidAngle;
    /// Creates a SolidAngle in spheres.
    fn spheres(self) -> SolidAngle;
}

impl SolidAngleConversions for f64 {
    fn steradians(self) -> SolidAngle {
        SolidAngle::steradians(self)
    }
    fn square_degrees(self) -> SolidAngle {
        SolidAngle::square_degrees(self)
    }
    fn spheres(self) -> SolidAngle {
        SolidAngle::spheres(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_angle_creation() {
        let sa = SolidAngle::steradians(1.0);
        assert_eq!(sa.value(), 1.0);
        assert_eq!(sa.unit(), SolidAngleUnit::Steradians);
    }

    #[test]
    fn test_solid_angle_conversions() {
        let sa = SolidAngle::spheres(1.0);
        assert!((sa.to_steradians() - 4.0 * PI).abs() < 1e-10);
    }

    #[test]
    fn test_solid_angle_arithmetic() {
        let sa1 = SolidAngle::steradians(1.0);
        let sa2 = SolidAngle::steradians(1.0);
        let sum = sa1 + sa2;
        assert_eq!(sum.to_steradians(), 2.0);
    }
}
