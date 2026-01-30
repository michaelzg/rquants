//! Information quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of information measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InformationUnit {
    /// Bytes (B) - primary unit
    Bytes,
    /// Bits (bit)
    Bits,
    /// Kilobytes (KB) - 1000 bytes
    Kilobytes,
    /// Megabytes (MB) - 1000² bytes
    Megabytes,
    /// Gigabytes (GB) - 1000³ bytes
    Gigabytes,
    /// Terabytes (TB) - 1000⁴ bytes
    Terabytes,
    /// Petabytes (PB) - 1000⁵ bytes
    Petabytes,
    /// Exabytes (EB) - 1000⁶ bytes
    Exabytes,
    /// Kibibytes (KiB) - 1024 bytes
    Kibibytes,
    /// Mebibytes (MiB) - 1024² bytes
    Mebibytes,
    /// Gibibytes (GiB) - 1024³ bytes
    Gibibytes,
    /// Tebibytes (TiB) - 1024⁴ bytes
    Tebibytes,
    /// Pebibytes (PiB) - 1024⁵ bytes
    Pebibytes,
    /// Exbibytes (EiB) - 1024⁶ bytes
    Exbibytes,
    /// Kilobits (Kbit) - 1000 bits
    Kilobits,
    /// Megabits (Mbit) - 1000² bits
    Megabits,
    /// Gigabits (Gbit) - 1000³ bits
    Gigabits,
    /// Terabits (Tbit) - 1000⁴ bits
    Terabits,
}

impl InformationUnit {
    /// All available information units.
    pub const ALL: &'static [InformationUnit] = &[
        InformationUnit::Bytes,
        InformationUnit::Bits,
        InformationUnit::Kilobytes,
        InformationUnit::Megabytes,
        InformationUnit::Gigabytes,
        InformationUnit::Terabytes,
        InformationUnit::Petabytes,
        InformationUnit::Exabytes,
        InformationUnit::Kibibytes,
        InformationUnit::Mebibytes,
        InformationUnit::Gibibytes,
        InformationUnit::Tebibytes,
        InformationUnit::Pebibytes,
        InformationUnit::Exbibytes,
        InformationUnit::Kilobits,
        InformationUnit::Megabits,
        InformationUnit::Gigabits,
        InformationUnit::Terabits,
    ];
}

// Conversion factors relative to Bytes
const BITS_PER_BYTE: f64 = 8.0;

impl fmt::Display for InformationUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for InformationUnit {
    fn symbol(&self) -> &'static str {
        match self {
            InformationUnit::Bytes => "B",
            InformationUnit::Bits => "bit",
            InformationUnit::Kilobytes => "KB",
            InformationUnit::Megabytes => "MB",
            InformationUnit::Gigabytes => "GB",
            InformationUnit::Terabytes => "TB",
            InformationUnit::Petabytes => "PB",
            InformationUnit::Exabytes => "EB",
            InformationUnit::Kibibytes => "KiB",
            InformationUnit::Mebibytes => "MiB",
            InformationUnit::Gibibytes => "GiB",
            InformationUnit::Tebibytes => "TiB",
            InformationUnit::Pebibytes => "PiB",
            InformationUnit::Exbibytes => "EiB",
            InformationUnit::Kilobits => "Kbit",
            InformationUnit::Megabits => "Mbit",
            InformationUnit::Gigabits => "Gbit",
            InformationUnit::Terabits => "Tbit",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            InformationUnit::Bytes => 1.0,
            InformationUnit::Bits => 1.0 / BITS_PER_BYTE,
            InformationUnit::Kilobytes => 1e3,
            InformationUnit::Megabytes => 1e6,
            InformationUnit::Gigabytes => 1e9,
            InformationUnit::Terabytes => 1e12,
            InformationUnit::Petabytes => 1e15,
            InformationUnit::Exabytes => 1e18,
            InformationUnit::Kibibytes => 1024.0,
            InformationUnit::Mebibytes => 1024.0 * 1024.0,
            InformationUnit::Gibibytes => 1024.0 * 1024.0 * 1024.0,
            InformationUnit::Tebibytes => 1024.0 * 1024.0 * 1024.0 * 1024.0,
            InformationUnit::Pebibytes => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
            InformationUnit::Exbibytes => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
            InformationUnit::Kilobits => 1e3 / BITS_PER_BYTE,
            InformationUnit::Megabits => 1e6 / BITS_PER_BYTE,
            InformationUnit::Gigabits => 1e9 / BITS_PER_BYTE,
            InformationUnit::Terabits => 1e12 / BITS_PER_BYTE,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            InformationUnit::Bytes
                | InformationUnit::Kilobytes
                | InformationUnit::Megabytes
                | InformationUnit::Gigabytes
                | InformationUnit::Terabytes
                | InformationUnit::Petabytes
                | InformationUnit::Exabytes
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct Information {
    value: f64,
    unit: InformationUnit,
}

impl Information {
    /// Creates a new Information quantity.
    pub const fn new_const(value: f64, unit: InformationUnit) -> Self {
        Self { value, unit }
    }

    // Constructors for byte-based units
    /// Creates Information in bytes.
    pub fn bytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Bytes)
    }

