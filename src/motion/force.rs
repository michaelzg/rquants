//! Force quantity and units.

use super::acceleration::{Acceleration, AccelerationUnit};
use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use crate::mass::{Mass, MassUnit};
use std::ops::{Div, Mul};

/// Units of force measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForceUnit {
    /// Newtons (N) - SI unit
    Newtons,
    /// Kilonewtons (kN)
    Kilonewtons,
    /// Kilogram-force (kgf)
    KilogramForce,
    /// Pound-force (lbf)
    PoundForce,
    /// Dynes (dyn) - CGS unit
    Dynes,
}

impl ForceUnit {
    /// All available force units.
    pub const ALL: &'static [ForceUnit] = &[
        ForceUnit::Newtons,
        ForceUnit::Kilonewtons,
        ForceUnit::KilogramForce,
        ForceUnit::PoundForce,
        ForceUnit::Dynes,
    ];
}

// Conversion factors to Newtons
const STANDARD_GRAVITY: f64 = 9.80665;
const POUND_TO_KG: f64 = 0.45359237;

impl_unit_display!(ForceUnit);

impl UnitOfMeasure for ForceUnit {
    fn symbol(&self) -> &'static str {
        match self {
            ForceUnit::Newtons => "N",
            ForceUnit::Kilonewtons => "kN",
            ForceUnit::KilogramForce => "kgf",
            ForceUnit::PoundForce => "lbf",
            ForceUnit::Dynes => "dyn",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            ForceUnit::Newtons => 1.0,
            ForceUnit::Kilonewtons => 1000.0,
            ForceUnit::KilogramForce => STANDARD_GRAVITY,
            ForceUnit::PoundForce => POUND_TO_KG * STANDARD_GRAVITY,
            ForceUnit::Dynes => 1e-5,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            ForceUnit::Newtons | ForceUnit::Kilonewtons | ForceUnit::Dynes
        )
    }
}

/// A quantity of force.
///
/// Force represents a push or pull on an object.
/// F = m * a (Newton's second law)
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let mass = Mass::kilograms(10.0);
/// let acceleration = Acceleration::meters_per_second_squared(9.8);
///
/// // Force = Mass * Acceleration
/// let force = mass * acceleration;
/// assert!((force.to_newtons() - 98.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Force {
    value: f64,
    unit: ForceUnit,
}

impl Force {
    /// Creates a new Force quantity.
    pub const fn new_const(value: f64, unit: ForceUnit) -> Self {
        Self { value, unit }
    }

    /// Creates a Force from mass and acceleration (F = ma).
    pub fn from_mass_and_acceleration(mass: Mass, acceleration: Acceleration) -> Self {
        let newtons = mass.to_kilograms() * acceleration.to_meters_per_second_squared();
        Self::new(newtons, ForceUnit::Newtons)
    }

    // Constructors
    /// Creates a Force in Newtons.
    pub fn newtons(value: f64) -> Self {
        Self::new(value, ForceUnit::Newtons)
    }

    /// Creates a Force in kilonewtons.
    pub fn kilonewtons(value: f64) -> Self {
        Self::new(value, ForceUnit::Kilonewtons)
    }

    /// Creates a Force in kilogram-force.
    pub fn kilogram_force(value: f64) -> Self {
        Self::new(value, ForceUnit::KilogramForce)
    }

    /// Creates a Force in pound-force.
    pub fn pound_force(value: f64) -> Self {
        Self::new(value, ForceUnit::PoundForce)
    }

    /// Creates a Force in dynes.
    pub fn dynes(value: f64) -> Self {
        Self::new(value, ForceUnit::Dynes)
    }

    // Conversion methods
    /// Converts to Newtons.
    pub fn to_newtons(&self) -> f64 {
        self.to(ForceUnit::Newtons)
    }

    /// Converts to kilonewtons.
    pub fn to_kilonewtons(&self) -> f64 {
        self.to(ForceUnit::Kilonewtons)
    }

    /// Converts to kilogram-force.
    pub fn to_kilogram_force(&self) -> f64 {
        self.to(ForceUnit::KilogramForce)
    }

