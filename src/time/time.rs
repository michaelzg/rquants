//! Time quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use crate::systems::metric::{MICRO, MILLI, NANO};

/// Time conversion constants.
pub mod constants {
    /// Nanoseconds per second.
    pub const NANOSECONDS_PER_SECOND: f64 = 1.0e9;
    /// Microseconds per second.
    pub const MICROSECONDS_PER_SECOND: f64 = 1.0e6;
    /// Milliseconds per second.
    pub const MILLISECONDS_PER_SECOND: f64 = 1.0e3;
    /// Seconds per minute.
    pub const SECONDS_PER_MINUTE: f64 = 60.0;
    /// Seconds per hour.
    pub const SECONDS_PER_HOUR: f64 = 3600.0;
    /// Seconds per day.
    pub const SECONDS_PER_DAY: f64 = 86400.0;
    /// Minutes per hour.
    pub const MINUTES_PER_HOUR: f64 = 60.0;
    /// Hours per day.
    pub const HOURS_PER_DAY: f64 = 24.0;
}

use constants::*;

/// Units of time measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeUnit {
    /// Nanoseconds (ns) - 10^-9 seconds
    Nanoseconds,
    /// Microseconds (µs) - 10^-6 seconds
    Microseconds,
    /// Milliseconds (ms) - 10^-3 seconds
    Milliseconds,
    /// Seconds (s) - SI base unit
    Seconds,
    /// Minutes (min) - 60 seconds
    Minutes,
    /// Hours (h) - 3600 seconds
    Hours,
    /// Days (d) - 86400 seconds
    Days,
}

impl TimeUnit {
    /// All available time units.
    pub const ALL: &'static [TimeUnit] = &[
        TimeUnit::Nanoseconds,
        TimeUnit::Microseconds,
        TimeUnit::Milliseconds,
        TimeUnit::Seconds,
        TimeUnit::Minutes,
        TimeUnit::Hours,
        TimeUnit::Days,
    ];
}

impl_unit_display!(TimeUnit);

impl UnitOfMeasure for TimeUnit {
    fn symbol(&self) -> &'static str {
        match self {
            TimeUnit::Nanoseconds => "ns",
            TimeUnit::Microseconds => "µs",
            TimeUnit::Milliseconds => "ms",
            TimeUnit::Seconds => "s",
            TimeUnit::Minutes => "min",
            TimeUnit::Hours => "h",
            TimeUnit::Days => "d",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            TimeUnit::Nanoseconds => NANO,
            TimeUnit::Microseconds => MICRO,
            TimeUnit::Milliseconds => MILLI,
            TimeUnit::Seconds => 1.0,
            TimeUnit::Minutes => SECONDS_PER_MINUTE,
            TimeUnit::Hours => SECONDS_PER_HOUR,
            TimeUnit::Days => SECONDS_PER_DAY,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            TimeUnit::Nanoseconds
                | TimeUnit::Microseconds
                | TimeUnit::Milliseconds
                | TimeUnit::Seconds
        )
    }
}

/// A quantity of time.
///
/// Time is one of the seven SI base quantities, with the second as its SI base unit.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let t1 = Time::seconds(60.0);
/// let t2 = Time::minutes(1.0);
///
/// // These represent the same duration
/// assert!((t1.to(TimeUnit::Seconds) - t2.to(TimeUnit::Seconds)).abs() < 1e-10);
///
/// // Arithmetic operations
/// let t3 = t1 + t2;
/// assert_eq!(t3.to(TimeUnit::Seconds), 120.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Time {
    value: f64,
    unit: TimeUnit,
}

impl Time {
    /// Creates a new Time quantity.
    pub const fn new_const(value: f64, unit: TimeUnit) -> Self {
        Self { value, unit }
    }

    /// Creates a Time in nanoseconds.
    pub fn nanoseconds(value: f64) -> Self {
        Self::new(value, TimeUnit::Nanoseconds)
    }

    /// Creates a Time in microseconds.
    pub fn microseconds(value: f64) -> Self {
        Self::new(value, TimeUnit::Microseconds)
    }

    /// Creates a Time in milliseconds.
    pub fn milliseconds(value: f64) -> Self {
        Self::new(value, TimeUnit::Milliseconds)
    }

    /// Creates a Time in seconds.
    pub fn seconds(value: f64) -> Self {
        Self::new(value, TimeUnit::Seconds)
    }

    /// Creates a Time in minutes.
    pub fn minutes(value: f64) -> Self {
        Self::new(value, TimeUnit::Minutes)
    }

    /// Creates a Time in hours.
    pub fn hours(value: f64) -> Self {
        Self::new(value, TimeUnit::Hours)
    }

    /// Creates a Time in days.
    pub fn days(value: f64) -> Self {
        Self::new(value, TimeUnit::Days)
    }

    /// Converts this time to nanoseconds.
    pub fn to_nanoseconds(&self) -> f64 {
        self.to(TimeUnit::Nanoseconds)
    }

    /// Converts this time to microseconds.
    pub fn to_microseconds(&self) -> f64 {
        self.to(TimeUnit::Microseconds)
    }

    /// Converts this time to milliseconds.
    pub fn to_milliseconds(&self) -> f64 {
        self.to(TimeUnit::Milliseconds)
    }

    /// Converts this time to seconds.
    pub fn to_seconds(&self) -> f64 {
        self.to(TimeUnit::Seconds)
    }

