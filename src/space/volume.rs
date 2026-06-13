//! Volume quantity and units.

use crate::systems::metric::{CENTI, DECI, KILO, MILLI};

/// Conversion factors
const CUBIC_INCH_TO_CUBIC_METER: f64 = 1.6387064e-5;
const CUBIC_FOOT_TO_CUBIC_METER: f64 = 0.028316846592;
const CUBIC_YARD_TO_CUBIC_METER: f64 = 0.764554857984;
const US_GALLON_TO_CUBIC_METER: f64 = 0.003785411784;
const US_QUART_TO_CUBIC_METER: f64 = US_GALLON_TO_CUBIC_METER / 4.0;
const US_PINT_TO_CUBIC_METER: f64 = US_GALLON_TO_CUBIC_METER / 8.0;
const US_CUP_TO_CUBIC_METER: f64 = US_GALLON_TO_CUBIC_METER / 16.0;
const US_FL_OZ_TO_CUBIC_METER: f64 = US_GALLON_TO_CUBIC_METER / 128.0;
crate::quantity! {
    /// A quantity of volume.
    ///
    /// Volume represents a three-dimensional extent, the product of three lengths.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let v1 = Volume::liters(1000.0);
    /// let v2 = Volume::cubic_meters(1.0);
    ///
    /// // These represent the same volume
    /// assert!((v1.to_cubic_meters() - v2.to_cubic_meters()).abs() < 1e-10);
    /// ```
    pub quantity Volume {
        unit: VolumeUnit;
        dimension: VolumeDimension;
        conversions: VolumeConversions;
        name: "Volume";
        primary: CubicMeters;
        si: CubicMeters;

        units {
            /// Cubic millimeters (mm³)
            CubicMillimeters {
                symbol: "mm³",
                factor: MILLI * MILLI * MILLI,
                ctor: cubic_millimeters,
                to: to_cubic_millimeters,
                si: true
            },
            /// Cubic centimeters (cm³) - same as milliliters
            CubicCentimeters {
                symbol: "cm³",
                factor: CENTI * CENTI * CENTI,
                ctor: cubic_centimeters,
                to: to_cubic_centimeters,
                si: true
            },
            /// Cubic meters (m³) - SI derived unit
            CubicMeters {
                symbol: "m³",
                factor: 1.0,
                ctor: cubic_meters,
                to: to_cubic_meters,
                si: true
            },
            /// Cubic kilometers (km³)
            CubicKilometers {
                symbol: "km³",
                factor: KILO * KILO * KILO,
                ctor: cubic_kilometers,
                to: to_cubic_kilometers,
                si: true
            },
            /// Milliliters (mL)
            Milliliters {
                symbol: "mL",
                factor: CENTI * CENTI * CENTI,
                ctor: milliliters,
                to: to_milliliters,
                si: true
            },
            /// Liters (L)
            Liters {
                symbol: "L",
                factor: DECI * DECI * DECI,
                ctor: liters,
                to: to_liters,
                si: true
            },
            /// Cubic inches (in³)
            CubicInches {
                symbol: "in³",
                factor: CUBIC_INCH_TO_CUBIC_METER,
                ctor: cubic_inches,
                to: to_cubic_inches,
                si: false
            },
            /// Cubic feet (ft³)
            CubicFeet {
                symbol: "ft³",
                factor: CUBIC_FOOT_TO_CUBIC_METER,
                ctor: cubic_feet,
                to: to_cubic_feet,
                si: false
            },
            /// Cubic yards (yd³)
            CubicYards {
                symbol: "yd³",
                factor: CUBIC_YARD_TO_CUBIC_METER,
                ctor: cubic_yards,
                to: to_cubic_yards,
                si: false
            },
            /// US fluid ounces
            UsFluidOunces {
                symbol: "fl oz",
                factor: US_FL_OZ_TO_CUBIC_METER,
                ctor: us_fluid_ounces,
                to: to_us_fluid_ounces,
                si: false
            },
            /// US cups
            UsCups {
                symbol: "cup",
                factor: US_CUP_TO_CUBIC_METER,
                ctor: us_cups,
                to: to_us_cups,
                si: false
            },
            /// US pints
            UsPints {
                symbol: "pt",
                factor: US_PINT_TO_CUBIC_METER,
                ctor: us_pints,
                to: to_us_pints,
                si: false
            },
            /// US quarts
            UsQuarts {
                symbol: "qt",
                factor: US_QUART_TO_CUBIC_METER,
                ctor: us_quarts,
                to: to_us_quarts,
                si: false
            },
            /// US gallons
            UsGallons {
                symbol: "gal",
                factor: US_GALLON_TO_CUBIC_METER,
                ctor: us_gallons,
                to: to_us_gallons,
                si: false
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_volume_creation() {
        let v = Volume::liters(1.0);
        assert_eq!(v.value(), 1.0);
        assert_eq!(v.unit(), VolumeUnit::Liters);
    }

    #[test]
    fn test_volume_conversions() {
        let v = Volume::liters(1000.0);
        assert!((v.to_cubic_meters() - 1.0).abs() < 1e-10);

        let v2 = Volume::liters(1.0);
        assert!((v2.to_milliliters() - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_volume_arithmetic() {
        let v1 = Volume::liters(500.0);
        let v2 = Volume::liters(500.0);
        let sum = v1 + v2;
        assert_eq!(sum.to_liters(), 1000.0);
    }

    #[test]
    fn test_gallon_conversions() {
        let v = Volume::us_gallons(1.0);
        // 1 US gallon ≈ 3.785 liters
        assert!((v.to_liters() - 3.785).abs() < 0.01);
    }
}
