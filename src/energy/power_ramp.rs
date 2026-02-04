//! Power ramp quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use crate::time::Time;
use std::ops::{Mul};

/// Units of power ramp (rate of power change) measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PowerRampUnit {
    /// Watts per hour (W/h) - primary unit
    WattsPerHour,
    /// Watts per minute (W/min)
    WattsPerMinute,
    /// Kilowatts per hour (kW/h)
    KilowattsPerHour,
    /// Kilowatts per minute (kW/min)
    KilowattsPerMinute,
    /// Megawatts per hour (MW/h)
    MegawattsPerHour,
    /// Gigawatts per hour (GW/h)
    GigawattsPerHour,
}

impl PowerRampUnit {
    /// All available power ramp units.
    pub const ALL: &'static [PowerRampUnit] = &[
        PowerRampUnit::WattsPerHour,
        PowerRampUnit::WattsPerMinute,
        PowerRampUnit::KilowattsPerHour,
        PowerRampUnit::KilowattsPerMinute,
        PowerRampUnit::MegawattsPerHour,
        PowerRampUnit::GigawattsPerHour,
    ];
}

impl_unit_display!(PowerRampUnit);

impl UnitOfMeasure for PowerRampUnit {
    fn symbol(&self) -> &'static str {
        match self {
            PowerRampUnit::WattsPerHour => "W/h",
            PowerRampUnit::WattsPerMinute => "W/min",
            PowerRampUnit::KilowattsPerHour => "kW/h",
            PowerRampUnit::KilowattsPerMinute => "kW/min",
            PowerRampUnit::MegawattsPerHour => "MW/h",
            PowerRampUnit::GigawattsPerHour => "GW/h",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            PowerRampUnit::WattsPerHour => 1.0,
            PowerRampUnit::WattsPerMinute => 60.0,         // 1 W/min = 60 W/h
            PowerRampUnit::KilowattsPerHour => 1e3,
            PowerRampUnit::KilowattsPerMinute => 1e3 * 60.0,
            PowerRampUnit::MegawattsPerHour => 1e6,
            PowerRampUnit::GigawattsPerHour => 1e9,
        }
    }

    fn is_si(&self) -> bool {
        true
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct PowerRamp {
    value: f64,
    unit: PowerRampUnit,
}

impl PowerRamp {
    /// Creates a new PowerRamp quantity.
    pub const fn new_const(value: f64, unit: PowerRampUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a PowerRamp in watts per hour.
    pub fn watts_per_hour(value: f64) -> Self {
        Self::new(value, PowerRampUnit::WattsPerHour)
    }

    /// Creates a PowerRamp in watts per minute.
    pub fn watts_per_minute(value: f64) -> Self {
        Self::new(value, PowerRampUnit::WattsPerMinute)
    }

    /// Creates a PowerRamp in kilowatts per hour.
    pub fn kilowatts_per_hour(value: f64) -> Self {
        Self::new(value, PowerRampUnit::KilowattsPerHour)
    }

    /// Creates a PowerRamp in kilowatts per minute.
    pub fn kilowatts_per_minute(value: f64) -> Self {
        Self::new(value, PowerRampUnit::KilowattsPerMinute)
    }

    /// Creates a PowerRamp in megawatts per hour.
    pub fn megawatts_per_hour(value: f64) -> Self {
        Self::new(value, PowerRampUnit::MegawattsPerHour)
    }

    /// Creates a PowerRamp in gigawatts per hour.
    pub fn gigawatts_per_hour(value: f64) -> Self {
        Self::new(value, PowerRampUnit::GigawattsPerHour)
    }

    // Conversion methods
    /// Converts to watts per hour.
    pub fn to_watts_per_hour(&self) -> f64 {
        self.to(PowerRampUnit::WattsPerHour)
    }

    /// Converts to watts per minute.
    pub fn to_watts_per_minute(&self) -> f64 {
        self.to(PowerRampUnit::WattsPerMinute)
    }

    /// Converts to kilowatts per hour.
    pub fn to_kilowatts_per_hour(&self) -> f64 {
        self.to(PowerRampUnit::KilowattsPerHour)
    }

    /// Converts to kilowatts per minute.
    pub fn to_kilowatts_per_minute(&self) -> f64 {
        self.to(PowerRampUnit::KilowattsPerMinute)
    }

    /// Converts to megawatts per hour.
    pub fn to_megawatts_per_hour(&self) -> f64 {
        self.to(PowerRampUnit::MegawattsPerHour)
    }

    /// Converts to gigawatts per hour.
    pub fn to_gigawatts_per_hour(&self) -> f64 {
        self.to(PowerRampUnit::GigawattsPerHour)
    }
}

impl_quantity!(PowerRamp, PowerRampUnit);

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

impl_dimension!(
    PowerRampDimension,
    PowerRamp,
    PowerRampUnit,
    "PowerRamp",
    PowerRampUnit::WattsPerHour,
    PowerRampUnit::WattsPerHour
);

/// Extension trait for creating PowerRamp quantities from numeric types.
pub trait PowerRampConversions {
    /// Creates a PowerRamp in watts per hour.
    fn watts_per_hour(self) -> PowerRamp;
    /// Creates a PowerRamp in watts per minute.
    fn watts_per_minute(self) -> PowerRamp;
    /// Creates a PowerRamp in kilowatts per hour.
    fn kilowatts_per_hour(self) -> PowerRamp;
    /// Creates a PowerRamp in kilowatts per minute.
    fn kilowatts_per_minute(self) -> PowerRamp;
    /// Creates a PowerRamp in megawatts per hour.
    fn megawatts_per_hour(self) -> PowerRamp;
    /// Creates a PowerRamp in gigawatts per hour.
    fn gigawatts_per_hour(self) -> PowerRamp;
}

impl PowerRampConversions for f64 {
    fn watts_per_hour(self) -> PowerRamp {
        PowerRamp::watts_per_hour(self)
    }
    fn watts_per_minute(self) -> PowerRamp {
        PowerRamp::watts_per_minute(self)
    }
    fn kilowatts_per_hour(self) -> PowerRamp {
        PowerRamp::kilowatts_per_hour(self)
    }
    fn kilowatts_per_minute(self) -> PowerRamp {
        PowerRamp::kilowatts_per_minute(self)
    }
    fn megawatts_per_hour(self) -> PowerRamp {
        PowerRamp::megawatts_per_hour(self)
    }
    fn gigawatts_per_hour(self) -> PowerRamp {
        PowerRamp::gigawatts_per_hour(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