    /// Converts to pound-force.
    pub fn to_pound_force(&self) -> f64 {
        self.to(ForceUnit::PoundForce)
    }

    /// Converts to dynes.
    pub fn to_dynes(&self) -> f64 {
        self.to(ForceUnit::Dynes)
    }
}

impl_quantity!(Force, ForceUnit);

// Force / Mass = Acceleration
impl Div<Mass> for Force {
    type Output = Acceleration;

    fn div(self, rhs: Mass) -> Self::Output {
        let mpss = self.to_newtons() / rhs.to_kilograms();
        Acceleration::new(mpss, AccelerationUnit::MetersPerSecondSquared)
    }
}

// Force / Acceleration = Mass
impl Div<Acceleration> for Force {
    type Output = Mass;

    fn div(self, rhs: Acceleration) -> Self::Output {
        let kg = self.to_newtons() / rhs.to_meters_per_second_squared();
        Mass::new(kg, MassUnit::Kilograms)
    }
}

// Mass * Acceleration = Force
impl Mul<Acceleration> for Mass {
    type Output = Force;

    fn mul(self, rhs: Acceleration) -> Self::Output {
        Force::from_mass_and_acceleration(self, rhs)
    }
}

// Acceleration * Mass = Force
impl Mul<Mass> for Acceleration {
    type Output = Force;

    fn mul(self, rhs: Mass) -> Self::Output {
        Force::from_mass_and_acceleration(rhs, self)
    }
}

impl_dimension!(
    ForceDimension,
    Force,
    ForceUnit,
    "Force",
    ForceUnit::Newtons,
    ForceUnit::Newtons
);

/// Extension trait for creating Force quantities from numeric types.
pub trait ForceConversions {
    /// Creates a Force in Newtons.
    fn newtons(self) -> Force;
    /// Creates a Force in kilonewtons.
    fn kilonewtons(self) -> Force;
    /// Creates a Force in kilogram-force.
    fn kilogram_force(self) -> Force;
    /// Creates a Force in pound-force.
    fn pound_force(self) -> Force;
    /// Creates a Force in dynes.
    fn dynes(self) -> Force;
}

impl ForceConversions for f64 {
    fn newtons(self) -> Force {
        Force::newtons(self)
    }
    fn kilonewtons(self) -> Force {
        Force::kilonewtons(self)
    }
    fn kilogram_force(self) -> Force {
        Force::kilogram_force(self)
    }
    fn pound_force(self) -> Force {
        Force::pound_force(self)
    }
    fn dynes(self) -> Force {
        Force::dynes(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_force_creation() {
        let f = Force::newtons(10.0);
        assert_eq!(f.value(), 10.0);
        assert_eq!(f.unit(), ForceUnit::Newtons);
    }

    #[test]
    fn test_force_conversions() {
        let f = Force::kilonewtons(1.0);
        assert_eq!(f.to_newtons(), 1000.0);

        let f2 = Force::kilogram_force(1.0);
        assert!((f2.to_newtons() - 9.80665).abs() < 0.0001);
    }

    #[test]
    fn test_fma() {
        // F = m * a
        let m = Mass::kilograms(10.0);
        let a = Acceleration::meters_per_second_squared(5.0);
        let f = m * a;
        assert_eq!(f.to_newtons(), 50.0);
    }

    #[test]
    fn test_force_divided_by_mass() {
        // a = F / m
        let f = Force::newtons(100.0);
        let m = Mass::kilograms(10.0);
        let a = f / m;
        assert_eq!(a.to_meters_per_second_squared(), 10.0);
    }

    #[test]
    fn test_force_divided_by_acceleration() {
        // m = F / a
        let f = Force::newtons(100.0);
        let a = Acceleration::meters_per_second_squared(10.0);
        let m = f / a;
        assert_eq!(m.to_kilograms(), 10.0);
    }

    #[test]
    fn test_pound_force() {
        let f = Force::pound_force(1.0);
        // 1 lbf = 0.45359237 kg * 9.80665 m/s² ≈ 4.448 N
        assert!((f.to_newtons() - 4.4482216).abs() < 0.001);
    }
}
