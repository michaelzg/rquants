//! Power ramp quantity and units.

use crate::core::Quantity;
use crate::time::Time;
use std::ops::Mul;
crate::quantity! {
    /// A quantity of power ramp (rate of power change).
    ///
    /// Power ramp represents how fast power changes over time.
    /// dP/dt = PowerRamp
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let ramp = PowerRamp::kilowatts_per_hour(100.0);
    /// let time = Time::hours(2.0);
    ///
    /// // Power change = PowerRamp * Time
    /// let power_change = ramp * time;
    /// assert!((power_change.to_kilowatts() - 200.0).abs() < 1e-10);
    /// ```
    pub quantity PowerRamp {
        unit: PowerRampUnit;
        dimension: PowerRampDimension;
        conversions: PowerRampConversions;
        name: "PowerRamp";
        primary: WattsPerHour;
        si: WattsPerHour;

        units {
            /// Watts per hour (W/h) - primary unit
            WattsPerHour {
                symbol: "W/h",
                factor: 1.0,
                ctor: watts_per_hour,
                to: to_watts_per_hour,
                si: false
            },
            /// Watts per minute (W/min)
            WattsPerMinute {
                symbol: "W/min",
                factor: 60.0,
                ctor: watts_per_minute,
                to: to_watts_per_minute,
                si: false
            },
            /// Kilowatts per hour (kW/h)
            KilowattsPerHour {
                symbol: "kW/h",
                factor: 1e3,
                ctor: kilowatts_per_hour,
                to: to_kilowatts_per_hour,
                si: false
            },
            /// Kilowatts per minute (kW/min)
            KilowattsPerMinute {
                symbol: "kW/min",
                factor: 1e3 * 60.0,
                ctor: kilowatts_per_minute,
                to: to_kilowatts_per_minute,
                si: false
            },
            /// Megawatts per hour (MW/h)
            MegawattsPerHour {
                symbol: "MW/h",
                factor: 1e6,
                ctor: megawatts_per_hour,
                to: to_megawatts_per_hour,
                si: false
            },
            /// Gigawatts per hour (GW/h)
            GigawattsPerHour {
                symbol: "GW/h",
                factor: 1e9,
                ctor: gigawatts_per_hour,
                to: to_gigawatts_per_hour,
                si: false
            }
        }
    }
}
// Cross-quantity operations
use super::power::{Power, PowerUnit};

// PowerRamp * Time = Power
impl Mul<Time> for PowerRamp {
    type Output = Power;

    fn mul(self, rhs: Time) -> Self::Output {
        let watts = self.to_watts_per_hour() * rhs.to_hours();
        Power::new(watts, PowerUnit::Watts)
    }
}

// Time * PowerRamp = Power
impl Mul<PowerRamp> for Time {
    type Output = Power;

    fn mul(self, rhs: PowerRamp) -> Self::Output {
        let watts = rhs.to_watts_per_hour() * self.to_hours();
        Power::new(watts, PowerUnit::Watts)
    }
}

// PowerRamp / Power = 1/Time (returns frequency-like, but we return Time for simplicity)
// Note: This is Power / PowerRamp = Time, implemented in power.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_power_ramp_creation() {
        let pr = PowerRamp::watts_per_hour(100.0);
        assert_eq!(pr.value(), 100.0);
        assert_eq!(pr.unit(), PowerRampUnit::WattsPerHour);
    }

    #[test]
    fn test_power_ramp_conversions() {
        let pr = PowerRamp::kilowatts_per_hour(1.0);
        assert_eq!(pr.to_watts_per_hour(), 1000.0);
    }

    #[test]
    fn test_watts_per_minute() {
        let pr = PowerRamp::watts_per_minute(1.0);
        // 1 W/min = 60 W/h
        assert!((pr.to_watts_per_hour() - 60.0).abs() < 1e-10);
    }

    #[test]
    fn test_power_ramp_times_time() {
        let pr = PowerRamp::kilowatts_per_hour(50.0);
        let t = Time::hours(2.0);
        let p = pr * t;
        // 50 kW/h * 2 h = 100 kW
        assert!((p.to_kilowatts() - 100.0).abs() < 1e-10);
    }
}
