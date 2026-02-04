//! Frequency quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use crate::systems::metric::{GIGA, KILO, MEGA, TERA};

/// Units of frequency measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrequencyUnit {
    /// Hertz (Hz) - cycles per second, SI unit
    Hertz,
    /// Kilohertz (kHz) - 10^3 Hz
    Kilohertz,
    /// Megahertz (MHz) - 10^6 Hz
    Megahertz,
    /// Gigahertz (GHz) - 10^9 Hz
    Gigahertz,
    /// Terahertz (THz) - 10^12 Hz
    Terahertz,
    /// Revolutions per minute (rpm)
    RevolutionsPerMinute,
}

impl FrequencyUnit {
    /// All available frequency units.
    pub const ALL: &'static [FrequencyUnit] = &[
        FrequencyUnit::Hertz,
        FrequencyUnit::Kilohertz,
        FrequencyUnit::Megahertz,
        FrequencyUnit::Gigahertz,
        FrequencyUnit::Terahertz,
        FrequencyUnit::RevolutionsPerMinute,
    ];
}

impl_unit_display!(FrequencyUnit);

impl UnitOfMeasure for FrequencyUnit {
    fn symbol(&self) -> &'static str {
        match self {
            FrequencyUnit::Hertz => "Hz",
            FrequencyUnit::Kilohertz => "kHz",
            FrequencyUnit::Megahertz => "MHz",
            FrequencyUnit::Gigahertz => "GHz",
            FrequencyUnit::Terahertz => "THz",
            FrequencyUnit::RevolutionsPerMinute => "rpm",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            FrequencyUnit::Hertz => 1.0,
            FrequencyUnit::Kilohertz => KILO,
            FrequencyUnit::Megahertz => MEGA,
            FrequencyUnit::Gigahertz => GIGA,
            FrequencyUnit::Terahertz => TERA,
            FrequencyUnit::RevolutionsPerMinute => 1.0 / 60.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            FrequencyUnit::Hertz
                | FrequencyUnit::Kilohertz
                | FrequencyUnit::Megahertz
                | FrequencyUnit::Gigahertz
                | FrequencyUnit::Terahertz
        )
    }
}

/// A quantity of frequency.
///
/// Frequency represents the number of cycles or occurrences per unit time.
/// The SI unit is Hertz (Hz), which is cycles per second.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let f1 = Frequency::hertz(1000.0);
/// let f2 = Frequency::kilohertz(1.0);
///
/// // These represent the same frequency
/// assert!((f1.to(FrequencyUnit::Hertz) - f2.to(FrequencyUnit::Hertz)).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Frequency {
    value: f64,
    unit: FrequencyUnit,
}

impl Frequency {
    /// Creates a new Frequency quantity.
    pub const fn new_const(value: f64, unit: FrequencyUnit) -> Self {
        Self { value, unit }
    }

    /// Creates a Frequency in Hertz.
    pub fn hertz(value: f64) -> Self {
        Self::new(value, FrequencyUnit::Hertz)
    }

    /// Creates a Frequency in kilohertz.
    pub fn kilohertz(value: f64) -> Self {
        Self::new(value, FrequencyUnit::Kilohertz)
    }

    /// Creates a Frequency in megahertz.
    pub fn megahertz(value: f64) -> Self {
        Self::new(value, FrequencyUnit::Megahertz)
    }

    /// Creates a Frequency in gigahertz.
    pub fn gigahertz(value: f64) -> Self {
        Self::new(value, FrequencyUnit::Gigahertz)
    }

    /// Creates a Frequency in terahertz.
    pub fn terahertz(value: f64) -> Self {
        Self::new(value, FrequencyUnit::Terahertz)
    }

    /// Creates a Frequency in revolutions per minute.
    pub fn rpm(value: f64) -> Self {
        Self::new(value, FrequencyUnit::RevolutionsPerMinute)
    }

    /// Converts this frequency to Hertz.
    pub fn to_hertz(&self) -> f64 {
        self.to(FrequencyUnit::Hertz)
    }

