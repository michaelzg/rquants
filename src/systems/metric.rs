//! Metric (SI) system prefixes.
//!
//! These prefixes represent powers of 10 and are used throughout the SI system.
//!
//! # Example
//!
//! ```rust
//! use rquants::systems::metric::*;
//!
//! // 5 kilometers = 5 * KILO meters = 5000 meters
//! let km_to_m = 5.0 * KILO;
//! assert_eq!(km_to_m, 5000.0);
//!
//! // 25 milligrams = 25 * MILLI grams = 0.025 grams
//! let mg_to_g = 25.0 * MILLI;
//! assert_eq!(mg_to_g, 0.025);
//! ```

/// Quecto prefix: 10^-30
pub const QUECTO: f64 = 1e-30;

/// Ronto prefix: 10^-27
pub const RONTO: f64 = 1e-27;

/// Yocto prefix: 10^-24
pub const YOCTO: f64 = 1e-24;

/// Zepto prefix: 10^-21
pub const ZEPTO: f64 = 1e-21;

/// Atto prefix: 10^-18
pub const ATTO: f64 = 1e-18;

/// Femto prefix: 10^-15
pub const FEMTO: f64 = 1e-15;

/// Pico prefix: 10^-12
pub const PICO: f64 = 1e-12;

/// Nano prefix: 10^-9
pub const NANO: f64 = 1e-9;

/// Micro prefix: 10^-6
pub const MICRO: f64 = 1e-6;

/// Milli prefix: 10^-3
pub const MILLI: f64 = 1e-3;

/// Centi prefix: 10^-2
pub const CENTI: f64 = 1e-2;

/// Deci prefix: 10^-1
pub const DECI: f64 = 1e-1;

/// Deca (deka) prefix: 10^1
pub const DECA: f64 = 1e1;

/// Hecto prefix: 10^2
pub const HECTO: f64 = 1e2;

/// Kilo prefix: 10^3
pub const KILO: f64 = 1e3;

/// Mega prefix: 10^6
pub const MEGA: f64 = 1e6;

/// Giga prefix: 10^9
pub const GIGA: f64 = 1e9;

/// Tera prefix: 10^12
pub const TERA: f64 = 1e12;

/// Peta prefix: 10^15
pub const PETA: f64 = 1e15;

/// Exa prefix: 10^18
pub const EXA: f64 = 1e18;

/// Zetta prefix: 10^21
pub const ZETTA: f64 = 1e21;

/// Yotta prefix: 10^24
pub const YOTTA: f64 = 1e24;

/// Ronna prefix: 10^27
pub const RONNA: f64 = 1e27;

/// Quetta prefix: 10^30
pub const QUETTA: f64 = 1e30;

/// Returns the metric prefix symbol for a given exponent.
///
/// # Example
///
/// ```rust
/// use rquants::systems::metric::metric_symbol;
///
/// assert_eq!(metric_symbol(3), Some("k"));
/// assert_eq!(metric_symbol(-3), Some("m"));
/// assert_eq!(metric_symbol(0), None);
/// ```
pub const fn metric_symbol(exponent: i32) -> Option<&'static str> {
    match exponent {
        -30 => Some("q"),
        -27 => Some("r"),
        -24 => Some("y"),
        -21 => Some("z"),
        -18 => Some("a"),
        -15 => Some("f"),
        -12 => Some("p"),
        -9 => Some("n"),
        -6 => Some("μ"),
        -3 => Some("m"),
        -2 => Some("c"),
        -1 => Some("d"),
        1 => Some("da"),
        2 => Some("h"),
        3 => Some("k"),
        6 => Some("M"),
        9 => Some("G"),
        12 => Some("T"),
        15 => Some("P"),
        18 => Some("E"),
        21 => Some("Z"),
        24 => Some("Y"),
        27 => Some("R"),
        30 => Some("Q"),
        _ => None,
    }
}

/// Returns the metric prefix value for a given exponent.
///
/// # Example
///
/// ```rust
/// use rquants::systems::metric::metric_prefix;
///
/// assert_eq!(metric_prefix(3), 1000.0);
/// assert_eq!(metric_prefix(-3), 0.001);
/// assert_eq!(metric_prefix(0), 1.0);
/// ```
pub fn metric_prefix(exponent: i32) -> f64 {
    10f64.powi(exponent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_prefixes() {
        assert_eq!(DECA, 10.0);
        assert_eq!(HECTO, 100.0);
        assert_eq!(KILO, 1_000.0);
        assert_eq!(MEGA, 1_000_000.0);
        assert_eq!(GIGA, 1_000_000_000.0);
        assert_eq!(TERA, 1_000_000_000_000.0);
    }

    #[test]
    fn test_negative_prefixes() {
        assert_eq!(DECI, 0.1);
        assert_eq!(CENTI, 0.01);
        assert_eq!(MILLI, 0.001);
        assert_eq!(MICRO, 0.000_001);
        assert_eq!(NANO, 0.000_000_001);
        assert_eq!(PICO, 0.000_000_000_001);
    }

    #[test]
    fn test_metric_symbol() {
        assert_eq!(metric_symbol(3), Some("k"));
        assert_eq!(metric_symbol(6), Some("M"));
        assert_eq!(metric_symbol(-3), Some("m"));
        assert_eq!(metric_symbol(-6), Some("μ"));
        assert_eq!(metric_symbol(0), None);
    }

    #[test]
    fn test_metric_prefix() {
        assert_eq!(metric_prefix(3), KILO);
        assert_eq!(metric_prefix(-3), MILLI);
        assert_eq!(metric_prefix(0), 1.0);
    }

    #[test]
    fn test_prefix_usage() {
        // 5 kilometers in meters
        assert_eq!(5.0 * KILO, 5000.0);

        // 500 milligrams in grams
        assert_eq!(500.0 * MILLI, 0.5);

        // 2.5 gigabytes in bytes
        assert_eq!(2.5 * GIGA, 2_500_000_000.0);
    }
}
