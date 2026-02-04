//! DataRate quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Div, Mul};

/// Units of data rate measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataRateUnit {
    /// Bytes per second (B/s) - primary unit
    BytesPerSecond,
    /// Bits per second (bps)
    BitsPerSecond,
    /// Kilobytes per second (KB/s)
    KilobytesPerSecond,
    /// Megabytes per second (MB/s)
    MegabytesPerSecond,
    /// Gigabytes per second (GB/s)
    GigabytesPerSecond,
    /// Kilobits per second (Kbps)
    KilobitsPerSecond,
    /// Megabits per second (Mbps)
    MegabitsPerSecond,
    /// Gigabits per second (Gbps)
    GigabitsPerSecond,
}

impl DataRateUnit {
    /// All available data rate units.
    pub const ALL: &'static [DataRateUnit] = &[
        DataRateUnit::BytesPerSecond,
        DataRateUnit::BitsPerSecond,
        DataRateUnit::KilobytesPerSecond,
        DataRateUnit::MegabytesPerSecond,
        DataRateUnit::GigabytesPerSecond,
        DataRateUnit::KilobitsPerSecond,
        DataRateUnit::MegabitsPerSecond,
        DataRateUnit::GigabitsPerSecond,
    ];
}

// Conversion factors relative to BytesPerSecond
const BITS_PER_BYTE: f64 = 8.0;

impl_unit_display!(DataRateUnit);

