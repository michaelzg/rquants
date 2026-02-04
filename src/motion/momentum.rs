//! Momentum quantity and units.

use super::velocity::{Velocity, VelocityUnit};
use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use crate::mass::{Mass, MassUnit};
use std::ops::{Div, Mul};

/// Units of momentum measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MomentumUnit {
    /// Kilogram-meters per second (kg·m/s) - SI unit
    KilogramMetersPerSecond,
    /// Newton-seconds (N·s) - equivalent to kg·m/s
    NewtonSeconds,
    /// Pound-feet per second (lb·ft/s)
    PoundFeetPerSecond,
}

impl MomentumUnit {
    /// All available momentum units.
    pub const ALL: &'static [MomentumUnit] = &[
        MomentumUnit::KilogramMetersPerSecond,
        MomentumUnit::NewtonSeconds,
        MomentumUnit::PoundFeetPerSecond,
    ];
}

// Conversion factors to kg·m/s
const LB_TO_KG: f64 = 0.45359237;
const FT_TO_M: f64 = 0.3048;

impl_unit_display!(MomentumUnit);

impl UnitOfMeasure for MomentumUnit {
    fn symbol(&self) -> &'static str {
        match self {
            MomentumUnit::KilogramMetersPerSecond => "kg·m/s",
            MomentumUnit::NewtonSeconds => "N·s",
            MomentumUnit::PoundFeetPerSecond => "lb·ft/s",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            MomentumUnit::KilogramMetersPerSecond => 1.0,
            MomentumUnit::NewtonSeconds => 1.0, // N·s = kg·m/s
            MomentumUnit::PoundFeetPerSecond => LB_TO_KG * FT_TO_M,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            MomentumUnit::KilogramMetersPerSecond | MomentumUnit::NewtonSeconds
        )
    }
}

/// A quantity of momentum (mass in motion).
///
/// Momentum is the product of mass and velocity.
/// p = m * v
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let mass = Mass::kilograms(10.0);
/// let velocity = Velocity::meters_per_second(5.0);
///
/// // Momentum = Mass * Velocity
/// let momentum = mass * velocity;
/// assert!((momentum.to_kilogram_meters_per_second() - 50.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Momentum {
    value: f64,
    unit: MomentumUnit,
}

impl Momentum {
    /// Creates a new Momentum quantity.
    pub const fn new_const(value: f64, unit: MomentumUnit) -> Self {
        Self { value, unit }
    }

    /// Creates a Momentum from mass and velocity (p = mv).
    pub fn from_mass_and_velocity(mass: Mass, velocity: Velocity) -> Self {
        let kgmps = mass.to_kilograms() * velocity.to_meters_per_second();
        Self::new(kgmps, MomentumUnit::KilogramMetersPerSecond)
    }

    // Constructors
    /// Creates a Momentum in kg·m/s.
    pub fn kilogram_meters_per_second(value: f64) -> Self {
        Self::new(value, MomentumUnit::KilogramMetersPerSecond)
    }

    /// Creates a Momentum in N·s.
    pub fn newton_seconds(value: f64) -> Self {
        Self::new(value, MomentumUnit::NewtonSeconds)
    }

    /// Creates a Momentum in lb·ft/s.
    pub fn pound_feet_per_second(value: f64) -> Self {
        Self::new(value, MomentumUnit::PoundFeetPerSecond)
    }

    // Conversion methods
    /// Converts to kg·m/s.
    pub fn to_kilogram_meters_per_second(&self) -> f64 {
        self.to(MomentumUnit::KilogramMetersPerSecond)
    }

    /// Converts to N·s.
    pub fn to_newton_seconds(&self) -> f64 {
        self.to(MomentumUnit::NewtonSeconds)
    }

    /// Converts to lb·ft/s.
    pub fn to_pound_feet_per_second(&self) -> f64 {
        self.to(MomentumUnit::PoundFeetPerSecond)
    }
}

impl_quantity!(Momentum, MomentumUnit);

// Momentum / Mass = Velocity
impl Div<Mass> for Momentum {
    type Output = Velocity;

    fn div(self, rhs: Mass) -> Self::Output {
        let mps = self.to_kilogram_meters_per_second() / rhs.to_kilograms();
        Velocity::new(mps, VelocityUnit::MetersPerSecond)
    }
}

// Momentum / Velocity = Mass
impl Div<Velocity> for Momentum {
    type Output = Mass;

    fn div(self, rhs: Velocity) -> Self::Output {
        let kg = self.to_kilogram_meters_per_second() / rhs.to_meters_per_second();
        Mass::new(kg, MassUnit::Kilograms)
    }
}

// Mass * Velocity = Momentum
impl Mul<Velocity> for Mass {
    type Output = Momentum;

    fn mul(self, rhs: Velocity) -> Self::Output {
        Momentum::from_mass_and_velocity(self, rhs)
    }
}

// Velocity * Mass = Momentum
impl Mul<Mass> for Velocity {
    type Output = Momentum;

    fn mul(self, rhs: Mass) -> Self::Output {
        Momentum::from_mass_and_velocity(rhs, self)
    }
}

impl_dimension!(
    MomentumDimension,
    Momentum,
    MomentumUnit,
    "Momentum",
    MomentumUnit::KilogramMetersPerSecond,
    MomentumUnit::KilogramMetersPerSecond
);

/// Extension trait for creating Momentum quantities from numeric types.
pub trait MomentumConversions {
    /// Creates a Momentum in kg·m/s.
    fn kilogram_meters_per_second(self) -> Momentum;
    /// Creates a Momentum in N·s.
    fn newton_seconds(self) -> Momentum;
}

impl MomentumConversions for f64 {
    fn kilogram_meters_per_second(self) -> Momentum {
        Momentum::kilogram_meters_per_second(self)
    }
    fn newton_seconds(self) -> Momentum {
        Momentum::newton_seconds(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_momentum_creation() {
        let p = Momentum::kilogram_meters_per_second(10.0);
        assert_eq!(p.value(), 10.0);
        assert_eq!(p.unit(), MomentumUnit::KilogramMetersPerSecond);
    }

    #[test]
    fn test_newton_seconds_equivalent() {
        let p1 = Momentum::kilogram_meters_per_second(10.0);
        let p2 = Momentum::newton_seconds(10.0);
        assert_eq!(
            p1.to_kilogram_meters_per_second(),
            p2.to_kilogram_meters_per_second()
        );
    }

    #[test]
    fn test_mass_times_velocity() {
        let m = Mass::kilograms(5.0);
        let v = Velocity::meters_per_second(10.0);
        let p = m * v;
        assert_eq!(p.to_kilogram_meters_per_second(), 50.0);
    }

    #[test]
    fn test_momentum_divided_by_mass() {
        let p = Momentum::kilogram_meters_per_second(100.0);
        let m = Mass::kilograms(10.0);
        let v = p / m;
        assert_eq!(v.to_meters_per_second(), 10.0);
    }

    #[test]
    fn test_momentum_divided_by_velocity() {
        let p = Momentum::kilogram_meters_per_second(100.0);
        let v = Velocity::meters_per_second(10.0);
        let m = p / v;
        assert_eq!(m.to_kilograms(), 10.0);
    }
}