    /// Converts this time to minutes.
    pub fn to_minutes(&self) -> f64 {
        self.to(TimeUnit::Minutes)
    }

    /// Converts this time to hours.
    pub fn to_hours(&self) -> f64 {
        self.to(TimeUnit::Hours)
    }

    /// Converts this time to days.
    pub fn to_days(&self) -> f64 {
        self.to(TimeUnit::Days)
    }

    /// Returns the value in milliseconds as a long (truncated).
    pub fn millis(&self) -> i64 {
        self.to_milliseconds() as i64
    }
}

impl_quantity!(Time, TimeUnit);

impl_dimension!(
    TimeDimension,
    Time,
    TimeUnit,
    "Time",
    TimeUnit::Seconds,
    TimeUnit::Seconds
);

/// Extension trait for creating Time quantities from numeric types.
pub trait TimeConversions {
    /// Creates a Time in nanoseconds.
    fn nanoseconds(self) -> Time;
    /// Creates a Time in microseconds.
    fn microseconds(self) -> Time;
    /// Creates a Time in milliseconds.
    fn milliseconds(self) -> Time;
    /// Creates a Time in seconds.
    fn seconds(self) -> Time;
    /// Creates a Time in minutes.
    fn minutes(self) -> Time;
    /// Creates a Time in hours.
    fn hours(self) -> Time;
    /// Creates a Time in days.
    fn days(self) -> Time;
}

impl TimeConversions for f64 {
    fn nanoseconds(self) -> Time {
        Time::nanoseconds(self)
    }
    fn microseconds(self) -> Time {
        Time::microseconds(self)
    }
    fn milliseconds(self) -> Time {
        Time::milliseconds(self)
    }
    fn seconds(self) -> Time {
        Time::seconds(self)
    }
    fn minutes(self) -> Time {
        Time::minutes(self)
    }
    fn hours(self) -> Time {
        Time::hours(self)
    }
    fn days(self) -> Time {
        Time::days(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_creation() {
        let t = Time::seconds(60.0);
        assert_eq!(t.value(), 60.0);
        assert_eq!(t.unit(), TimeUnit::Seconds);
    }

    #[test]
    fn test_time_conversions() {
        let t = Time::seconds(3600.0);
        assert_eq!(t.to_hours(), 1.0);
        assert_eq!(t.to_minutes(), 60.0);
        assert_eq!(t.to_milliseconds(), 3_600_000.0);
    }

    #[test]
    fn test_time_arithmetic() {
        let t1 = Time::seconds(30.0);
        let t2 = Time::seconds(30.0);
        let sum = t1 + t2;
        assert_eq!(sum.to_seconds(), 60.0);

        let diff = t1 - Time::seconds(10.0);
        assert_eq!(diff.to_seconds(), 20.0);
    }

    #[test]
    fn test_time_scalar_multiplication() {
        let t = Time::seconds(10.0);
        let doubled = t * 2.0;
        assert_eq!(doubled.to_seconds(), 20.0);

        let tripled = 3.0 * t;
        assert_eq!(tripled.to_seconds(), 30.0);
    }

    #[test]
    fn test_time_division() {
        let t1 = Time::hours(2.0);
        let t2 = Time::hours(1.0);
        assert_eq!(t1 / t2, 2.0);

        let half = t1 / 2.0;
        assert_eq!(half.to_hours(), 1.0);
    }

    #[test]
    fn test_time_comparison() {
        let t1 = Time::hours(1.0);
        let t2 = Time::minutes(60.0);
        assert_eq!(t1, t2);

        let t3 = Time::minutes(30.0);
        assert!(t1 > t3);
    }

    #[test]
    fn test_time_display() {
        let t = Time::seconds(5.5);
        assert_eq!(format!("{}", t), "5.5 s");
    }

    #[test]
    fn test_time_dsl() {
        let t = 5.0.seconds();
        assert_eq!(t.to_seconds(), 5.0);

        let t2 = 100.0.milliseconds();
        assert_eq!(t2.to_milliseconds(), 100.0);
    }

    #[test]
    fn test_unit_symbols() {
        assert_eq!(TimeUnit::Nanoseconds.symbol(), "ns");
        assert_eq!(TimeUnit::Microseconds.symbol(), "µs");
        assert_eq!(TimeUnit::Milliseconds.symbol(), "ms");
        assert_eq!(TimeUnit::Seconds.symbol(), "s");
        assert_eq!(TimeUnit::Minutes.symbol(), "min");
        assert_eq!(TimeUnit::Hours.symbol(), "h");
        assert_eq!(TimeUnit::Days.symbol(), "d");
    }

    #[test]
    fn test_conversion_factors() {
        // 1 day = 86400 seconds
        assert_eq!(TimeUnit::Days.conversion_factor(), 86400.0);
        // 1 hour = 3600 seconds
        assert_eq!(TimeUnit::Hours.conversion_factor(), 3600.0);
        // 1 minute = 60 seconds
        assert_eq!(TimeUnit::Minutes.conversion_factor(), 60.0);
        // 1 second = 1 second (base unit)
        assert_eq!(TimeUnit::Seconds.conversion_factor(), 1.0);
    }

    #[test]
    fn test_cross_unit_operations() {
        let t1 = Time::hours(1.0);
        let t2 = Time::minutes(30.0);
        let sum = t1 + t2;
        assert!((sum.to_minutes() - 90.0).abs() < 1e-10);
    }
}
