//! Acceleration quantity and units.

use super::velocity::{Velocity, VelocityUnit};
use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use crate::time::Time;
use std::ops::{Div, Mul};

/// Units of acceleration measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccelerationUnit {
    /// Meters per second squared (m/s²) - SI unit
    MetersPerSecondSquared,
    /// Millimeters per second squared (mm/s²)
    MillimetersPerSecondSquared,
    /// Feet per second squared (ft/s²)
    FeetPerSecondSquared,
    /// Miles per hour squared (mph²)
    MilesPerHourSquared,
    /// Standard gravity (g) ≈ 9.80665 m/s²
    EarthGravities,
}

impl AccelerationUnit {
    /// All available acceleration units.
    pub const ALL: &'static [AccelerationUnit] = &[
        AccelerationUnit::MetersPerSecondSquared,
        AccelerationUnit::MillimetersPerSecondSquared,
        AccelerationUnit::FeetPerSecondSquared,
        AccelerationUnit::MilesPerHourSquared,
        AccelerationUnit::EarthGravities,
    ];
}

// Conversion factors to m/s²
const MM_PER_M: f64 = 0.001;
const FT_PER_M: f64 = 0.3048;
const MILE_PER_M: f64 = 1609.344;
const SECONDS_PER_HOUR: f64 = 3600.0;
const STANDARD_GRAVITY: f64 = 9.80665;

impl_unit_display!(AccelerationUnit);

impl UnitOfMeasure for AccelerationUnit {
    fn symbol(&self) -> &'static str {
        match self {
            AccelerationUnit::MetersPerSecondSquared => "m/s²",
            AccelerationUnit::MillimetersPerSecondSquared => "mm/s²",
            AccelerationUnit::FeetPerSecondSquared => "ft/s²",
            AccelerationUnit::MilesPerHourSquared => "mph²",
            AccelerationUnit::EarthGravities => "g",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            AccelerationUnit::MetersPerSecondSquared => 1.0,
            AccelerationUnit::MillimetersPerSecondSquared => MM_PER_M,
            AccelerationUnit::FeetPerSecondSquared => FT_PER_M,
            AccelerationUnit::MilesPerHourSquared => {
                MILE_PER_M / (SECONDS_PER_HOUR * SECONDS_PER_HOUR)
            }
            AccelerationUnit::EarthGravities => STANDARD_GRAVITY,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            AccelerationUnit::MetersPerSecondSquared
                | AccelerationUnit::MillimetersPerSecondSquared
        )
    }
}

/// A quantity of acceleration (rate of change of velocity).
///
/// Acceleration represents change in velocity per unit time.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let a = Acceleration::meters_per_second_squared(9.8);
/// let time = Time::seconds(2.0);
///
/// // Velocity = Acceleration * Time
/// let velocity = a * time;
/// assert!((velocity.to_meters_per_second() - 19.6).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Acceleration {
    value: f64,
    unit: AccelerationUnit,
}

impl Acceleration {
    /// Creates a new Acceleration quantity.
    pub const fn new_const(value: f64, unit: AccelerationUnit) -> Self {
        Self { value, unit }
    }

    /// Creates an Acceleration from velocity and time.
    pub fn from_velocity_and_time(velocity: Velocity, time: Time) -> Self {
        let mpss = velocity.to_meters_per_second() / time.to_seconds();
        Self::new(mpss, AccelerationUnit::MetersPerSecondSquared)
    }

    // Constructors
    /// Creates an Acceleration in m/s².
    pub fn meters_per_second_squared(value: f64) -> Self {
        Self::new(value, AccelerationUnit::MetersPerSecondSquared)
    }

    /// Creates an Acceleration in mm/s².
    pub fn millimeters_per_second_squared(value: f64) -> Self {
        Self::new(value, AccelerationUnit::MillimetersPerSecondSquared)
    }

    /// Creates an Acceleration in ft/s².
    pub fn feet_per_second_squared(value: f64) -> Self {
        Self::new(value, AccelerationUnit::FeetPerSecondSquared)
    }

    /// Creates an Acceleration in miles per hour squared.
    pub fn miles_per_hour_squared(value: f64) -> Self {
        Self::new(value, AccelerationUnit::MilesPerHourSquared)
    }

    /// Creates an Acceleration in g (standard gravity).
    pub fn earth_gravities(value: f64) -> Self {
        Self::new(value, AccelerationUnit::EarthGravities)
    }

