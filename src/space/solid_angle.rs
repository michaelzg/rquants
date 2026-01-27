//! Solid angle quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

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

impl fmt::Display for SolidAngleUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

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

impl fmt::Display for SolidAngle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for SolidAngle {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for SolidAngle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for SolidAngle {
    type Unit = SolidAngleUnit;

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

// Arithmetic operations

impl Add for SolidAngle {
    type Output = SolidAngle;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        SolidAngle::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for SolidAngle {
    type Output = SolidAngle;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        SolidAngle::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for SolidAngle {
    type Output = SolidAngle;

    fn mul(self, rhs: f64) -> Self::Output {
        SolidAngle::new(self.value * rhs, self.unit)
    }
}

impl Mul<SolidAngle> for f64 {
    type Output = SolidAngle;

    fn mul(self, rhs: SolidAngle) -> Self::Output {
        SolidAngle::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for SolidAngle {
    type Output = SolidAngle;

    fn div(self, rhs: f64) -> Self::Output {
        SolidAngle::new(self.value / rhs, self.unit)
    }
}

impl Div<SolidAngle> for SolidAngle {
    type Output = f64;

    fn div(self, rhs: SolidAngle) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for SolidAngle {
    type Output = SolidAngle;

    fn neg(self) -> Self::Output {
        SolidAngle::new(-self.value, self.unit)
    }
}

/// Dimension for SolidAngle.
pub struct SolidAngleDimension;

impl Dimension for SolidAngleDimension {
    type Quantity = SolidAngle;
    type Unit = SolidAngleUnit;

    fn name() -> &'static str {
        "SolidAngle"
    }

    fn primary_unit() -> Self::Unit {
        SolidAngleUnit::Steradians
    }

    fn si_unit() -> Self::Unit {
        SolidAngleUnit::Steradians
    }

    fn units() -> &'static [Self::Unit] {
        SolidAngleUnit::ALL
    }
}

/// Extension trait for creating SolidAngle quantities from numeric types.
pub trait SolidAngleConversions {
    /// Creates a SolidAngle in steradians.
    fn steradians(self) -> SolidAngle;
    /// Creates a SolidAngle in spheres.
    fn spheres(self) -> SolidAngle;
}

impl SolidAngleConversions for f64 {
    fn steradians(self) -> SolidAngle {
        SolidAngle::steradians(self)
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
