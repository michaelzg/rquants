//! Area density quantity and units (mass per area).

use super::mass::{Mass, MassUnit};
use crate::core::Quantity;
use crate::space::area::{Area, AreaUnit};
use std::ops::{Div, Mul};

// Conversion factors to kg/m² (primary unit)
// 1 hectare = 10,000 m², so 1 kg/ha = 0.0001 kg/m²
const KG_PER_HECTARE_FACTOR: f64 = 0.0001;
// 1 cm² = 0.0001 m², 1 g = 0.001 kg, so 1 g/cm² = 0.001/0.0001 = 10 kg/m²
const G_PER_CM2_FACTOR: f64 = 10.0;
// 1 lb ≈ 0.4536 kg, 1 acre ≈ 4046.86 m², so 1 lb/ac ≈ 0.000112 kg/m²
const LB_PER_ACRE_FACTOR: f64 = 0.45359237 / 4046.8564224;
crate::quantity! {
    /// A quantity of area density (mass per area).
    ///
    /// Area density represents the ratio of mass to area, useful for
    /// surface coatings, agricultural applications, and material science.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let coating = AreaDensity::kilograms_per_square_meter(0.5);
    /// let surface = Area::square_meters(10.0);
    ///
    /// // Mass = AreaDensity * Area
    /// let mass = coating * surface;
    /// assert!((mass.to_kilograms() - 5.0).abs() < 1e-10);
    /// ```
    pub quantity AreaDensity {
        unit: AreaDensityUnit;
        dimension: AreaDensityDimension;
        conversions: AreaDensityConversions;
        name: "AreaDensity";
        primary: KilogramsPerSquareMeter;
        si: KilogramsPerSquareMeter;

        units {
            /// Kilograms per square meter (kg/m²) - SI unit
            KilogramsPerSquareMeter {
                symbol: "kg/m²",
                factor: 1.0,
                ctor: kilograms_per_square_meter,
                to: to_kilograms_per_square_meter,
                si: true
            },
            /// Kilograms per hectare (kg/ha)
            KilogramsPerHectare {
                symbol: "kg/ha",
                factor: KG_PER_HECTARE_FACTOR,
                ctor: kilograms_per_hectare,
                to: to_kilograms_per_hectare,
                si: false
            },
            /// Grams per square centimeter (g/cm²)
            GramsPerSquareCentimeter {
                symbol: "g/cm²",
                factor: G_PER_CM2_FACTOR,
                ctor: grams_per_square_centimeter,
                to: to_grams_per_square_centimeter,
                si: true
            },
            /// Pounds per acre (lb/ac)
            PoundsPerAcre {
                symbol: "lb/ac",
                factor: LB_PER_ACRE_FACTOR,
                ctor: pounds_per_acre,
                to: to_pounds_per_acre,
                si: false
            }
        }
    }
}
impl AreaDensity {
    /// Creates an AreaDensity from mass and area.
    pub fn from_mass_and_area(mass: Mass, area: Area) -> Self {
        let kg_per_m2 = mass.to_kilograms() / area.to_square_meters();
        Self::new(kg_per_m2, AreaDensityUnit::KilogramsPerSquareMeter)
    }
}

// AreaDensity * Area = Mass
impl Mul<Area> for AreaDensity {
    type Output = Mass;

    fn mul(self, rhs: Area) -> Self::Output {
        let mass_kg = self.to_kilograms_per_square_meter() * rhs.to_square_meters();
        Mass::new(mass_kg, MassUnit::Kilograms)
    }
}

// Mass / Area = AreaDensity
impl Div<Area> for Mass {
    type Output = AreaDensity;

    fn div(self, rhs: Area) -> Self::Output {
        AreaDensity::from_mass_and_area(self, rhs)
    }
}

// Mass / AreaDensity = Area
impl Div<AreaDensity> for Mass {
    type Output = Area;

    fn div(self, rhs: AreaDensity) -> Self::Output {
        let area_m2 = self.to_kilograms() / rhs.to_kilograms_per_square_meter();
        Area::new(area_m2, AreaUnit::SquareMeters)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_area_density_creation() {
        let d = AreaDensity::kilograms_per_square_meter(1.0);
        assert_eq!(d.value(), 1.0);
        assert_eq!(d.unit(), AreaDensityUnit::KilogramsPerSquareMeter);
    }

    #[test]
    fn test_area_density_conversions() {
        // 1 kg/m² = 10,000 kg/ha
        let d = AreaDensity::kilograms_per_square_meter(1.0);
        assert_eq!(d.to_kilograms_per_hectare(), 10000.0);

        // 1 g/cm² = 10 kg/m²
        let d2 = AreaDensity::grams_per_square_centimeter(1.0);
        assert_eq!(d2.to_kilograms_per_square_meter(), 10.0);
    }

    #[test]
    fn test_area_density_times_area() {
        let density = AreaDensity::kilograms_per_square_meter(2.0);
        let area = Area::square_meters(5.0);
        let mass = density * area;
        assert_eq!(mass.to_kilograms(), 10.0);
    }

    #[test]
    fn test_mass_divided_by_area() {
        let mass = Mass::kilograms(10.0);
        let area = Area::square_meters(5.0);
        let density = mass / area;
        assert_eq!(density.to_kilograms_per_square_meter(), 2.0);
    }

    #[test]
    fn test_mass_divided_by_area_density() {
        let mass = Mass::kilograms(10.0);
        let density = AreaDensity::kilograms_per_square_meter(2.0);
        let area = mass / density;
        assert_eq!(area.to_square_meters(), 5.0);
    }
}