    /// Creates Information in bits.
    pub fn bits(value: f64) -> Self {
        Self::new(value, InformationUnit::Bits)
    }

    /// Creates Information in kilobytes.
    pub fn kilobytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Kilobytes)
    }

    /// Creates Information in megabytes.
    pub fn megabytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Megabytes)
    }

    /// Creates Information in gigabytes.
    pub fn gigabytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Gigabytes)
    }

    /// Creates Information in terabytes.
    pub fn terabytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Terabytes)
    }

    /// Creates Information in petabytes.
    pub fn petabytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Petabytes)
    }

    /// Creates Information in exabytes.
    pub fn exabytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Exabytes)
    }

    /// Creates Information in kibibytes.
    pub fn kibibytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Kibibytes)
    }

    /// Creates Information in mebibytes.
    pub fn mebibytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Mebibytes)
    }

    /// Creates Information in gibibytes.
    pub fn gibibytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Gibibytes)
    }

    /// Creates Information in tebibytes.
    pub fn tebibytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Tebibytes)
    }

    /// Creates Information in pebibytes.
    pub fn pebibytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Pebibytes)
    }

    /// Creates Information in exbibytes.
    pub fn exbibytes(value: f64) -> Self {
        Self::new(value, InformationUnit::Exbibytes)
    }

    /// Creates Information in kilobits.
    pub fn kilobits(value: f64) -> Self {
        Self::new(value, InformationUnit::Kilobits)
    }

    /// Creates Information in megabits.
    pub fn megabits(value: f64) -> Self {
        Self::new(value, InformationUnit::Megabits)
    }

    /// Creates Information in gigabits.
    pub fn gigabits(value: f64) -> Self {
        Self::new(value, InformationUnit::Gigabits)
    }

    /// Creates Information in terabits.
    pub fn terabits(value: f64) -> Self {
        Self::new(value, InformationUnit::Terabits)
    }

    // Conversion methods
    /// Converts to bytes.
    pub fn to_bytes(&self) -> f64 {
        self.to(InformationUnit::Bytes)
    }

    /// Converts to bits.
    pub fn to_bits(&self) -> f64 {
        self.to(InformationUnit::Bits)
    }

    /// Converts to kilobytes.
    pub fn to_kilobytes(&self) -> f64 {
        self.to(InformationUnit::Kilobytes)
    }

    /// Converts to megabytes.
    pub fn to_megabytes(&self) -> f64 {
        self.to(InformationUnit::Megabytes)
    }

    /// Converts to gigabytes.
    pub fn to_gigabytes(&self) -> f64 {
        self.to(InformationUnit::Gigabytes)
    }

    /// Converts to terabytes.
    pub fn to_terabytes(&self) -> f64 {
        self.to(InformationUnit::Terabytes)
    }

    /// Converts to petabytes.
    pub fn to_petabytes(&self) -> f64 {
        self.to(InformationUnit::Petabytes)
    }

    /// Converts to exabytes.
    pub fn to_exabytes(&self) -> f64 {
        self.to(InformationUnit::Exabytes)
    }

    /// Converts to kibibytes.
    pub fn to_kibibytes(&self) -> f64 {
        self.to(InformationUnit::Kibibytes)
    }

    /// Converts to mebibytes.
    pub fn to_mebibytes(&self) -> f64 {
        self.to(InformationUnit::Mebibytes)
    }

    /// Converts to gibibytes.
    pub fn to_gibibytes(&self) -> f64 {
        self.to(InformationUnit::Gibibytes)
    }

    /// Converts to tebibytes.
    pub fn to_tebibytes(&self) -> f64 {
        self.to(InformationUnit::Tebibytes)
    }

    /// Converts to pebibytes.
    pub fn to_pebibytes(&self) -> f64 {
        self.to(InformationUnit::Pebibytes)
    }

    /// Converts to exbibytes.
    pub fn to_exbibytes(&self) -> f64 {
        self.to(InformationUnit::Exbibytes)
    }

    /// Converts to kilobits.
    pub fn to_kilobits(&self) -> f64 {
        self.to(InformationUnit::Kilobits)
    }

    /// Converts to megabits.
    pub fn to_megabits(&self) -> f64 {
        self.to(InformationUnit::Megabits)
    }

    /// Converts to gigabits.
    pub fn to_gigabits(&self) -> f64 {
        self.to(InformationUnit::Gigabits)
    }

    /// Converts to terabits.
    pub fn to_terabits(&self) -> f64 {
        self.to(InformationUnit::Terabits)
    }
}

