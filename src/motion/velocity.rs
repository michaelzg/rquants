//! Velocity quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use crate::space::length::Length;
use crate::time::Time;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of velocity measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VelocityUnit {
    /// Meters per second (m/s) - SI unit
    MetersPerSecond,
    /// Millimeters per second (mm/s)
    MillimetersPerSecond,
    /// Kilometers per second (km/s)
    KilometersPerSecond,
    /// Kilometers per hour (km/h)
    KilometersPerHour,
    /// Feet per second (ft/s)
    FeetPerSecond,
    /// Miles per hour (mph)
    MilesPerHour,
    /// Knots (kn) - nautical miles per hour
    Knots,
}

impl VelocityUnit {
    /// All available velocity units.
    pub const ALL: &'static [VelocityUnit] = &[
        VelocityUnit::MetersPerSecond,
        VelocityUnit::MillimetersPerSecond,
        VelocityUnit::KilometersPerSecond,
        VelocityUnit::KilometersPerHour,
        VelocityUnit::FeetPerSecond,
        VelocityUnit::MilesPerHour,
        VelocityUnit::Knots,
    ];
}

// Conversion factors to m/s
const SECONDS_PER_HOUR: f64 = 3600.0;
const MM_PER_M: f64 = 0.001;
const KM_PER_M: f64 = 1000.0;
const FT_PER_M: f64 = 0.3048;
const MILE_PER_M: f64 = 1609.344;
const NAUTICAL_MILE_PER_M: f64 = 1852.0;

impl fmt::Display for VelocityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for VelocityUnit {
    fn symbol(&self) -> &'static str {
        match self {
            VelocityUnit::MetersPerSecond => "m/s",
            VelocityUnit::MillimetersPerSecond => "mm/s",
            VelocityUnit::KilometersPerSecond => "km/s",
            VelocityUnit::KilometersPerHour => "km/h",
            VelocityUnit::FeetPerSecond => "ft/s",
            VelocityUnit::MilesPerHour => "mph",
            VelocityUnit::Knots => "kn",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            VelocityUnit::MetersPerSecond => 1.0,
            VelocityUnit::MillimetersPerSecond => MM_PER_M,
            VelocityUnit::KilometersPerSecond => KM_PER_M,
            VelocityUnit::KilometersPerHour => KM_PER_M / SECONDS_PER_HOUR,
            VelocityUnit::FeetPerSecond => FT_PER_M,
            VelocityUnit::MilesPerHour => MILE_PER_M / SECONDS_PER_HOUR,
            VelocityUnit::Knots => NAUTICAL_MILE_PER_M / SECONDS_PER_HOUR,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            VelocityUnit::MetersPerSecond
                | VelocityUnit::MillimetersPerSecond
                | VelocityUnit::KilometersPerSecond
                | VelocityUnit::KilometersPerHour
        )
    }
}

/// A quantity of velocity (rate of change of position).
///
/// Velocity represents distance traveled per unit time.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let v = Velocity::meters_per_second(10.0);
/// let time = Time::seconds(5.0);
///
/// // Distance = Velocity * Time
/// let distance = v * time;
/// assert!((distance.to_meters() - 50.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    value: f64,
    unit: VelocityUnit,
}

impl Velocity {
    /// Creates a new Velocity quantity.
    pub const fn new_const(value: f64, unit: VelocityUnit) -> Self {
        Self { value, unit }
    }

    /// Creates a Velocity from length and time.
    pub fn from_length_and_time(length: Length, time: Time) -> Self {
        let mps = length.to_meters() / time.to_seconds();
        Self::new(mps, VelocityUnit::MetersPerSecond)
    }

    // Constructors
    /// Creates a Velocity in m/s.
    pub fn meters_per_second(value: f64) -> Self {
        Self::new(value, VelocityUnit::MetersPerSecond)
    }

    /// Creates a Velocity in mm/s.
    pub fn millimeters_per_second(value: f64) -> Self {
        Self::new(value, VelocityUnit::MillimetersPerSecond)
    }

    /// Creates a Velocity in km/s.
    pub fn kilometers_per_second(value: f64) -> Self {
        Self::new(value, VelocityUnit::KilometersPerSecond)
    }

    /// Creates a Velocity in km/h.
    pub fn kilometers_per_hour(value: f64) -> Self {
        Self::new(value, VelocityUnit::KilometersPerHour)
    }

    /// Creates a Velocity in ft/s.
    pub fn feet_per_second(value: f64) -> Self {
        Self::new(value, VelocityUnit::FeetPerSecond)
    }

    /// Creates a Velocity in mph.
    pub fn miles_per_hour(value: f64) -> Self {
        Self::new(value, VelocityUnit::MilesPerHour)
    }

    /// Creates a Velocity in knots.
    pub fn knots(value: f64) -> Self {
        Self::new(value, VelocityUnit::Knots)
    }

    // Conversion methods
    /// Converts to m/s.
    pub fn to_meters_per_second(&self) -> f64 {
        self.to(VelocityUnit::MetersPerSecond)
    }

