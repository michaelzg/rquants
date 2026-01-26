//! Binary system prefixes.
//!
//! These prefixes represent powers of 1024 (2^10) and are primarily used
//! for digital information quantities.
//!
//! # Example
//!
//! ```rust
//! use rquants::systems::binary::*;
//!
//! // 5 kibibytes = 5 * KIBI bytes = 5120 bytes
//! let kib_to_b = 5.0 * KIBI;
//! assert_eq!(kib_to_b, 5120.0);
//!
//! // 1 gibibyte = 1 * GIBI bytes = 1,073,741,824 bytes
//! let gib_to_b = 1.0 * GIBI;
//! assert_eq!(gib_to_b, 1_073_741_824.0);
//! ```
//!
//! # Binary vs Metric Prefixes
//!
//! Binary prefixes were introduced to distinguish between powers of 1024
//! and powers of 1000:
//!
//! | Binary | Value | Metric | Value |
//! |--------|-------|--------|-------|
//! | KiB (kibibyte) | 1,024 | KB (kilobyte) | 1,000 |
//! | MiB (mebibyte) | 1,048,576 | MB (megabyte) | 1,000,000 |
//! | GiB (gibibyte) | 1,073,741,824 | GB (gigabyte) | 1,000,000,000 |

/// Kibi prefix: 2^10 = 1,024
pub const KIBI: f64 = 1024.0;

/// Mebi prefix: 2^20 = 1,048,576
pub const MEBI: f64 = 1_048_576.0;

/// Gibi prefix: 2^30 = 1,073,741,824
pub const GIBI: f64 = 1_073_741_824.0;

/// Tebi prefix: 2^40 = 1,099,511,627,776
pub const TEBI: f64 = 1_099_511_627_776.0;

/// Pebi prefix: 2^50 = 1,125,899,906,842,624
pub const PEBI: f64 = 1_125_899_906_842_624.0;

/// Exbi prefix: 2^60 = 1,152,921,504,606,846,976
pub const EXBI: f64 = 1_152_921_504_606_846_976.0;

/// Zebi prefix: 2^70 = 1,180,591,620,717,411,303,424
pub const ZEBI: f64 = 1_180_591_620_717_411_303_424.0;

/// Yobi prefix: 2^80 = 1,208,925,819,614,629,174,706,176
pub const YOBI: f64 = 1_208_925_819_614_629_174_706_176.0;

/// Returns the binary prefix symbol for a given power of 2^10.
///
/// # Example
///
/// ```rust
/// use rquants::systems::binary::binary_symbol;
///
/// assert_eq!(binary_symbol(1), Some("Ki"));
/// assert_eq!(binary_symbol(2), Some("Mi"));
/// assert_eq!(binary_symbol(0), None);
/// ```
pub const fn binary_symbol(power_of_1024: u32) -> Option<&'static str> {
    match power_of_1024 {
        1 => Some("Ki"),
        2 => Some("Mi"),
        3 => Some("Gi"),
        4 => Some("Ti"),
        5 => Some("Pi"),
        6 => Some("Ei"),
        7 => Some("Zi"),
        8 => Some("Yi"),
        _ => None,
    }
}

/// Returns the binary prefix value for a given power of 1024.
///
/// # Example
///
/// ```rust
/// use rquants::systems::binary::binary_prefix;
///
/// assert_eq!(binary_prefix(1), 1024.0);
/// assert_eq!(binary_prefix(2), 1_048_576.0);
/// assert_eq!(binary_prefix(0), 1.0);
/// ```
pub fn binary_prefix(power_of_1024: u32) -> f64 {
    KIBI.powi(power_of_1024 as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_prefixes() {
        assert_eq!(KIBI, 1024.0);
        assert_eq!(MEBI, 1024.0 * 1024.0);
        assert_eq!(GIBI, 1024.0 * 1024.0 * 1024.0);
        assert_eq!(TEBI, 1024.0_f64.powi(4));
    }

    #[test]
    fn test_binary_symbol() {
        assert_eq!(binary_symbol(1), Some("Ki"));
        assert_eq!(binary_symbol(2), Some("Mi"));
        assert_eq!(binary_symbol(3), Some("Gi"));
        assert_eq!(binary_symbol(4), Some("Ti"));
        assert_eq!(binary_symbol(0), None);
        assert_eq!(binary_symbol(9), None);
    }

    #[test]
    fn test_binary_prefix() {
        assert_eq!(binary_prefix(0), 1.0);
        assert_eq!(binary_prefix(1), KIBI);
        assert_eq!(binary_prefix(2), MEBI);
        assert_eq!(binary_prefix(3), GIBI);
    }

    #[test]
    fn test_prefix_usage() {
        // 5 kibibytes in bytes
        assert_eq!(5.0 * KIBI, 5120.0);

        // 1 gibibyte in bytes
        assert_eq!(1.0 * GIBI, 1_073_741_824.0);

        // Compare binary vs metric
        let one_kb_metric = 1000.0;
        let one_kib_binary = KIBI;
        assert!(one_kib_binary > one_kb_metric);
    }

    #[test]
    fn test_binary_vs_metric() {
        use crate::systems::metric::{GIGA, KILO, MEGA};

        // Show the difference between binary and metric prefixes
        assert_eq!(KIBI - KILO, 24.0); // 1024 - 1000
        assert_eq!(MEBI - MEGA, 48_576.0); // 1,048,576 - 1,000,000
        assert_eq!(GIBI - GIGA, 73_741_824.0); // 1,073,741,824 - 1,000,000,000
    }
}
