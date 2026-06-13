//! Luminous exposure quantity and units.
use crate::core::Quantity;
use std::ops::Div;
crate::quantity! {
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
    pub quantity LuminousExposure {
        unit: LuminousExposureUnit;
        dimension: LuminousExposureDimension;
        conversions: LuminousExposureConversions;
        name: "LuminousExposure";
        primary: LuxSeconds;
        si: LuxSeconds;

        units {
            /// Lux-seconds (lx·s) - SI unit
            LuxSeconds {
                symbol: "lx·s",
                factor: 1.0,
                ctor: lux_seconds,
                to: to_lux_seconds,
                si: true
            }
        }
    }
}
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

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
