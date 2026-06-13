//! Luminous intensity quantity and units.
use crate::core::Quantity;
use std::ops::{Div, Mul};
crate::quantity! {
    /// A quantity of luminous intensity.
    ///
    /// Luminous intensity represents the luminous flux emitted per unit solid angle
    /// in a particular direction.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let intensity = LuminousIntensity::candelas(100.0);
    /// let solid_angle = SolidAngle::steradians(2.0);
    ///
    /// // LuminousIntensity * SolidAngle = LuminousFlux
    /// let flux = intensity * solid_angle;
    /// assert!((flux.to_lumens() - 200.0).abs() < 1e-10);
    /// ```
    pub quantity LuminousIntensity {
        unit: LuminousIntensityUnit;
        dimension: LuminousIntensityDimension;
        conversions: LuminousIntensityConversions;
        name: "LuminousIntensity";
        primary: Candelas;
        si: Candelas;

        units {
            /// Candelas (cd) - SI unit
            Candelas {
                symbol: "cd",
                factor: 1.0,
                ctor: candelas,
                to: to_candelas,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use super::luminance::{Luminance, LuminanceUnit};
use super::luminous_flux::{LuminousFlux, LuminousFluxUnit};
use crate::space::{Area, AreaUnit, SolidAngle};

// LuminousIntensity * SolidAngle = LuminousFlux
impl Mul<SolidAngle> for LuminousIntensity {
    type Output = LuminousFlux;

    fn mul(self, rhs: SolidAngle) -> Self::Output {
        let lumens = self.to_candelas() * rhs.to_steradians();
        LuminousFlux::new(lumens, LuminousFluxUnit::Lumens)
    }
}

// SolidAngle * LuminousIntensity = LuminousFlux
impl Mul<LuminousIntensity> for SolidAngle {
    type Output = LuminousFlux;

    fn mul(self, rhs: LuminousIntensity) -> Self::Output {
        let lumens = rhs.to_candelas() * self.to_steradians();
        LuminousFlux::new(lumens, LuminousFluxUnit::Lumens)
    }
}

// LuminousIntensity / Area = Luminance
impl Div<Area> for LuminousIntensity {
    type Output = Luminance;

    fn div(self, rhs: Area) -> Self::Output {
        let cdpm2 = self.to_candelas() / rhs.to_square_meters();
        Luminance::new(cdpm2, LuminanceUnit::CandelasPerSquareMeter)
    }
}

// LuminousIntensity / Luminance = Area
impl Div<Luminance> for LuminousIntensity {
    type Output = Area;

    fn div(self, rhs: Luminance) -> Self::Output {
        let m2 = self.to_candelas() / rhs.to_candelas_per_square_meter();
        Area::new(m2, AreaUnit::SquareMeters)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_luminous_intensity_creation() {
        let i = LuminousIntensity::candelas(100.0);
        assert_eq!(i.value(), 100.0);
        assert_eq!(i.unit(), LuminousIntensityUnit::Candelas);
    }

    #[test]
    fn test_luminous_intensity_times_solid_angle() {
        let i = LuminousIntensity::candelas(50.0);
        let sa = SolidAngle::steradians(2.0);
        let f = i * sa;
        // 50 cd * 2 sr = 100 lm
        assert!((f.to_lumens() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_luminous_intensity_divided_by_area() {
        let i = LuminousIntensity::candelas(100.0);
        let a = Area::square_meters(2.0);
        let l = i / a;
        // 100 cd / 2 m² = 50 cd/m²
        assert!((l.to_candelas_per_square_meter() - 50.0).abs() < 1e-10);
    }
}
