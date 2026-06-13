//! Information quantity and units.
use crate::core::Quantity;
use std::ops::Div;

// Conversion factors relative to Bytes
const BITS_PER_BYTE: f64 = 8.0;
crate::quantity! {
    /// A quantity of information.
    ///
    /// Information represents digital data quantity.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let data = Information::megabytes(100.0);
    /// let bits = data.to_bits();
    /// assert_eq!(bits, 800_000_000.0);
    /// ```
    pub quantity Information {
        unit: InformationUnit;
        dimension: InformationDimension;
        conversions: InformationConversions;
        name: "Information";
        primary: Bytes;
        si: Bytes;

        units {
            /// Bytes (B) - primary unit
            Bytes {
                symbol: "B",
                factor: 1.0,
                ctor: bytes,
                to: to_bytes,
                si: true
            },
            /// Bits (bit)
            Bits {
                symbol: "bit",
                factor: 1.0 / BITS_PER_BYTE,
                ctor: bits,
                to: to_bits,
                si: false
            },
            /// Kilobytes (KB) - 1000 bytes
            Kilobytes {
                symbol: "KB",
                factor: 1e3,
                ctor: kilobytes,
                to: to_kilobytes,
                si: true
            },
            /// Megabytes (MB) - 1000² bytes
            Megabytes {
                symbol: "MB",
                factor: 1e6,
                ctor: megabytes,
                to: to_megabytes,
                si: true
            },
            /// Gigabytes (GB) - 1000³ bytes
            Gigabytes {
                symbol: "GB",
                factor: 1e9,
                ctor: gigabytes,
                to: to_gigabytes,
                si: true
            },
            /// Terabytes (TB) - 1000⁴ bytes
            Terabytes {
                symbol: "TB",
                factor: 1e12,
                ctor: terabytes,
                to: to_terabytes,
                si: true
            },
            /// Petabytes (PB) - 1000⁵ bytes
            Petabytes {
                symbol: "PB",
                factor: 1e15,
                ctor: petabytes,
                to: to_petabytes,
                si: true
            },
            /// Exabytes (EB) - 1000⁶ bytes
            Exabytes {
                symbol: "EB",
                factor: 1e18,
                ctor: exabytes,
                to: to_exabytes,
                si: true
            },
            /// Kibibytes (KiB) - 1024 bytes
            Kibibytes {
                symbol: "KiB",
                factor: 1024.0,
                ctor: kibibytes,
                to: to_kibibytes,
                si: false
            },
            /// Mebibytes (MiB) - 1024² bytes
            Mebibytes {
                symbol: "MiB",
                factor: 1024.0 * 1024.0,
                ctor: mebibytes,
                to: to_mebibytes,
                si: false
            },
            /// Gibibytes (GiB) - 1024³ bytes
            Gibibytes {
                symbol: "GiB",
                factor: 1024.0 * 1024.0 * 1024.0,
                ctor: gibibytes,
                to: to_gibibytes,
                si: false
            },
            /// Tebibytes (TiB) - 1024⁴ bytes
            Tebibytes {
                symbol: "TiB",
                factor: 1024.0 * 1024.0 * 1024.0 * 1024.0,
                ctor: tebibytes,
                to: to_tebibytes,
                si: false
            },
            /// Pebibytes (PiB) - 1024⁵ bytes
            Pebibytes {
                symbol: "PiB",
                factor: 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
                ctor: pebibytes,
                to: to_pebibytes,
                si: false
            },
            /// Exbibytes (EiB) - 1024⁶ bytes
            Exbibytes {
                symbol: "EiB",
                factor: 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
                ctor: exbibytes,
                to: to_exbibytes,
                si: false
            },
            /// Kilobits (Kbit) - 1000 bits
            Kilobits {
                symbol: "Kbit",
                factor: 1e3 / BITS_PER_BYTE,
                ctor: kilobits,
                to: to_kilobits,
                si: false
            },
            /// Megabits (Mbit) - 1000² bits
            Megabits {
                symbol: "Mbit",
                factor: 1e6 / BITS_PER_BYTE,
                ctor: megabits,
                to: to_megabits,
                si: false
            },
            /// Gigabits (Gbit) - 1000³ bits
            Gigabits {
                symbol: "Gbit",
                factor: 1e9 / BITS_PER_BYTE,
                ctor: gigabits,
                to: to_gigabits,
                si: false
            },
            /// Terabits (Tbit) - 1000⁴ bits
            Terabits {
                symbol: "Tbit",
                factor: 1e12 / BITS_PER_BYTE,
                ctor: terabits,
                to: to_terabits,
                si: false
            }
        }
    }
}
// Cross-quantity operations
use super::data_rate::{DataRate, DataRateUnit};
use crate::time::Time;

// Information / Time = DataRate
impl Div<Time> for Information {
    type Output = DataRate;

    fn div(self, rhs: Time) -> Self::Output {
        let bps = self.to_bytes() / rhs.to_seconds();
        DataRate::new(bps, DataRateUnit::BytesPerSecond)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_information_creation() {
        let info = Information::bytes(1000.0);
        assert_eq!(info.value(), 1000.0);
        assert_eq!(info.unit(), InformationUnit::Bytes);
    }

    #[test]
    fn test_information_conversions() {
        let info = Information::kilobytes(1.0);
        assert_eq!(info.to_bytes(), 1000.0);

        let info2 = Information::megabytes(1.0);
        assert_eq!(info2.to_kilobytes(), 1000.0);
    }

    #[test]
    fn test_bits_conversion() {
        let info = Information::bytes(1.0);
        assert_eq!(info.to_bits(), 8.0);

        let info2 = Information::bits(8.0);
        assert_eq!(info2.to_bytes(), 1.0);
    }

    #[test]
    fn test_binary_prefixes() {
        let info = Information::kibibytes(1.0);
        assert_eq!(info.to_bytes(), 1024.0);

        let info2 = Information::mebibytes(1.0);
        assert_eq!(info2.to_kibibytes(), 1024.0);
    }

    #[test]
    fn test_information_divided_by_time() {
        let info = Information::megabytes(100.0);
        let t = Time::seconds(10.0);
        let rate = info / t;
        // 100 MB / 10 s = 10 MB/s = 10,000,000 B/s
        assert!((rate.to_bytes_per_second() - 10_000_000.0).abs() < 1e-6);
    }
}
