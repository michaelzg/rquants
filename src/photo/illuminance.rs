//! Illuminance quantity and units.
use crate::core::Quantity;
use std::ops::Mul;
crate::quantity! {
    /// A quantity of illuminance.
    ///
    /// Illuminance represents the luminous flux incident on a surface per unit area.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let illuminance = Illuminance::lux(500.0);
    /// let area = Area::square_meters(4.0);
    ///
    /// // Illuminance * Area = LuminousFlux
    /// let flux = illuminance * area;
    /// assert!((flux.to_lumens() - 2000.0).abs() < 1e-10);
    /// ```
    pub quantity Illuminance {
        unit: IlluminanceUnit;
        dimension: IlluminanceDimension;
        conversions: IlluminanceConversions;
        name: "Illuminance";
        primary: Lux;
        si: Lux;

        units {
            /// Lux (lx) - SI unit (lm/m²)
            Lux {
                symbol: "lx",
                factor: 1.0,
                ctor: lux,
                to: to_lux,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use super::luminous_exposure::{LuminousExposure, LuminousExposureUnit};
use super::luminous_flux::{LuminousFlux, LuminousFluxUnit};
use crate::space::Area;
use crate::time::Time;

// Illuminance * Area = LuminousFlux
impl Mul<Area> for Illuminance {
    type Output = LuminousFlux;

    fn mul(self, rhs: Area) -> Self::Output {
        let lumens = self.to_lux() * rhs.to_square_meters();
        LuminousFlux::new(lumens, LuminousFluxUnit::Lumens)
    }
}

// Area * Illuminance = LuminousFlux
impl Mul<Illuminance> for Area {
    type Output = LuminousFlux;

    fn mul(self, rhs: Illuminance) -> Self::Output {
        let lumens = rhs.to_lux() * self.to_square_meters();
        LuminousFlux::new(lumens, LuminousFluxUnit::Lumens)
    }
}

// Illuminance * Time = LuminousExposure
impl Mul<Time> for Illuminance {
    type Output = LuminousExposure;

    fn mul(self, rhs: Time) -> Self::Output {
        let lx_s = self.to_lux() * rhs.to_seconds();
        LuminousExposure::new(lx_s, LuminousExposureUnit::LuxSeconds)
    }
}

// Time * Illuminance = LuminousExposure
impl Mul<Illuminance> for Time {
    type Output = LuminousExposure;

    fn mul(self, rhs: Illuminance) -> Self::Output {
        let lx_s = rhs.to_lux() * self.to_seconds();
        LuminousExposure::new(lx_s, LuminousExposureUnit::LuxSeconds)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_illuminance_creation() {
        let i = Illuminance::lux(500.0);
        assert_eq!(i.value(), 500.0);
        assert_eq!(i.unit(), IlluminanceUnit::Lux);
    }

    #[test]
    fn test_illuminance_times_area() {
        let i = Illuminance::lux(100.0);
        let a = Area::square_meters(5.0);
        let f = i * a;
        // 100 lx * 5 m² = 500 lm
        assert!((f.to_lumens() - 500.0).abs() < 1e-10);
    }

    #[test]
    fn test_illuminance_times_time() {
        let i = Illuminance::lux(200.0);
        let t = Time::seconds(3.0);
        let e = i * t;
        // 200 lx * 3 s = 600 lx·s
        assert!((e.to_lux_seconds() - 600.0).abs() < 1e-10);
    }
}
