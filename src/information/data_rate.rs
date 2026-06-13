//! DataRate quantity and units.
use crate::core::Quantity;
use std::ops::{Div, Mul};

// Conversion factors relative to BytesPerSecond
const BITS_PER_BYTE: f64 = 8.0;
crate::quantity! {
    /// A quantity of data rate.
    ///
    /// DataRate represents the rate of information transfer.
    /// DataRate = Information / Time
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let rate = DataRate::megabits_per_second(100.0);
    /// let time = Time::seconds(10.0);
    ///
    /// // Information = DataRate * Time
    /// let info = rate * time;
    /// assert!((info.to_megabits() - 1000.0).abs() < 1e-10);
    /// ```
    pub quantity DataRate {
        unit: DataRateUnit;
        dimension: DataRateDimension;
        conversions: DataRateConversions;
        name: "DataRate";
        primary: BytesPerSecond;
        si: BytesPerSecond;

        units {
            /// Bytes per second (B/s) - primary unit
            BytesPerSecond {
                symbol: "B/s",
                factor: 1.0,
                ctor: bytes_per_second,
                to: to_bytes_per_second,
                si: true
            },
            /// Bits per second (bps)
            BitsPerSecond {
                symbol: "bps",
                factor: 1.0 / BITS_PER_BYTE,
                ctor: bits_per_second,
                to: to_bits_per_second,
                si: false
            },
            /// Kilobytes per second (KB/s)
            KilobytesPerSecond {
                symbol: "KB/s",
                factor: 1e3,
                ctor: kilobytes_per_second,
                to: to_kilobytes_per_second,
                si: true
            },
            /// Megabytes per second (MB/s)
            MegabytesPerSecond {
                symbol: "MB/s",
                factor: 1e6,
                ctor: megabytes_per_second,
                to: to_megabytes_per_second,
                si: true
            },
            /// Gigabytes per second (GB/s)
            GigabytesPerSecond {
                symbol: "GB/s",
                factor: 1e9,
                ctor: gigabytes_per_second,
                to: to_gigabytes_per_second,
                si: true
            },
            /// Kilobits per second (Kbps)
            KilobitsPerSecond {
                symbol: "Kbps",
                factor: 1e3 / BITS_PER_BYTE,
                ctor: kilobits_per_second,
                to: to_kilobits_per_second,
                si: false
            },
            /// Megabits per second (Mbps)
            MegabitsPerSecond {
                symbol: "Mbps",
                factor: 1e6 / BITS_PER_BYTE,
                ctor: megabits_per_second,
                to: to_megabits_per_second,
                si: false
            },
            /// Gigabits per second (Gbps)
            GigabitsPerSecond {
                symbol: "Gbps",
                factor: 1e9 / BITS_PER_BYTE,
                ctor: gigabits_per_second,
                to: to_gigabits_per_second,
                si: false
            }
        }
    }
}
// Cross-quantity operations
use super::information::{Information, InformationUnit};
use crate::time::{Time, TimeUnit};

// DataRate * Time = Information
impl Mul<Time> for DataRate {
    type Output = Information;

    fn mul(self, rhs: Time) -> Self::Output {
        let bytes = self.to_bytes_per_second() * rhs.to_seconds();
        Information::new(bytes, InformationUnit::Bytes)
    }
}

// Time * DataRate = Information
impl Mul<DataRate> for Time {
    type Output = Information;

    fn mul(self, rhs: DataRate) -> Self::Output {
        let bytes = rhs.to_bytes_per_second() * self.to_seconds();
        Information::new(bytes, InformationUnit::Bytes)
    }
}

// Information / DataRate = Time
impl Div<DataRate> for Information {
    type Output = Time;

    fn div(self, rhs: DataRate) -> Self::Output {
        let seconds = self.to_bytes() / rhs.to_bytes_per_second();
        Time::new(seconds, TimeUnit::Seconds)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_data_rate_creation() {
        let rate = DataRate::bytes_per_second(1000.0);
        assert_eq!(rate.value(), 1000.0);
        assert_eq!(rate.unit(), DataRateUnit::BytesPerSecond);
    }

    #[test]
    fn test_data_rate_conversions() {
        let rate = DataRate::kilobytes_per_second(1.0);
        assert_eq!(rate.to_bytes_per_second(), 1000.0);

        let rate2 = DataRate::megabytes_per_second(1.0);
        assert_eq!(rate2.to_kilobytes_per_second(), 1000.0);
    }

    #[test]
    fn test_bits_per_second_conversion() {
        let rate = DataRate::bytes_per_second(1.0);
        assert_eq!(rate.to_bits_per_second(), 8.0);

        let rate2 = DataRate::bits_per_second(8.0);
        assert_eq!(rate2.to_bytes_per_second(), 1.0);
    }

    #[test]
    fn test_data_rate_times_time() {
        let rate = DataRate::megabytes_per_second(10.0);
        let t = Time::seconds(10.0);
        let info = rate * t;
        // 10 MB/s * 10 s = 100 MB = 100,000,000 B
        assert!((info.to_bytes() - 100_000_000.0).abs() < 1e-6);
    }

    #[test]
    fn test_information_divided_by_data_rate() {
        let info = Information::megabytes(100.0);
        let rate = DataRate::megabytes_per_second(10.0);
        let time = info / rate;
        // 100 MB / 10 MB/s = 10 s
        assert!((time.to_seconds() - 10.0).abs() < 1e-10);
    }
}
