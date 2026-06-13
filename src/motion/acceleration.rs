//! Acceleration quantity and units.

use super::velocity::{Velocity, VelocityUnit};
use crate::core::Quantity;
use crate::time::Time;
use std::ops::{Div, Mul};

// Conversion factors to m/s²
const MM_PER_M: f64 = 0.001;
const FT_PER_M: f64 = 0.3048;
const MILE_PER_M: f64 = 1609.344;
const SECONDS_PER_HOUR: f64 = 3600.0;
const STANDARD_GRAVITY: f64 = 9.80665;
crate::quantity! {
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
    pub quantity Acceleration {
        unit: AccelerationUnit;
        dimension: AccelerationDimension;
        conversions: AccelerationConversions;
        name: "Acceleration";
        primary: MetersPerSecondSquared;
        si: MetersPerSecondSquared;

        units {
            /// Meters per second squared (m/s²) - SI unit
            MetersPerSecondSquared {
                symbol: "m/s²",
                factor: 1.0,
                ctor: meters_per_second_squared,
                to: to_meters_per_second_squared,
                si: true
            },
            /// Millimeters per second squared (mm/s²)
            MillimetersPerSecondSquared {
                symbol: "mm/s²",
                factor: MM_PER_M,
                ctor: millimeters_per_second_squared,
                to: to_millimeters_per_second_squared,
                si: true
            },
            /// Feet per second squared (ft/s²)
            FeetPerSecondSquared {
                symbol: "ft/s²",
                factor: FT_PER_M,
                ctor: feet_per_second_squared,
                to: to_feet_per_second_squared,
                si: false
            },
            /// Miles per hour squared (mph²)
            MilesPerHourSquared {
                symbol: "mph²",
                factor: MILE_PER_M / (SECONDS_PER_HOUR * SECONDS_PER_HOUR),
                ctor: miles_per_hour_squared,
                to: to_miles_per_hour_squared,
                si: false
            },
            /// Standard gravity (g) ≈ 9.80665 m/s²
            EarthGravities {
                symbol: "g",
                factor: STANDARD_GRAVITY,
                ctor: earth_gravities,
                to: to_earth_gravities,
                si: false
            }
        }
    }
}
impl Acceleration {
    /// Creates an Acceleration from velocity and time.
    pub fn from_velocity_and_time(velocity: Velocity, time: Time) -> Self {
        let mpss = velocity.to_meters_per_second() / time.to_seconds();
        Self::new(mpss, AccelerationUnit::MetersPerSecondSquared)
    }
}

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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

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
