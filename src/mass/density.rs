//! Density quantity and units.

use super::mass::{Mass, MassUnit};
use crate::core::Quantity;
use crate::space::volume::{Volume, VolumeUnit};
use std::ops::{Div, Mul};

// Conversion factors to kg/m³ (primary unit)
// 1 L = 0.001 m³, so 1 kg/L = 1000 kg/m³
const KG_PER_LITER_FACTOR: f64 = 1000.0;
// 1 g/L = 1 kg/m³
const G_PER_LITER_FACTOR: f64 = 1.0;
// 1 mg/L = 0.001 kg/m³
const MG_PER_LITER_FACTOR: f64 = 0.001;
// 1 g/mL = 1000 kg/m³ (same as g/cm³)
const G_PER_ML_FACTOR: f64 = 1000.0;
// 1 lb/ft³ ≈ 16.0185 kg/m³
const LB_PER_CUFT_FACTOR: f64 = 16.01846337396;
// 1 lb/gal (US) ≈ 119.826 kg/m³
const LB_PER_GAL_FACTOR: f64 = 119.8264273167;
crate::quantity! {
    /// A quantity of density (mass per volume).
    ///
    /// Density represents the ratio of mass to volume.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// // Water density at room temperature
    /// let water = Density::kilograms_per_cubic_meter(1000.0);
    /// let volume = Volume::liters(1.0);
    ///
    /// // Mass = Density * Volume
    /// let mass = water * volume;
    /// assert!((mass.to_kilograms() - 1.0).abs() < 1e-10);
    /// ```
    pub quantity Density {
        unit: DensityUnit;
        dimension: DensityDimension;
        conversions: DensityConversions;
        name: "Density";
        primary: KilogramsPerCubicMeter;
        si: KilogramsPerCubicMeter;

        units {
            /// Kilograms per cubic meter (kg/m³) - SI unit
            KilogramsPerCubicMeter {
                symbol: "kg/m³",
                factor: 1.0,
                ctor: kilograms_per_cubic_meter,
                to: to_kilograms_per_cubic_meter,
                si: true
            },
            /// Kilograms per liter (kg/L)
            KilogramsPerLiter {
                symbol: "kg/L",
                factor: KG_PER_LITER_FACTOR,
                ctor: kilograms_per_liter,
                to: to_kilograms_per_liter,
                si: true
            },
            /// Grams per liter (g/L)
            GramsPerLiter {
                symbol: "g/L",
                factor: G_PER_LITER_FACTOR,
                ctor: grams_per_liter,
                to: to_grams_per_liter,
                si: true
            },
            /// Milligrams per liter (mg/L)
            MilligramsPerLiter {
                symbol: "mg/L",
                factor: MG_PER_LITER_FACTOR,
                ctor: milligrams_per_liter,
                to: to_milligrams_per_liter,
                si: true
            },
            /// Grams per milliliter (g/mL)
            GramsPerMilliliter {
                symbol: "g/mL",
                factor: G_PER_ML_FACTOR,
                ctor: grams_per_milliliter,
                to: to_grams_per_milliliter,
                si: true
            },
            /// Grams per cubic centimeter (g/cm³)
            GramsPerCubicCentimeter {
                symbol: "g/cm³",
                factor: G_PER_ML_FACTOR,
                ctor: grams_per_cubic_centimeter,
                to: to_grams_per_cubic_centimeter,
                si: true
            },
            /// Pounds per cubic foot (lb/ft³)
            PoundsPerCubicFoot {
                symbol: "lb/ft³",
                factor: LB_PER_CUFT_FACTOR,
                ctor: pounds_per_cubic_foot,
                to: to_pounds_per_cubic_foot,
                si: false
            },
            /// Pounds per gallon (lb/gal)
            PoundsPerGallon {
                symbol: "lb/gal",
                factor: LB_PER_GAL_FACTOR,
                ctor: pounds_per_gallon,
                to: to_pounds_per_gallon,
                si: false
            }
        }
    }
}
impl Density {
    /// Creates a Density from mass and volume.
    pub fn from_mass_and_volume(mass: Mass, volume: Volume) -> Self {
        let kg_per_m3 = mass.to_kilograms() / volume.to_cubic_meters();
        Self::new(kg_per_m3, DensityUnit::KilogramsPerCubicMeter)
    }
}

// Density * Volume = Mass
impl Mul<Volume> for Density {
    type Output = Mass;

    fn mul(self, rhs: Volume) -> Self::Output {
        let mass_kg = self.to_kilograms_per_cubic_meter() * rhs.to_cubic_meters();
        Mass::new(mass_kg, MassUnit::Kilograms)
    }
}

// Mass / Volume = Density
impl Div<Volume> for Mass {
    type Output = Density;

    fn div(self, rhs: Volume) -> Self::Output {
        Density::from_mass_and_volume(self, rhs)
    }
}

// Mass / Density = Volume
impl Div<Density> for Mass {
    type Output = Volume;

    fn div(self, rhs: Density) -> Self::Output {
        let volume_m3 = self.to_kilograms() / rhs.to_kilograms_per_cubic_meter();
        Volume::new(volume_m3, VolumeUnit::CubicMeters)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_density_creation() {
        let d = Density::kilograms_per_cubic_meter(1000.0);
        assert_eq!(d.value(), 1000.0);
        assert_eq!(d.unit(), DensityUnit::KilogramsPerCubicMeter);
    }

    #[test]
    fn test_density_conversions() {
        // 1 kg/L = 1000 kg/m³
        let d = Density::kilograms_per_liter(1.0);
        assert_eq!(d.to_kilograms_per_cubic_meter(), 1000.0);

        // 1 g/mL = 1 kg/L = 1000 kg/m³
        let d2 = Density::grams_per_milliliter(1.0);
        assert_eq!(d2.to_kilograms_per_liter(), 1.0);
    }

    #[test]
    fn test_density_times_volume() {
        // Water: 1000 kg/m³, 1 liter → 1 kg
        let density = Density::kilograms_per_cubic_meter(1000.0);
        let volume = Volume::liters(1.0);
        let mass = density * volume;
        assert!((mass.to_kilograms() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_mass_divided_by_volume() {
        let mass = Mass::kilograms(2.0);
        let volume = Volume::liters(1.0);
        let density = mass / volume;
        assert!((density.to_kilograms_per_cubic_meter() - 2000.0).abs() < 1e-10);
    }

    #[test]
    fn test_mass_divided_by_density() {
        let mass = Mass::kilograms(1.0);
        let density = Density::kilograms_per_cubic_meter(1000.0);
        let volume = mass / density;
        assert!((volume.to_liters() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_from_mass_and_volume() {
        let mass = Mass::grams(500.0);
        let volume = Volume::milliliters(250.0);
        let density = Density::from_mass_and_volume(mass, volume);
        assert!((density.to_grams_per_milliliter() - 2.0).abs() < 1e-10);
    }
}