impl UnitOfMeasure for DataRateUnit {
    fn symbol(&self) -> &'static str {
        match self {
            DataRateUnit::BytesPerSecond => "B/s",
            DataRateUnit::BitsPerSecond => "bps",
            DataRateUnit::KilobytesPerSecond => "KB/s",
            DataRateUnit::MegabytesPerSecond => "MB/s",
            DataRateUnit::GigabytesPerSecond => "GB/s",
            DataRateUnit::KilobitsPerSecond => "Kbps",
            DataRateUnit::MegabitsPerSecond => "Mbps",
            DataRateUnit::GigabitsPerSecond => "Gbps",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            DataRateUnit::BytesPerSecond => 1.0,
            DataRateUnit::BitsPerSecond => 1.0 / BITS_PER_BYTE,
            DataRateUnit::KilobytesPerSecond => 1e3,
            DataRateUnit::MegabytesPerSecond => 1e6,
            DataRateUnit::GigabytesPerSecond => 1e9,
            DataRateUnit::KilobitsPerSecond => 1e3 / BITS_PER_BYTE,
            DataRateUnit::MegabitsPerSecond => 1e6 / BITS_PER_BYTE,
            DataRateUnit::GigabitsPerSecond => 1e9 / BITS_PER_BYTE,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            DataRateUnit::BytesPerSecond
                | DataRateUnit::KilobytesPerSecond
                | DataRateUnit::MegabytesPerSecond
                | DataRateUnit::GigabytesPerSecond
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct DataRate {
    value: f64,
    unit: DataRateUnit,
}

impl DataRate {
    /// Creates a new DataRate quantity.
    pub const fn new_const(value: f64, unit: DataRateUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a DataRate in bytes per second.
    pub fn bytes_per_second(value: f64) -> Self {
        Self::new(value, DataRateUnit::BytesPerSecond)
    }

    /// Creates a DataRate in bits per second.
    pub fn bits_per_second(value: f64) -> Self {
        Self::new(value, DataRateUnit::BitsPerSecond)
    }

    /// Creates a DataRate in kilobytes per second.
    pub fn kilobytes_per_second(value: f64) -> Self {
        Self::new(value, DataRateUnit::KilobytesPerSecond)
    }

    /// Creates a DataRate in megabytes per second.
    pub fn megabytes_per_second(value: f64) -> Self {
        Self::new(value, DataRateUnit::MegabytesPerSecond)
    }

    /// Creates a DataRate in gigabytes per second.
    pub fn gigabytes_per_second(value: f64) -> Self {
        Self::new(value, DataRateUnit::GigabytesPerSecond)
    }

    /// Creates a DataRate in kilobits per second.
    pub fn kilobits_per_second(value: f64) -> Self {
        Self::new(value, DataRateUnit::KilobitsPerSecond)
    }

    /// Creates a DataRate in megabits per second.
    pub fn megabits_per_second(value: f64) -> Self {
        Self::new(value, DataRateUnit::MegabitsPerSecond)
    }

    /// Creates a DataRate in gigabits per second.
    pub fn gigabits_per_second(value: f64) -> Self {
        Self::new(value, DataRateUnit::GigabitsPerSecond)
    }

    // Conversion methods
    /// Converts to bytes per second.
    pub fn to_bytes_per_second(&self) -> f64 {
        self.to(DataRateUnit::BytesPerSecond)
    }

    /// Converts to bits per second.
    pub fn to_bits_per_second(&self) -> f64 {
        self.to(DataRateUnit::BitsPerSecond)
    }

    /// Converts to kilobytes per second.
    pub fn to_kilobytes_per_second(&self) -> f64 {
        self.to(DataRateUnit::KilobytesPerSecond)
    }

    /// Converts to megabytes per second.
    pub fn to_megabytes_per_second(&self) -> f64 {
        self.to(DataRateUnit::MegabytesPerSecond)
    }

    /// Converts to gigabytes per second.
    pub fn to_gigabytes_per_second(&self) -> f64 {
        self.to(DataRateUnit::GigabytesPerSecond)
    }

    /// Converts to kilobits per second.
    pub fn to_kilobits_per_second(&self) -> f64 {
        self.to(DataRateUnit::KilobitsPerSecond)
    }

    /// Converts to megabits per second.
    pub fn to_megabits_per_second(&self) -> f64 {
        self.to(DataRateUnit::MegabitsPerSecond)
    }

    /// Converts to gigabits per second.
    pub fn to_gigabits_per_second(&self) -> f64 {
        self.to(DataRateUnit::GigabitsPerSecond)
    }
}

impl_quantity!(DataRate, DataRateUnit);

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

impl_dimension!(
    DataRateDimension,
    DataRate,
    DataRateUnit,
    "DataRate",
    DataRateUnit::BytesPerSecond,
    DataRateUnit::BytesPerSecond
);

/// Extension trait for creating DataRate quantities from numeric types.
pub trait DataRateConversions {
    /// Creates a DataRate in bytes per second.
    fn bytes_per_second(self) -> DataRate;
    /// Creates a DataRate in bits per second.
    fn bits_per_second(self) -> DataRate;
    /// Creates a DataRate in kilobytes per second.
    fn kilobytes_per_second(self) -> DataRate;
    /// Creates a DataRate in megabytes per second.
    fn megabytes_per_second(self) -> DataRate;
    /// Creates a DataRate in gigabytes per second.
    fn gigabytes_per_second(self) -> DataRate;
    /// Creates a DataRate in kilobits per second.
    fn kilobits_per_second(self) -> DataRate;
    /// Creates a DataRate in megabits per second.
    fn megabits_per_second(self) -> DataRate;
    /// Creates a DataRate in gigabits per second.
    fn gigabits_per_second(self) -> DataRate;
}

impl DataRateConversions for f64 {
    fn bytes_per_second(self) -> DataRate {
        DataRate::bytes_per_second(self)
    }
    fn bits_per_second(self) -> DataRate {
        DataRate::bits_per_second(self)
    }
    fn kilobytes_per_second(self) -> DataRate {
        DataRate::kilobytes_per_second(self)
    }
    fn megabytes_per_second(self) -> DataRate {
        DataRate::megabytes_per_second(self)
    }
    fn gigabytes_per_second(self) -> DataRate {
        DataRate::gigabytes_per_second(self)
    }
    fn kilobits_per_second(self) -> DataRate {
        DataRate::kilobits_per_second(self)
    }
    fn megabits_per_second(self) -> DataRate {
        DataRate::megabits_per_second(self)
    }
    fn gigabits_per_second(self) -> DataRate {
        DataRate::gigabits_per_second(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