    /// Converts this frequency to kilohertz.
    pub fn to_kilohertz(&self) -> f64 {
        self.to(FrequencyUnit::Kilohertz)
    }

    /// Converts this frequency to megahertz.
    pub fn to_megahertz(&self) -> f64 {
        self.to(FrequencyUnit::Megahertz)
    }

    /// Converts this frequency to gigahertz.
    pub fn to_gigahertz(&self) -> f64 {
        self.to(FrequencyUnit::Gigahertz)
    }

    /// Converts this frequency to terahertz.
    pub fn to_terahertz(&self) -> f64 {
        self.to(FrequencyUnit::Terahertz)
    }

    /// Converts this frequency to revolutions per minute.
    pub fn to_rpm(&self) -> f64 {
        self.to(FrequencyUnit::RevolutionsPerMinute)
    }

    /// Returns the period (1 / frequency) in seconds.
    pub fn period_seconds(&self) -> f64 {
        1.0 / self.to_hertz()
    }
}

impl_quantity!(Frequency, FrequencyUnit);

impl_dimension!(
    FrequencyDimension,
    Frequency,
    FrequencyUnit,
    "Frequency",
    FrequencyUnit::Hertz,
    FrequencyUnit::Hertz
);

/// Extension trait for creating Frequency quantities from numeric types.
pub trait FrequencyConversions {
    /// Creates a Frequency in Hertz.
    fn hertz(self) -> Frequency;
    /// Creates a Frequency in kilohertz.
    fn kilohertz(self) -> Frequency;
    /// Creates a Frequency in megahertz.
    fn megahertz(self) -> Frequency;
    /// Creates a Frequency in gigahertz.
    fn gigahertz(self) -> Frequency;
    /// Creates a Frequency in terahertz.
    fn terahertz(self) -> Frequency;
    /// Creates a Frequency in revolutions per minute.
    fn rpm(self) -> Frequency;
}

impl FrequencyConversions for f64 {
    fn hertz(self) -> Frequency {
        Frequency::hertz(self)
    }
    fn kilohertz(self) -> Frequency {
        Frequency::kilohertz(self)
    }
    fn megahertz(self) -> Frequency {
        Frequency::megahertz(self)
    }
    fn gigahertz(self) -> Frequency {
        Frequency::gigahertz(self)
    }
    fn terahertz(self) -> Frequency {
        Frequency::terahertz(self)
    }
    fn rpm(self) -> Frequency {
        Frequency::rpm(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_creation() {
        let f = Frequency::hertz(1000.0);
        assert_eq!(f.value(), 1000.0);
        assert_eq!(f.unit(), FrequencyUnit::Hertz);
    }

    #[test]
    fn test_frequency_conversions() {
        let f = Frequency::hertz(1_000_000.0);
        assert_eq!(f.to_kilohertz(), 1000.0);
        assert_eq!(f.to_megahertz(), 1.0);
    }

    #[test]
    fn test_frequency_arithmetic() {
        let f1 = Frequency::hertz(500.0);
        let f2 = Frequency::hertz(500.0);
        let sum = f1 + f2;
        assert_eq!(sum.to_hertz(), 1000.0);
    }

    #[test]
    fn test_frequency_comparison() {
        let f1 = Frequency::kilohertz(1.0);
        let f2 = Frequency::hertz(1000.0);
        assert_eq!(f1, f2);
    }

    #[test]
    fn test_frequency_display() {
        let f = Frequency::megahertz(2.4);
        assert_eq!(format!("{}", f), "2.4 MHz");
    }

    #[test]
    fn test_frequency_dsl() {
        let f = 100.0.hertz();
        assert_eq!(f.to_hertz(), 100.0);
    }

    #[test]
    fn test_rpm_conversion() {
        let f = Frequency::rpm(60.0);
        // 60 rpm = 1 Hz
        assert!((f.to_hertz() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_period() {
        let f = Frequency::hertz(2.0);
        assert_eq!(f.period_seconds(), 0.5);
    }
}
