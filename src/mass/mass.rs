//! Mass quantity and units.

use crate::systems::metric::{KILO, MEGA, MICRO, MILLI, NANO};

// Conversion factors to grams (primary unit)
const KILOGRAM_TO_GRAM: f64 = KILO; // 1000
const POUND_TO_GRAM: f64 = KILOGRAM_TO_GRAM * 0.45359237; // ~453.59 g
const OUNCE_TO_GRAM: f64 = POUND_TO_GRAM / 16.0; // ~28.35 g
const TROY_GRAIN_TO_GRAM: f64 = 0.06479891; // ~64.8 mg
const DALTON_TO_GRAM: f64 = 1.66053906660e-24; // atomic mass unit
crate::quantity! {
    /// A quantity of mass.
    ///
    /// Mass represents the amount of matter in an object.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let m1 = Mass::kilograms(1.0);
    /// let m2 = Mass::pounds(2.20462);
    ///
    /// // These represent approximately the same mass
    /// assert!((m1.to_grams() - m2.to_grams()).abs() < 0.01);
    /// ```
    pub quantity Mass {
        unit: MassUnit;
        dimension: MassDimension;
        conversions: MassConversions;
        name: "Mass";
        primary: Grams;
        si: Kilograms;

        units {
            /// Nanograms (ng)
            Nanograms {
                symbol: "ng",
                factor: NANO,
                ctor: nanograms,
                to: to_nanograms,
                si: true
            },
            /// Micrograms (mcg)
            Micrograms {
                symbol: "mcg",
                factor: MICRO,
                ctor: micrograms,
                to: to_micrograms,
                si: true
            },
            /// Milligrams (mg)
            Milligrams {
                symbol: "mg",
                factor: MILLI,
                ctor: milligrams,
                to: to_milligrams,
                si: true
            },
            /// Grams (g) - primary unit
            Grams {
                symbol: "g",
                factor: 1.0,
                ctor: grams,
                to: to_grams,
                si: true
            },
            /// Kilograms (kg) - SI base unit
            Kilograms {
                symbol: "kg",
                factor: KILO,
                ctor: kilograms,
                to: to_kilograms,
                si: true
            },
            /// Tonnes (t) - metric ton = 1000 kg
            Tonnes {
                symbol: "t",
                factor: MEGA,
                ctor: tonnes,
                to: to_tonnes,
                si: true
            },
            /// Ounces (oz)
            Ounces {
                symbol: "oz",
                factor: OUNCE_TO_GRAM,
                ctor: ounces,
                to: to_ounces,
                si: false
            },
            /// Pounds (lb)
            Pounds {
                symbol: "lb",
                factor: POUND_TO_GRAM,
                ctor: pounds,
                to: to_pounds,
                si: false
            },
            /// Kilopounds (klb)
            Kilopounds {
                symbol: "klb",
                factor: POUND_TO_GRAM * KILO,
                ctor: kilopounds,
                to: to_kilopounds,
                si: false
            },
            /// Megapounds (Mlb)
            Megapounds {
                symbol: "Mlb",
                factor: POUND_TO_GRAM * MEGA,
                ctor: megapounds,
                to: to_megapounds,
                si: false
            },
            /// Stone (st) = 14 pounds
            Stone {
                symbol: "st",
                factor: POUND_TO_GRAM * 14.0,
                ctor: stone,
                to: to_stone,
                si: false
            },
            /// Troy grains (gr)
            TroyGrains {
                symbol: "gr",
                factor: TROY_GRAIN_TO_GRAM,
                ctor: troy_grains,
                to: to_troy_grains,
                si: false
            },
            /// Pennyweights (dwt) = 24 troy grains
            Pennyweights {
                symbol: "dwt",
                factor: TROY_GRAIN_TO_GRAM * 24.0,
                ctor: pennyweights,
                to: to_pennyweights,
                si: false
            },
            /// Troy ounces (oz t) = 480 troy grains
            TroyOunces {
                symbol: "oz t",
                factor: TROY_GRAIN_TO_GRAM * 480.0,
                ctor: troy_ounces,
                to: to_troy_ounces,
                si: false
            },
            /// Troy pounds (lb t) = 12 troy ounces
            TroyPounds {
                symbol: "lb t",
                factor: TROY_GRAIN_TO_GRAM * 480.0 * 12.0,
                ctor: troy_pounds,
                to: to_troy_pounds,
                si: false
            },
            /// Tolas - South Asian unit
            Tolas {
                symbol: "tola",
                factor: TROY_GRAIN_TO_GRAM * 180.0,
                ctor: tolas,
                to: to_tolas,
                si: false
            },
            /// Carats (ct) - for gemstones
            Carats {
                symbol: "ct",
                factor: MILLI * 200.0,
                ctor: carats,
                to: to_carats,
                si: false
            },
            /// Solar masses (M☉) - astronomical
            SolarMasses {
                symbol: "M☉",
                factor: 1.98855e33,
                ctor: solar_masses,
                to: to_solar_masses,
                si: false
            },
            /// Daltons (Da) - atomic mass unit
            Dalton {
                symbol: "Da",
                factor: DALTON_TO_GRAM,
                ctor: daltons,
                to: to_daltons,
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
    fn test_mass_creation() {
        let m = Mass::kilograms(1.0);
        assert_eq!(m.value(), 1.0);
        assert_eq!(m.unit(), MassUnit::Kilograms);
    }

    #[test]
    fn test_mass_conversions() {
        let m = Mass::kilograms(1.0);
        assert_eq!(m.to_grams(), 1000.0);

        let m2 = Mass::grams(1000.0);
        assert_eq!(m2.to_kilograms(), 1.0);
    }

    #[test]
    fn test_mass_arithmetic() {
        let m1 = Mass::grams(500.0);
        let m2 = Mass::grams(500.0);
        let sum = m1 + m2;
        assert_eq!(sum.to_grams(), 1000.0);
    }

    #[test]
    fn test_pound_conversions() {
        let m = Mass::pounds(1.0);
        // 1 pound ≈ 453.59 grams
        assert!((m.to_grams() - 453.59237).abs() < 0.001);
    }

    #[test]
    fn test_tonne_conversions() {
        let m = Mass::tonnes(1.0);
        assert_eq!(m.to_kilograms(), 1000.0);
    }

    #[test]
    fn test_ounce_conversions() {
        let m = Mass::ounces(16.0);
        // 16 ounces = 1 pound
        assert!((m.to_pounds() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_carat_conversions() {
        let m = Mass::carats(5.0);
        // 1 carat = 200 mg, so 5 carats = 1000 mg = 1 gram
        assert_eq!(m.to_grams(), 1.0);
    }
}
