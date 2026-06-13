//! Time quantity and units.

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
crate::quantity! {
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
    pub quantity Time {
        unit: TimeUnit;
        dimension: TimeDimension;
        conversions: TimeConversions;
        name: "Time";
        primary: Seconds;
        si: Seconds;

        units {
            /// Nanoseconds (ns) - 10^-9 seconds
            Nanoseconds {
                symbol: "ns",
                factor: NANO,
                ctor: nanoseconds,
                to: to_nanoseconds,
                si: true
            },
            /// Microseconds (µs) - 10^-6 seconds
            Microseconds {
                symbol: "µs",
                factor: MICRO,
                ctor: microseconds,
                to: to_microseconds,
                si: true
            },
            /// Milliseconds (ms) - 10^-3 seconds
            Milliseconds {
                symbol: "ms",
                factor: MILLI,
                ctor: milliseconds,
                to: to_milliseconds,
                si: true
            },
            /// Seconds (s) - SI base unit
            Seconds {
                symbol: "s",
                factor: 1.0,
                ctor: seconds,
                to: to_seconds,
                si: true
            },
            /// Minutes (min) - 60 seconds
            Minutes {
                symbol: "min",
                factor: SECONDS_PER_MINUTE,
                ctor: minutes,
                to: to_minutes,
                si: false
            },
            /// Hours (h) - 3600 seconds
            Hours {
                symbol: "h",
                factor: SECONDS_PER_HOUR,
                ctor: hours,
                to: to_hours,
                si: false
            },
            /// Days (d) - 86400 seconds
            Days {
                symbol: "d",
                factor: SECONDS_PER_DAY,
                ctor: days,
                to: to_days,
                si: false
            }
        }
    }
}
impl Time {
    /// Returns the value in milliseconds as a long (truncated).
    pub fn millis(&self) -> i64 {
        self.to_milliseconds() as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

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
