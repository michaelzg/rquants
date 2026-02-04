//! Angle quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::f64::consts::PI;

/// Units of angle measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AngleUnit {
    /// Radians (rad) - SI unit
    Radians,
    /// Degrees (°)
    Degrees,
    /// Gradians/Gons (gon)
    Gradians,
    /// Turns (complete rotations)
    Turns,
    /// Arc minutes (')
    ArcMinutes,
    /// Arc seconds ('')
    ArcSeconds,
}

impl AngleUnit {
    /// All available angle units.
    pub const ALL: &'static [AngleUnit] = &[
        AngleUnit::Radians,
        AngleUnit::Degrees,
        AngleUnit::Gradians,
        AngleUnit::Turns,
        AngleUnit::ArcMinutes,
        AngleUnit::ArcSeconds,
    ];
}

impl_unit_display!(AngleUnit);

impl UnitOfMeasure for AngleUnit {
    fn symbol(&self) -> &'static str {
        match self {
            AngleUnit::Radians => "rad",
            AngleUnit::Degrees => "°",
            AngleUnit::Gradians => "gon",
            AngleUnit::Turns => "tr",
            AngleUnit::ArcMinutes => "'",
            AngleUnit::ArcSeconds => "''",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            AngleUnit::Radians => 1.0,
            AngleUnit::Degrees => PI / 180.0,
            AngleUnit::Gradians => PI / 200.0,
            AngleUnit::Turns => 2.0 * PI,
            AngleUnit::ArcMinutes => PI / (180.0 * 60.0),
            AngleUnit::ArcSeconds => PI / (180.0 * 3600.0),
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, AngleUnit::Radians)
    }
}

/// A quantity of angle.
///
/// Angle represents a plane angle measurement.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
/// use std::f64::consts::PI;
///
/// let a1 = Angle::radians(PI);
/// let a2 = Angle::degrees(180.0);
///
/// // These represent the same angle
/// assert!((a1.to_radians() - a2.to_radians()).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Angle {
    value: f64,
    unit: AngleUnit,
}

impl Angle {
    /// Creates a new Angle quantity.
    pub const fn new_const(value: f64, unit: AngleUnit) -> Self {
        Self { value, unit }
    }

    /// Creates an Angle in radians.
    pub fn radians(value: f64) -> Self {
        Self::new(value, AngleUnit::Radians)
    }

    /// Creates an Angle in degrees.
    pub fn degrees(value: f64) -> Self {
        Self::new(value, AngleUnit::Degrees)
    }

    /// Creates an Angle in gradians.
    pub fn gradians(value: f64) -> Self {
        Self::new(value, AngleUnit::Gradians)
    }

    /// Creates an Angle in turns.
    pub fn turns(value: f64) -> Self {
        Self::new(value, AngleUnit::Turns)
    }

    /// Creates an Angle in arc minutes.
    pub fn arc_minutes(value: f64) -> Self {
        Self::new(value, AngleUnit::ArcMinutes)
    }

    /// Creates an Angle in arc seconds.
    pub fn arc_seconds(value: f64) -> Self {
        Self::new(value, AngleUnit::ArcSeconds)
    }

    // Conversion methods
    /// Converts to radians.
    pub fn to_radians(&self) -> f64 {
        self.to(AngleUnit::Radians)
    }

    /// Converts to degrees.
    pub fn to_degrees(&self) -> f64 {
        self.to(AngleUnit::Degrees)
    }

    /// Converts to gradians.
    pub fn to_gradians(&self) -> f64 {
        self.to(AngleUnit::Gradians)
    }

    /// Converts to turns.
    pub fn to_turns(&self) -> f64 {
        self.to(AngleUnit::Turns)
    }

    /// Converts to arc minutes.
    pub fn to_arc_minutes(&self) -> f64 {
        self.to(AngleUnit::ArcMinutes)
    }

    /// Converts to arc seconds.
    pub fn to_arc_seconds(&self) -> f64 {
        self.to(AngleUnit::ArcSeconds)
    }

    // Trigonometric functions
    /// Returns the sine of this angle.
    pub fn sin(&self) -> f64 {
        self.to_radians().sin()
    }

    /// Returns the cosine of this angle.
    pub fn cos(&self) -> f64 {
        self.to_radians().cos()
    }

    /// Returns the tangent of this angle.
    pub fn tan(&self) -> f64 {
        self.to_radians().tan()
    }

    /// Creates an angle from its sine value.
    pub fn asin(value: f64) -> Self {
        Self::radians(value.asin())
    }

    /// Creates an angle from its cosine value.
    pub fn acos(value: f64) -> Self {
        Self::radians(value.acos())
    }

    /// Creates an angle from its tangent value.
    pub fn atan(value: f64) -> Self {
        Self::radians(value.atan())
    }

    /// Creates an angle from atan2(y, x).
    pub fn atan2(y: f64, x: f64) -> Self {
        Self::radians(y.atan2(x))
    }
}

impl_quantity!(Angle, AngleUnit);

impl_dimension!(
    AngleDimension,
    Angle,
    AngleUnit,
    "Angle",
    AngleUnit::Radians,
    AngleUnit::Radians
);

/// Extension trait for creating Angle quantities from numeric types.
pub trait AngleConversions {
    /// Creates an Angle in radians.
    fn radians(self) -> Angle;
    /// Creates an Angle in degrees.
    fn degrees(self) -> Angle;
    /// Creates an Angle in gradians.
    fn gradians(self) -> Angle;
    /// Creates an Angle in turns.
    fn turns(self) -> Angle;
    /// Creates an Angle in arc minutes.
    fn arc_minutes(self) -> Angle;
    /// Creates an Angle in arc seconds.
    fn arc_seconds(self) -> Angle;
}

impl AngleConversions for f64 {
    fn radians(self) -> Angle {
        Angle::radians(self)
    }
    fn degrees(self) -> Angle {
        Angle::degrees(self)
    }
    fn gradians(self) -> Angle {
        Angle::gradians(self)
    }
    fn turns(self) -> Angle {
        Angle::turns(self)
    }
    fn arc_minutes(self) -> Angle {
        Angle::arc_minutes(self)
    }
    fn arc_seconds(self) -> Angle {
        Angle::arc_seconds(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_creation() {
        let a = Angle::degrees(90.0);
        assert_eq!(a.value(), 90.0);
        assert_eq!(a.unit(), AngleUnit::Degrees);
    }

    #[test]
    fn test_angle_conversions() {
        let a = Angle::degrees(180.0);
        assert!((a.to_radians() - PI).abs() < 1e-10);

        let a2 = Angle::turns(1.0);
        assert!((a2.to_degrees() - 360.0).abs() < 1e-10);
    }

    #[test]
    fn test_trig_functions() {
        let a = Angle::degrees(90.0);
        assert!((a.sin() - 1.0).abs() < 1e-10);
        assert!(a.cos().abs() < 1e-10);

        let a2 = Angle::degrees(45.0);
        assert!((a2.tan() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_trig() {
        let a = Angle::asin(1.0);
        assert!((a.to_degrees() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn test_angle_arithmetic() {
        let a1 = Angle::degrees(45.0);
        let a2 = Angle::degrees(45.0);
        let sum = a1 + a2;
        assert!((sum.to_degrees() - 90.0).abs() < 1e-10);
    }
}
