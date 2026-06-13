//! Velocity quantity and units.

use crate::core::Quantity;
use crate::space::length::Length;
use crate::time::Time;
use std::ops::{Div, Mul};

// Conversion factors to m/s
const SECONDS_PER_HOUR: f64 = 3600.0;
const MM_PER_M: f64 = 0.001;
const KM_PER_M: f64 = 1000.0;
const FT_PER_M: f64 = 0.3048;
const MILE_PER_M: f64 = 1609.344;
const NAUTICAL_MILE_PER_M: f64 = 1852.0;
crate::quantity! {
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
    pub quantity Velocity {
        unit: VelocityUnit;
        dimension: VelocityDimension;
        conversions: VelocityConversions;
        name: "Velocity";
        primary: MetersPerSecond;
        si: MetersPerSecond;

        units {
            /// Meters per second (m/s) - SI unit
            MetersPerSecond {
                symbol: "m/s",
                factor: 1.0,
                ctor: meters_per_second,
                to: to_meters_per_second,
                si: true
            },
            /// Millimeters per second (mm/s)
            MillimetersPerSecond {
                symbol: "mm/s",
                factor: MM_PER_M,
                ctor: millimeters_per_second,
                to: to_millimeters_per_second,
                si: true
            },
            /// Kilometers per second (km/s)
            KilometersPerSecond {
                symbol: "km/s",
                factor: KM_PER_M,
                ctor: kilometers_per_second,
                to: to_kilometers_per_second,
                si: true
            },
            /// Kilometers per hour (km/h)
            KilometersPerHour {
                symbol: "km/h",
                factor: KM_PER_M / SECONDS_PER_HOUR,
                ctor: kilometers_per_hour,
                to: to_kilometers_per_hour,
                si: true
            },
            /// Feet per second (ft/s)
            FeetPerSecond {
                symbol: "ft/s",
                factor: FT_PER_M,
                ctor: feet_per_second,
                to: to_feet_per_second,
                si: false
            },
            /// Miles per hour (mph)
            MilesPerHour {
                symbol: "mph",
                factor: MILE_PER_M / SECONDS_PER_HOUR,
                ctor: miles_per_hour,
                to: to_miles_per_hour,
                si: false
            },
            /// Knots (kn) - nautical miles per hour
            Knots {
                symbol: "kn",
                factor: NAUTICAL_MILE_PER_M / SECONDS_PER_HOUR,
                ctor: knots,
                to: to_knots,
                si: false
            }
        }
    }
}
impl Velocity {
    /// Creates a Velocity from length and time.
    pub fn from_length_and_time(length: Length, time: Time) -> Self {
        let mps = length.to_meters() / time.to_seconds();
        Self::new(mps, VelocityUnit::MetersPerSecond)
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

// Length / Time = Velocity
impl Div<Time> for Length {
    type Output = Velocity;

    fn div(self, rhs: Time) -> Self::Output {
        Velocity::from_length_and_time(self, rhs)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

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
