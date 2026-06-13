//! Length quantity and units.

use super::area::{Area, AreaUnit};
use super::volume::{Volume, VolumeUnit};
use crate::core::Quantity;
use crate::systems::metric::{CENTI, DECI, HECTO, KILO, MICRO, MILLI, NANO, PICO};
use std::ops::Mul;

/// Conversion factors for imperial units.
const FEET_TO_METERS: f64 = 0.3048;
const YARDS_TO_METERS: f64 = 0.9144;
const MILES_TO_METERS: f64 = 1609.344;
const NAUTICAL_MILES_TO_METERS: f64 = 1852.0;
const AU_TO_METERS: f64 = 1.495978707e11;
const LIGHT_YEAR_TO_METERS: f64 = 9.4607304725808e15;
const PARSEC_TO_METERS: f64 = 3.08567758149137e16;
crate::quantity! {
    /// A quantity of length.
    ///
    /// Length is one of the seven SI base quantities, with the meter as its SI base unit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let l1 = Length::meters(1000.0);
    /// let l2 = Length::kilometers(1.0);
    ///
    /// // These represent the same length
    /// assert!((l1.to(LengthUnit::Meters) - l2.to(LengthUnit::Meters)).abs() < 1e-10);
    /// ```
    pub quantity Length {
        unit: LengthUnit;
        dimension: LengthDimension;
        conversions: LengthConversions;
        name: "Length";
        primary: Meters;
        si: Meters;

        units {
            /// Angstroms (Å) - 10^-10 meters
            Angstroms {
                symbol: "Å",
                factor: 100.0 * PICO,
                ctor: angstroms,
                to: to_angstroms,
                si: false
            },
            /// Nanometers (nm) - 10^-9 meters
            Nanometers {
                symbol: "nm",
                factor: NANO,
                ctor: nanometers,
                to: to_nanometers,
                si: true
            },
            /// Micrometers/Microns (µm) - 10^-6 meters
            Micrometers {
                symbol: "µm",
                factor: MICRO,
                ctor: micrometers,
                to: to_micrometers,
                si: true
            },
            /// Millimeters (mm) - 10^-3 meters
            Millimeters {
                symbol: "mm",
                factor: MILLI,
                ctor: millimeters,
                to: to_millimeters,
                si: true
            },
            /// Centimeters (cm) - 10^-2 meters
            Centimeters {
                symbol: "cm",
                factor: CENTI,
                ctor: centimeters,
                to: to_centimeters,
                si: true
            },
            /// Decimeters (dm) - 10^-1 meters
            Decimeters {
                symbol: "dm",
                factor: DECI,
                ctor: decimeters,
                to: to_decimeters,
                si: true
            },
            /// Meters (m) - SI base unit
            Meters {
                symbol: "m",
                factor: 1.0,
                ctor: meters,
                to: to_meters,
                si: true
            },
            /// Hectometers (hm) - 10^2 meters
            Hectometers {
                symbol: "hm",
                factor: HECTO,
                ctor: hectometers,
                to: to_hectometers,
                si: true
            },
            /// Kilometers (km) - 10^3 meters
            Kilometers {
                symbol: "km",
                factor: KILO,
                ctor: kilometers,
                to: to_kilometers,
                si: true
            },
            /// Inches (in) - 0.0254 meters
            Inches {
                symbol: "in",
                factor: FEET_TO_METERS / 12.0,
                ctor: inches,
                to: to_inches,
                si: false
            },
            /// Feet (ft) - 0.3048 meters
            Feet {
                symbol: "ft",
                factor: FEET_TO_METERS,
                ctor: feet,
                to: to_feet,
                si: false
            },
            /// Yards (yd) - 0.9144 meters
            Yards {
                symbol: "yd",
                factor: YARDS_TO_METERS,
                ctor: yards,
                to: to_yards,
                si: false
            },
            /// Miles (mi) - 1609.344 meters
            Miles {
                symbol: "mi",
                factor: MILES_TO_METERS,
                ctor: miles,
                to: to_miles,
                si: false
            },
            /// Nautical miles (nmi) - 1852 meters
            NauticalMiles {
                symbol: "nmi",
                factor: NAUTICAL_MILES_TO_METERS,
                ctor: nautical_miles,
                to: to_nautical_miles,
                si: false
            },
            /// Astronomical units (au)
            AstronomicalUnits {
                symbol: "au",
                factor: AU_TO_METERS,
                ctor: astronomical_units,
                to: to_astronomical_units,
                si: false
            },
            /// Light years (ly)
            LightYears {
                symbol: "ly",
                factor: LIGHT_YEAR_TO_METERS,
                ctor: light_years,
                to: to_light_years,
                si: false
            },
            /// Parsecs (pc)
            Parsecs {
                symbol: "pc",
                factor: PARSEC_TO_METERS,
                ctor: parsecs,
                to: to_parsecs,
                si: false
            }
        }
    }
}
impl Length {
    /// Returns the squared length (this * this).
    pub fn squared(&self) -> Area {
        *self * *self
    }