    /// Converts to km/h.
    pub fn to_kilometers_per_hour(&self) -> f64 {
        self.to(VelocityUnit::KilometersPerHour)
    }

    /// Converts to mph.
    pub fn to_miles_per_hour(&self) -> f64 {
        self.to(VelocityUnit::MilesPerHour)
    }

    /// Converts to ft/s.
    pub fn to_feet_per_second(&self) -> f64 {
        self.to(VelocityUnit::FeetPerSecond)
    }

    /// Converts to knots.
    pub fn to_knots(&self) -> f64 {
        self.to(VelocityUnit::Knots)
    }
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Velocity {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Velocity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Velocity {
    type Unit = VelocityUnit;

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

impl Add for Velocity {
    type Output = Velocity;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Velocity::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Velocity {
    type Output = Velocity;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Velocity::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: f64) -> Self::Output {
        Velocity::new(self.value * rhs, self.unit)
    }
}

impl Mul<Velocity> for f64 {
    type Output = Velocity;

    fn mul(self, rhs: Velocity) -> Self::Output {
        Velocity::new(self * rhs.value, rhs.unit)
    }
}

// Velocity * Time = Length
impl Mul<Time> for Velocity {
    type Output = Length;

    fn mul(self, rhs: Time) -> Self::Output {
        let meters = self.to_meters_per_second() * rhs.to_seconds();
        Length::meters(meters)
    }
}

impl Div<f64> for Velocity {
    type Output = Velocity;

    fn div(self, rhs: f64) -> Self::Output {
        Velocity::new(self.value / rhs, self.unit)
    }
}

impl Div<Velocity> for Velocity {
    type Output = f64;

    fn div(self, rhs: Velocity) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

// Velocity / Time = Acceleration (defined in acceleration.rs)

impl Neg for Velocity {
    type Output = Velocity;

    fn neg(self) -> Self::Output {
        Velocity::new(-self.value, self.unit)
    }
}

// Length / Time = Velocity
impl Div<Time> for Length {
    type Output = Velocity;

    fn div(self, rhs: Time) -> Self::Output {
        Velocity::from_length_and_time(self, rhs)
    }
}

/// Dimension for Velocity.
pub struct VelocityDimension;

impl Dimension for VelocityDimension {
    type Quantity = Velocity;
    type Unit = VelocityUnit;

    fn name() -> &'static str {
        "Velocity"
    }

    fn primary_unit() -> Self::Unit {
        VelocityUnit::MetersPerSecond
    }

    fn si_unit() -> Self::Unit {
        VelocityUnit::MetersPerSecond
    }

    fn units() -> &'static [Self::Unit] {
        VelocityUnit::ALL
    }
}

/// Extension trait for creating Velocity quantities from numeric types.
pub trait VelocityConversions {
    /// Creates a Velocity in m/s.
    fn meters_per_second(self) -> Velocity;
    /// Creates a Velocity in km/h.
    fn kilometers_per_hour(self) -> Velocity;
    /// Creates a Velocity in mph.
    fn miles_per_hour(self) -> Velocity;
    /// Creates a Velocity in knots.
    fn knots(self) -> Velocity;
}

impl VelocityConversions for f64 {
    fn meters_per_second(self) -> Velocity {
        Velocity::meters_per_second(self)
    }
    fn kilometers_per_hour(self) -> Velocity {
        Velocity::kilometers_per_hour(self)
    }
    fn miles_per_hour(self) -> Velocity {
        Velocity::miles_per_hour(self)
    }
    fn knots(self) -> Velocity {
        Velocity::knots(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_creation() {
        let v = Velocity::meters_per_second(10.0);
        assert_eq!(v.value(), 10.0);
        assert_eq!(v.unit(), VelocityUnit::MetersPerSecond);
    }

    #[test]
    fn test_velocity_conversions() {
        let v = Velocity::meters_per_second(1.0);
        assert!((v.to_kilometers_per_hour() - 3.6).abs() < 0.001);

        let v2 = Velocity::kilometers_per_hour(3.6);
        assert!((v2.to_meters_per_second() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_velocity_times_time() {
        let v = Velocity::meters_per_second(10.0);
        let t = Time::seconds(5.0);
        let d = v * t;
        assert_eq!(d.to_meters(), 50.0);
    }

    #[test]
    fn test_length_divided_by_time() {
        let d = Length::meters(100.0);
        let t = Time::seconds(10.0);
        let v = d / t;
        assert_eq!(v.to_meters_per_second(), 10.0);
    }

    #[test]
    fn test_mph_conversion() {
        let v = Velocity::miles_per_hour(60.0);
        // 60 mph ≈ 26.82 m/s
        assert!((v.to_meters_per_second() - 26.8224).abs() < 0.001);
    }

    #[test]
    fn test_knots_conversion() {
        let v = Velocity::knots(1.0);
        // 1 knot = 1852/3600 m/s ≈ 0.5144 m/s
        assert!((v.to_meters_per_second() - 0.5144).abs() < 0.001);
    }
}
