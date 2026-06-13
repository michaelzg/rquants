//! Area quantity and units.

use super::length::Length;
use super::volume::{Volume, VolumeUnit};
use crate::core::Quantity;
use crate::systems::metric::{CENTI, HECTO, KILO, MILLI};
use std::ops::Mul;

/// Conversion factors
const SQ_FOOT_TO_SQ_METER: f64 = 0.09290304;
const SQ_YARD_TO_SQ_METER: f64 = 0.83612736;
const SQ_MILE_TO_SQ_METER: f64 = 2_589_988.110336;
const ACRE_TO_SQ_METER: f64 = 4046.8564224;
const SQ_INCH_TO_SQ_METER: f64 = 0.00064516;
crate::quantity! {
    /// A quantity of area.
    ///
    /// Area represents a two-dimensional extent, the product of two lengths.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let a1 = Area::square_meters(100.0);
    /// let a2 = Area::hectares(0.01);
    ///
    /// // These represent the same area
    /// assert!((a1.to_square_meters() - a2.to_square_meters()).abs() < 1e-10);
    /// ```
    pub quantity Area {
        unit: AreaUnit;
        dimension: AreaDimension;
        conversions: AreaConversions;
        name: "Area";
        primary: SquareMeters;
        si: SquareMeters;

        units {
            /// Square millimeters (mm²)
            SquareMillimeters {
                symbol: "mm²",
                factor: MILLI * MILLI,
                ctor: square_millimeters,
                to: to_square_millimeters,
                si: true
            },
            /// Square centimeters (cm²)
            SquareCentimeters {
                symbol: "cm²",
                factor: CENTI * CENTI,
                ctor: square_centimeters,
                to: to_square_centimeters,
                si: true
            },
            /// Square meters (m²) - SI derived unit
            SquareMeters {
                symbol: "m²",
                factor: 1.0,
                ctor: square_meters,
                to: to_square_meters,
                si: true
            },
            /// Square kilometers (km²)
            SquareKilometers {
                symbol: "km²",
                factor: KILO * KILO,
                ctor: square_kilometers,
                to: to_square_kilometers,
                si: true
            },
            /// Hectares (ha) - 10,000 m²
            Hectares {
                symbol: "ha",
                factor: HECTO * HECTO,
                ctor: hectares,
                to: to_hectares,
                si: true
            },
            /// Square inches (in²)
            SquareInches {
                symbol: "in²",
                factor: SQ_INCH_TO_SQ_METER,
                ctor: square_inches,
                to: to_square_inches,
                si: false
            },
            /// Square feet (ft²)
            SquareFeet {
                symbol: "ft²",
                factor: SQ_FOOT_TO_SQ_METER,
                ctor: square_feet,
                to: to_square_feet,
                si: false
            },
            /// Square yards (yd²)
            SquareYards {
                symbol: "yd²",
                factor: SQ_YARD_TO_SQ_METER,
                ctor: square_yards,
                to: to_square_yards,
                si: false
            },
            /// Square miles (mi²)
            SquareMiles {
                symbol: "mi²",
                factor: SQ_MILE_TO_SQ_METER,
                ctor: square_miles,
                to: to_square_miles,
                si: false
            },
            /// Acres
            Acres {
                symbol: "ac",
                factor: ACRE_TO_SQ_METER,
                ctor: acres,
                to: to_acres,
                si: false
            }
        }
    }
}

// Area * Length = Volume
impl Mul<Length> for Area {
    type Output = Volume;

    fn mul(self, rhs: Length) -> Self::Output {
        let volume_meters_cu = self.to_square_meters() * rhs.to_meters();
        Volume::new(volume_meters_cu, VolumeUnit::CubicMeters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_area_creation() {
        let a = Area::square_meters(100.0);
        assert_eq!(a.value(), 100.0);
        assert_eq!(a.unit(), AreaUnit::SquareMeters);
    }

    #[test]
    fn test_area_conversions() {
        let a = Area::square_meters(10000.0);
        assert_eq!(a.to_hectares(), 1.0);

        let a2 = Area::square_kilometers(1.0);
        assert_eq!(a2.to_hectares(), 100.0);
    }

    #[test]
    fn test_area_arithmetic() {
        let a1 = Area::square_meters(50.0);
        let a2 = Area::square_meters(50.0);
        let sum = a1 + a2;
        assert_eq!(sum.to_square_meters(), 100.0);
    }

    #[test]
    fn test_imperial_conversions() {
        let a = Area::acres(1.0);
        // 1 acre ≈ 43,560 square feet
        assert!((a.to_square_feet() - 43560.0).abs() < 1.0);
    }
}
