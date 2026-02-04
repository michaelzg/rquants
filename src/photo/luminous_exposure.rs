//! Luminous exposure quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Div};

/// Units of luminous exposure measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LuminousExposureUnit {
    /// Lux-seconds (lx·s) - SI unit
    LuxSeconds,
}

impl LuminousExposureUnit {
    /// All available luminous exposure units.
    pub const ALL: &'static [LuminousExposureUnit] = &[LuminousExposureUnit::LuxSeconds];
}

impl_unit_display!(LuminousExposureUnit);

impl UnitOfMeasure for LuminousExposureUnit {
    fn symbol(&self) -> &'static str {
        match self {
            LuminousExposureUnit::LuxSeconds => "lx·s",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            LuminousExposureUnit::LuxSeconds => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, LuminousExposureUnit::LuxSeconds)
    }
}

/// A quantity of luminous exposure.
///
/// Luminous exposure represents the time integral of illuminance.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let exposure = LuminousExposure::lux_seconds(600.0);
/// let time = Time::seconds(3.0);
///
/// // LuminousExposure / Time = Illuminance
/// let illuminance = exposure / time;
/// assert!((illuminance.to_lux() - 200.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct LuminousExposure {
    value: f64,
    unit: LuminousExposureUnit,
}

impl LuminousExposure {
    /// Creates a new LuminousExposure quantity.
    pub const fn new_const(value: f64, unit: LuminousExposureUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a LuminousExposure in lux-seconds.
    pub fn lux_seconds(value: f64) -> Self {
        Self::new(value, LuminousExposureUnit::LuxSeconds)
    }

    // Conversion methods
    /// Converts to lux-seconds.
    pub fn to_lux_seconds(&self) -> f64 {
        self.to(LuminousExposureUnit::LuxSeconds)
    }
}

impl_quantity!(LuminousExposure, LuminousExposureUnit);

// Cross-quantity operations
use super::illuminance::{Illuminance, IlluminanceUnit};
use crate::time::{Time, TimeUnit};

// LuminousExposure / Time = Illuminance
impl Div<Time> for LuminousExposure {
    type Output = Illuminance;

    fn div(self, rhs: Time) -> Self::Output {
        let lux = self.to_lux_seconds() / rhs.to_seconds();
        Illuminance::new(lux, IlluminanceUnit::Lux)
    }
}

// LuminousExposure / Illuminance = Time
impl Div<Illuminance> for LuminousExposure {
    type Output = Time;

    fn div(self, rhs: Illuminance) -> Self::Output {
        let seconds = self.to_lux_seconds() / rhs.to_lux();
        Time::new(seconds, TimeUnit::Seconds)
    }
}

impl_dimension!(
    LuminousExposureDimension,
    LuminousExposure,
    LuminousExposureUnit,
    "LuminousExposure",
    LuminousExposureUnit::LuxSeconds,
    LuminousExposureUnit::LuxSeconds
);

/// Extension trait for creating LuminousExposure quantities from numeric types.
pub trait LuminousExposureConversions {
    /// Creates a LuminousExposure in lux-seconds.
    fn lux_seconds(self) -> LuminousExposure;
}

impl LuminousExposureConversions for f64 {
    fn lux_seconds(self) -> LuminousExposure {
        LuminousExposure::lux_seconds(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luminous_exposure_creation() {
        let e = LuminousExposure::lux_seconds(600.0);
        assert_eq!(e.value(), 600.0);
        assert_eq!(e.unit(), LuminousExposureUnit::LuxSeconds);
    }

    #[test]
    fn test_luminous_exposure_divided_by_time() {
        let e = LuminousExposure::lux_seconds(900.0);
        let t = Time::seconds(3.0);
        let i = e / t;
        // 900 lx·s / 3 s = 300 lx
        assert!((i.to_lux() - 300.0).abs() < 1e-10);
    }

    #[test]
    fn test_illuminance_times_time() {
        let i = Illuminance::lux(150.0);
        let t = Time::seconds(4.0);
        let e = i * t;
        // 150 lx * 4 s = 600 lx·s
        assert!((e.to_lux_seconds() - 600.0).abs() < 1e-10);
    }
}
