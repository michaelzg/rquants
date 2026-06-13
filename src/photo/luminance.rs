//! Luminance quantity and units.
use crate::core::Quantity;
use std::ops::Mul;
crate::quantity! {
    /// A quantity of luminance.
    ///
    /// Luminance represents the luminous intensity per unit area of light traveling
    /// in a given direction.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let luminance = Luminance::candelas_per_square_meter(1000.0);
    /// let area = Area::square_meters(0.5);
    ///
    /// // Luminance * Area = LuminousIntensity
    /// let intensity = luminance * area;
    /// assert!((intensity.to_candelas() - 500.0).abs() < 1e-10);
    /// ```
    pub quantity Luminance {
        unit: LuminanceUnit;
        dimension: LuminanceDimension;
        conversions: LuminanceConversions;
        name: "Luminance";
        primary: CandelasPerSquareMeter;
        si: CandelasPerSquareMeter;

        units {
            /// Candelas per square meter (cd/m²) - SI unit
            CandelasPerSquareMeter {
                symbol: "cd/m²",
                factor: 1.0,
                ctor: candelas_per_square_meter,
                to: to_candelas_per_square_meter,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use super::luminous_intensity::{LuminousIntensity, LuminousIntensityUnit};
use crate::space::Area;

// Luminance * Area = LuminousIntensity
impl Mul<Area> for Luminance {
    type Output = LuminousIntensity;

    fn mul(self, rhs: Area) -> Self::Output {
        let cd = self.to_candelas_per_square_meter() * rhs.to_square_meters();
        LuminousIntensity::new(cd, LuminousIntensityUnit::Candelas)
    }
}

// Area * Luminance = LuminousIntensity
impl Mul<Luminance> for Area {
    type Output = LuminousIntensity;

    fn mul(self, rhs: Luminance) -> Self::Output {
        let cd = rhs.to_candelas_per_square_meter() * self.to_square_meters();
        LuminousIntensity::new(cd, LuminousIntensityUnit::Candelas)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_luminance_creation() {
        let l = Luminance::candelas_per_square_meter(5000.0);
        assert_eq!(l.value(), 5000.0);
        assert_eq!(l.unit(), LuminanceUnit::CandelasPerSquareMeter);
    }

    #[test]
    fn test_luminance_times_area() {
        let l = Luminance::candelas_per_square_meter(200.0);
        let a = Area::square_meters(3.0);
        let i = l * a;
        // 200 cd/m² * 3 m² = 600 cd
        assert!((i.to_candelas() - 600.0).abs() < 1e-10);
    }
}