    /// Returns the cubed length (this * this * this).
    pub fn cubed(&self) -> Volume {
        self.squared() * *self
    }
}
// Length * Length = Area
impl Mul<Length> for Length {
    type Output = Area;

    fn mul(self, rhs: Length) -> Self::Output {
        let area_meters_sq = self.to_meters() * rhs.to_meters();
        Area::new(area_meters_sq, AreaUnit::SquareMeters)
    }
}

// Length * Area = Volume
impl Mul<Area> for Length {
    type Output = Volume;

    fn mul(self, rhs: Area) -> Self::Output {
        let volume_meters_cu = self.to_meters() * rhs.to_square_meters();
        Volume::new(volume_meters_cu, VolumeUnit::CubicMeters)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_length_creation() {
        let l = Length::meters(100.0);
        assert_eq!(l.value(), 100.0);
        assert_eq!(l.unit(), LengthUnit::Meters);
    }

    #[test]
    fn test_length_conversions() {
        let l = Length::meters(1000.0);
        assert_eq!(l.to_kilometers(), 1.0);
        assert_eq!(l.to_centimeters(), 100_000.0);
    }

    #[test]
    fn test_imperial_conversions() {
        let l = Length::feet(1.0);
        assert!((l.to_inches() - 12.0).abs() < 1e-10);

        let l2 = Length::miles(1.0);
        assert!((l2.to_feet() - 5280.0).abs() < 0.1);
    }

    #[test]
    fn test_length_arithmetic() {
        let l1 = Length::meters(50.0);
        let l2 = Length::meters(50.0);
        let sum = l1 + l2;
        assert_eq!(sum.to_meters(), 100.0);
    }

    #[test]
    fn test_length_multiplication_to_area() {
        let width = Length::meters(10.0);
        let height = Length::meters(5.0);
        let area = width * height;
        assert_eq!(area.to_square_meters(), 50.0);
    }

    #[test]
    fn test_length_squared() {
        let l = Length::meters(5.0);
        let area = l.squared();
        assert_eq!(area.to_square_meters(), 25.0);
    }

    #[test]
    fn test_length_cubed() {
        let l = Length::meters(2.0);
        let volume = l.cubed();
        assert_eq!(volume.to_cubic_meters(), 8.0);
    }

    #[test]
    fn test_length_comparison() {
        let l1 = Length::kilometers(1.0);
        let l2 = Length::meters(1000.0);
        assert_eq!(l1, l2);
    }

    #[test]
    fn test_length_dsl() {
        let l = 100.0.meters();
        assert_eq!(l.to_meters(), 100.0);
    }

    #[test]
    fn test_astronomical_units() {
        let au = Length::astronomical_units(1.0);
        // 1 AU is approximately 149.6 million km
        assert!((au.to_kilometers() / 1e8 - 1.496).abs() < 0.001);
    }
}
