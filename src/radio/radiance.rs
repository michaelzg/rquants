//! Radiance quantity and units.
use crate::core::Quantity;
use std::ops::{Div, Mul};
crate::quantity! {
    /// A quantity of radiance.
    ///
    /// Radiance represents power per unit solid angle per unit area.
    /// SI unit: W/(sr·m²)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
    /// assert_eq!(rad.to_watts_per_steradian_per_square_meter(), 100.0);
    /// ```
    pub quantity Radiance {
        unit: RadianceUnit;
        dimension: RadianceDimension;
        conversions: RadianceConversions;
        name: "Radiance";
        primary: WattsPerSteradianPerSquareMeter;
        si: WattsPerSteradianPerSquareMeter;

        units {
            /// Watts per steradian per square meter (W/(sr·m²)) - SI unit
            WattsPerSteradianPerSquareMeter {
                symbol: "W/(sr·m²)",
                factor: 1.0,
                ctor: watts_per_steradian_per_square_meter,
                to: to_watts_per_steradian_per_square_meter,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use super::radiant_intensity::{RadiantIntensity, RadiantIntensityUnit};
use crate::space::{Area, AreaUnit};

// Radiance * Area = RadiantIntensity
impl Mul<Area> for Radiance {
    type Output = RadiantIntensity;

    fn mul(self, rhs: Area) -> Self::Output {
        let wsr = self.to_watts_per_steradian_per_square_meter() * rhs.to_square_meters();
        RadiantIntensity::new(wsr, RadiantIntensityUnit::WattsPerSteradian)
    }
}

// Area * Radiance = RadiantIntensity
impl Mul<Radiance> for Area {
    type Output = RadiantIntensity;

    fn mul(self, rhs: Radiance) -> Self::Output {
        let wsr = rhs.to_watts_per_steradian_per_square_meter() * self.to_square_meters();
        RadiantIntensity::new(wsr, RadiantIntensityUnit::WattsPerSteradian)
    }
}

// RadiantIntensity / Radiance = Area
impl Div<Radiance> for RadiantIntensity {
    type Output = Area;

    fn div(self, rhs: Radiance) -> Self::Output {
        let m2 = self.to_watts_per_steradian() / rhs.to_watts_per_steradian_per_square_meter();
        Area::new(m2, AreaUnit::SquareMeters)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_radiance_creation() {
        let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
        assert_eq!(rad.value(), 100.0);
        assert_eq!(rad.unit(), RadianceUnit::WattsPerSteradianPerSquareMeter);
    }

    #[test]
    fn test_radiance_conversions() {
        let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
        assert_eq!(rad.to_watts_per_steradian_per_square_meter(), 100.0);
    }

    #[test]
    fn test_radiance_times_area() {
        let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
        let area = Area::square_meters(2.0);
        let ri = rad * area;
        assert_eq!(ri.to_watts_per_steradian(), 200.0);
    }

    #[test]
    fn test_radiant_intensity_divided_by_radiance() {
        let ri = RadiantIntensity::watts_per_steradian(200.0);
        let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
        let area = ri / rad;
        assert_eq!(area.to_square_meters(), 2.0);
    }
}