    // Conversion methods
    /// Converts to m/s².
    pub fn to_meters_per_second_squared(&self) -> f64 {
        self.to(AccelerationUnit::MetersPerSecondSquared)
    }

    /// Converts to ft/s².
    pub fn to_feet_per_second_squared(&self) -> f64 {
        self.to(AccelerationUnit::FeetPerSecondSquared)
    }

    /// Converts to g (standard gravity).
    pub fn to_earth_gravities(&self) -> f64 {
        self.to(AccelerationUnit::EarthGravities)
    }

    /// Converts to mm/s².
    pub fn to_millimeters_per_second_squared(&self) -> f64 {
        self.to(AccelerationUnit::MillimetersPerSecondSquared)
    }

    /// Converts to mph².
    pub fn to_miles_per_hour_squared(&self) -> f64 {
        self.to(AccelerationUnit::MilesPerHourSquared)
    }
}

impl_quantity!(Acceleration, AccelerationUnit);

// Acceleration * Time = Velocity
impl Mul<Time> for Acceleration {
    type Output = Velocity;

    fn mul(self, rhs: Time) -> Self::Output {
        let mps = self.to_meters_per_second_squared() * rhs.to_seconds();
        Velocity::new(mps, VelocityUnit::MetersPerSecond)
    }
}

// Velocity / Time = Acceleration
impl Div<Time> for Velocity {
    type Output = Acceleration;

    fn div(self, rhs: Time) -> Self::Output {
        Acceleration::from_velocity_and_time(self, rhs)
    }
}

// Velocity / Acceleration = Time
impl Div<Acceleration> for Velocity {
    type Output = Time;

    fn div(self, rhs: Acceleration) -> Self::Output {
        let seconds = self.to_meters_per_second() / rhs.to_meters_per_second_squared();
        Time::seconds(seconds)
    }
}

impl_dimension!(
    AccelerationDimension,
    Acceleration,
    AccelerationUnit,
    "Acceleration",
    AccelerationUnit::MetersPerSecondSquared,
    AccelerationUnit::MetersPerSecondSquared
);

/// Extension trait for creating Acceleration quantities from numeric types.
pub trait AccelerationConversions {
    /// Creates an Acceleration in m/s².
    fn meters_per_second_squared(self) -> Acceleration;
    /// Creates an Acceleration in mm/s².
    fn millimeters_per_second_squared(self) -> Acceleration;
    /// Creates an Acceleration in ft/s².
    fn feet_per_second_squared(self) -> Acceleration;
    /// Creates an Acceleration in mph².
    fn miles_per_hour_squared(self) -> Acceleration;
    /// Creates an Acceleration in g.
    fn earth_gravities(self) -> Acceleration;
}

impl AccelerationConversions for f64 {
    fn meters_per_second_squared(self) -> Acceleration {
        Acceleration::meters_per_second_squared(self)
    }
    fn millimeters_per_second_squared(self) -> Acceleration {
        Acceleration::millimeters_per_second_squared(self)
    }
    fn feet_per_second_squared(self) -> Acceleration {
        Acceleration::feet_per_second_squared(self)
    }
    fn miles_per_hour_squared(self) -> Acceleration {
        Acceleration::miles_per_hour_squared(self)
    }
    fn earth_gravities(self) -> Acceleration {
        Acceleration::earth_gravities(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceleration_creation() {
        let a = Acceleration::meters_per_second_squared(10.0);
        assert_eq!(a.value(), 10.0);
        assert_eq!(a.unit(), AccelerationUnit::MetersPerSecondSquared);
    }

    #[test]
    fn test_earth_gravity() {
        let a = Acceleration::earth_gravities(1.0);
        assert!((a.to_meters_per_second_squared() - 9.80665).abs() < 0.0001);
    }

    #[test]
    fn test_acceleration_times_time() {
        let a = Acceleration::meters_per_second_squared(10.0);
        let t = Time::seconds(5.0);
        let v = a * t;
        assert_eq!(v.to_meters_per_second(), 50.0);
    }

    #[test]
    fn test_velocity_divided_by_time() {
        let v = Velocity::meters_per_second(100.0);
        let t = Time::seconds(10.0);
        let a = v / t;
        assert_eq!(a.to_meters_per_second_squared(), 10.0);
    }

    #[test]
    fn test_velocity_divided_by_acceleration() {
        let v = Velocity::meters_per_second(100.0);
        let a = Acceleration::meters_per_second_squared(10.0);
        let t = v / a;
        assert_eq!(t.to_seconds(), 10.0);
    }
}