impl fmt::Display for Information {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Information {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Information {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Information {
    type Unit = InformationUnit;

    fn new(value: f64, unit: Self::Unit) -> Self {
        Self { value, unit }
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> Self::Unit {
        self.unit
    }
}

// Arithmetic operations

impl Add for Information {
    type Output = Information;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Information::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Information {
    type Output = Information;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Information::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Information {
    type Output = Information;

    fn mul(self, rhs: f64) -> Self::Output {
        Information::new(self.value * rhs, self.unit)
    }
}

impl Mul<Information> for f64 {
    type Output = Information;

    fn mul(self, rhs: Information) -> Self::Output {
        Information::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for Information {
    type Output = Information;

    fn div(self, rhs: f64) -> Self::Output {
        Information::new(self.value / rhs, self.unit)
    }
}

impl Div<Information> for Information {
    type Output = f64;

    fn div(self, rhs: Information) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Information {
    type Output = Information;

    fn neg(self) -> Self::Output {
        Information::new(-self.value, self.unit)
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

/// Dimension for Information.
pub struct InformationDimension;

impl Dimension for InformationDimension {
    type Quantity = Information;
    type Unit = InformationUnit;

    fn name() -> &'static str {
        "Information"
    }

    fn primary_unit() -> Self::Unit {
        InformationUnit::Bytes
    }

    fn si_unit() -> Self::Unit {
        InformationUnit::Bytes
    }

    fn units() -> &'static [Self::Unit] {
        InformationUnit::ALL
    }
}

/// Extension trait for creating Information quantities from numeric types.
pub trait InformationConversions {
    /// Creates Information in bytes.
    fn bytes(self) -> Information;
    /// Creates Information in bits.
    fn bits(self) -> Information;
    /// Creates Information in kilobytes.
    fn kilobytes(self) -> Information;
    /// Creates Information in megabytes.
    fn megabytes(self) -> Information;
    /// Creates Information in gigabytes.
    fn gigabytes(self) -> Information;
    /// Creates Information in terabytes.
    fn terabytes(self) -> Information;
    /// Creates Information in petabytes.
    fn petabytes(self) -> Information;
    /// Creates Information in exabytes.
    fn exabytes(self) -> Information;
    /// Creates Information in kibibytes.
    fn kibibytes(self) -> Information;
    /// Creates Information in mebibytes.
    fn mebibytes(self) -> Information;
    /// Creates Information in gibibytes.
    fn gibibytes(self) -> Information;
    /// Creates Information in tebibytes.
    fn tebibytes(self) -> Information;
    /// Creates Information in pebibytes.
    fn pebibytes(self) -> Information;
    /// Creates Information in exbibytes.
    fn exbibytes(self) -> Information;
    /// Creates Information in kilobits.
    fn kilobits(self) -> Information;
    /// Creates Information in megabits.
    fn megabits(self) -> Information;
    /// Creates Information in gigabits.
    fn gigabits(self) -> Information;
    /// Creates Information in terabits.
    fn terabits(self) -> Information;
}

impl InformationConversions for f64 {
    fn bytes(self) -> Information {
        Information::bytes(self)
    }
    fn bits(self) -> Information {
        Information::bits(self)
    }
    fn kilobytes(self) -> Information {
        Information::kilobytes(self)
    }
    fn megabytes(self) -> Information {
        Information::megabytes(self)
    }
    fn gigabytes(self) -> Information {
        Information::gigabytes(self)
    }
    fn terabytes(self) -> Information {
        Information::terabytes(self)
    }
    fn petabytes(self) -> Information {
        Information::petabytes(self)
    }
    fn exabytes(self) -> Information {
        Information::exabytes(self)
    }
    fn kibibytes(self) -> Information {
        Information::kibibytes(self)
    }
    fn mebibytes(self) -> Information {
        Information::mebibytes(self)
    }
    fn gibibytes(self) -> Information {
        Information::gibibytes(self)
    }
    fn tebibytes(self) -> Information {
        Information::tebibytes(self)
    }
    fn pebibytes(self) -> Information {
        Information::pebibytes(self)
    }
    fn exbibytes(self) -> Information {
        Information::exbibytes(self)
    }
    fn kilobits(self) -> Information {
        Information::kilobits(self)
    }
    fn megabits(self) -> Information {
        Information::megabits(self)
    }
    fn gigabits(self) -> Information {
        Information::gigabits(self)
    }
    fn terabits(self) -> Information {
        Information::terabits(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
