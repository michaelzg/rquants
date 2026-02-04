//! Mass quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use crate::systems::metric::{KILO, MEGA, MICRO, MILLI, NANO};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of mass measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MassUnit {
    // SI metric units
    /// Nanograms (ng)
    Nanograms,
    /// Micrograms (mcg)
    Micrograms,
    /// Milligrams (mg)
    Milligrams,
    /// Grams (g) - primary unit
    Grams,
    /// Kilograms (kg) - SI base unit
    Kilograms,
    /// Tonnes (t) - metric ton = 1000 kg
    Tonnes,

    // Imperial/US units
    /// Ounces (oz)
    Ounces,
    /// Pounds (lb)
    Pounds,
    /// Kilopounds (klb)
    Kilopounds,
    /// Megapounds (Mlb)
    Megapounds,
    /// Stone (st) = 14 pounds
    Stone,

    // Troy weights
    /// Troy grains (gr)
    TroyGrains,
    /// Pennyweights (dwt) = 24 troy grains
    Pennyweights,
    /// Troy ounces (oz t) = 480 troy grains
    TroyOunces,
    /// Troy pounds (lb t) = 12 troy ounces
    TroyPounds,

    // Other units
    /// Tolas - South Asian unit
    Tolas,
    /// Carats (ct) - for gemstones
    Carats,
    /// Solar masses (M☉) - astronomical
    SolarMasses,
    /// Daltons (Da) - atomic mass unit
    Dalton,
}

impl MassUnit {
    /// All available mass units.
    pub const ALL: &'static [MassUnit] = &[
        MassUnit::Nanograms,
        MassUnit::Micrograms,
        MassUnit::Milligrams,
        MassUnit::Grams,
        MassUnit::Kilograms,
        MassUnit::Tonnes,
        MassUnit::Ounces,
        MassUnit::Pounds,
        MassUnit::Kilopounds,
        MassUnit::Megapounds,
        MassUnit::Stone,
        MassUnit::TroyGrains,
        MassUnit::Pennyweights,
        MassUnit::TroyOunces,
        MassUnit::TroyPounds,
        MassUnit::Tolas,
        MassUnit::Carats,
        MassUnit::SolarMasses,
        MassUnit::Dalton,
    ];
}

// Conversion factors to grams (primary unit)
const KILOGRAM_TO_GRAM: f64 = KILO; // 1000
const POUND_TO_GRAM: f64 = KILOGRAM_TO_GRAM * 0.45359237; // ~453.59 g
const OUNCE_TO_GRAM: f64 = POUND_TO_GRAM / 16.0; // ~28.35 g
const TROY_GRAIN_TO_GRAM: f64 = 0.06479891; // ~64.8 mg
const DALTON_TO_GRAM: f64 = 1.66053906660e-24; // atomic mass unit

impl fmt::Display for MassUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for MassUnit {
    fn symbol(&self) -> &'static str {
        match self {
            MassUnit::Nanograms => "ng",
            MassUnit::Micrograms => "mcg",
            MassUnit::Milligrams => "mg",
            MassUnit::Grams => "g",
            MassUnit::Kilograms => "kg",
            MassUnit::Tonnes => "t",
            MassUnit::Ounces => "oz",
            MassUnit::Pounds => "lb",
            MassUnit::Kilopounds => "klb",
            MassUnit::Megapounds => "Mlb",
            MassUnit::Stone => "st",
            MassUnit::TroyGrains => "gr",
            MassUnit::Pennyweights => "dwt",
            MassUnit::TroyOunces => "oz t",
            MassUnit::TroyPounds => "lb t",
            MassUnit::Tolas => "tola",
            MassUnit::Carats => "ct",
            MassUnit::SolarMasses => "M☉",
            MassUnit::Dalton => "Da",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            // SI metric (relative to grams)
            MassUnit::Nanograms => NANO,
            MassUnit::Micrograms => MICRO,
            MassUnit::Milligrams => MILLI,
            MassUnit::Grams => 1.0,
            MassUnit::Kilograms => KILO,
            MassUnit::Tonnes => MEGA,

            // Imperial
            MassUnit::Ounces => OUNCE_TO_GRAM,
            MassUnit::Pounds => POUND_TO_GRAM,
            MassUnit::Kilopounds => POUND_TO_GRAM * KILO,
            MassUnit::Megapounds => POUND_TO_GRAM * MEGA,
            MassUnit::Stone => POUND_TO_GRAM * 14.0,

            // Troy weights
            MassUnit::TroyGrains => TROY_GRAIN_TO_GRAM,
            MassUnit::Pennyweights => TROY_GRAIN_TO_GRAM * 24.0,
            MassUnit::TroyOunces => TROY_GRAIN_TO_GRAM * 480.0,
            MassUnit::TroyPounds => TROY_GRAIN_TO_GRAM * 480.0 * 12.0,

            // Other
            MassUnit::Tolas => TROY_GRAIN_TO_GRAM * 180.0,
            MassUnit::Carats => MILLI * 200.0, // 200 mg = 0.2 g
            MassUnit::SolarMasses => 1.98855e33,
            MassUnit::Dalton => DALTON_TO_GRAM,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            MassUnit::Nanograms
                | MassUnit::Micrograms
                | MassUnit::Milligrams
                | MassUnit::Grams
                | MassUnit::Kilograms
                | MassUnit::Tonnes
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct Mass {
    value: f64,
    unit: MassUnit,
}

impl Mass {
    /// Creates a new Mass quantity.
    pub const fn new_const(value: f64, unit: MassUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a Mass in nanograms.
    pub fn nanograms(value: f64) -> Self {
        Self::new(value, MassUnit::Nanograms)
    }

    /// Creates a Mass in micrograms.
    pub fn micrograms(value: f64) -> Self {
        Self::new(value, MassUnit::Micrograms)
    }

    /// Creates a Mass in milligrams.
    pub fn milligrams(value: f64) -> Self {
        Self::new(value, MassUnit::Milligrams)
    }

    /// Creates a Mass in grams.
    pub fn grams(value: f64) -> Self {
        Self::new(value, MassUnit::Grams)
    }

    /// Creates a Mass in kilograms.
    pub fn kilograms(value: f64) -> Self {
        Self::new(value, MassUnit::Kilograms)
    }

    /// Creates a Mass in tonnes.
    pub fn tonnes(value: f64) -> Self {
        Self::new(value, MassUnit::Tonnes)
    }

    /// Creates a Mass in ounces.
    pub fn ounces(value: f64) -> Self {
        Self::new(value, MassUnit::Ounces)
    }

    /// Creates a Mass in pounds.
    pub fn pounds(value: f64) -> Self {
        Self::new(value, MassUnit::Pounds)
    }

    /// Creates a Mass in stone.
    pub fn stone(value: f64) -> Self {
        Self::new(value, MassUnit::Stone)
    }

    /// Creates a Mass in troy grains.
    pub fn troy_grains(value: f64) -> Self {
        Self::new(value, MassUnit::TroyGrains)
    }

    /// Creates a Mass in troy ounces.
    pub fn troy_ounces(value: f64) -> Self {
        Self::new(value, MassUnit::TroyOunces)
    }

    /// Creates a Mass in carats.
    pub fn carats(value: f64) -> Self {
        Self::new(value, MassUnit::Carats)
    }

    /// Creates a Mass in daltons.
    pub fn daltons(value: f64) -> Self {
        Self::new(value, MassUnit::Dalton)
    }

    // Conversion methods
    /// Converts to nanograms.
    pub fn to_nanograms(&self) -> f64 {
        self.to(MassUnit::Nanograms)
    }

    /// Converts to micrograms.
    pub fn to_micrograms(&self) -> f64 {
        self.to(MassUnit::Micrograms)
    }

    /// Converts to milligrams.
    pub fn to_milligrams(&self) -> f64 {
        self.to(MassUnit::Milligrams)
    }

    /// Converts to grams.
    pub fn to_grams(&self) -> f64 {
        self.to(MassUnit::Grams)
    }

    /// Converts to kilograms.
    pub fn to_kilograms(&self) -> f64 {
        self.to(MassUnit::Kilograms)
    }

    /// Converts to tonnes.
    pub fn to_tonnes(&self) -> f64 {
        self.to(MassUnit::Tonnes)
    }

    /// Converts to ounces.
    pub fn to_ounces(&self) -> f64 {
        self.to(MassUnit::Ounces)
    }

    /// Converts to pounds.
    pub fn to_pounds(&self) -> f64 {
        self.to(MassUnit::Pounds)
    }

    /// Converts to stone.
    pub fn to_stone(&self) -> f64 {
        self.to(MassUnit::Stone)
    }

    /// Converts to carats.
    pub fn to_carats(&self) -> f64 {
        self.to(MassUnit::Carats)
    }

    /// Creates a Mass in kilopounds.
    pub fn kilopounds(value: f64) -> Self {
        Self::new(value, MassUnit::Kilopounds)
    }

    /// Creates a Mass in megapounds.
    pub fn megapounds(value: f64) -> Self {
        Self::new(value, MassUnit::Megapounds)
    }

    /// Creates a Mass in pennyweights.
    pub fn pennyweights(value: f64) -> Self {
        Self::new(value, MassUnit::Pennyweights)
    }

    /// Creates a Mass in troy pounds.
    pub fn troy_pounds(value: f64) -> Self {
        Self::new(value, MassUnit::TroyPounds)
    }

    /// Creates a Mass in tolas.
    pub fn tolas(value: f64) -> Self {
        Self::new(value, MassUnit::Tolas)
    }

    /// Creates a Mass in solar masses.
    pub fn solar_masses(value: f64) -> Self {
        Self::new(value, MassUnit::SolarMasses)
    }

    /// Converts to kilopounds.
    pub fn to_kilopounds(&self) -> f64 {
        self.to(MassUnit::Kilopounds)
    }

    /// Converts to megapounds.
    pub fn to_megapounds(&self) -> f64 {
        self.to(MassUnit::Megapounds)
    }

    /// Converts to troy grains.
    pub fn to_troy_grains(&self) -> f64 {
        self.to(MassUnit::TroyGrains)
    }

    /// Converts to pennyweights.
    pub fn to_pennyweights(&self) -> f64 {
        self.to(MassUnit::Pennyweights)
    }

    /// Converts to troy ounces.
    pub fn to_troy_ounces(&self) -> f64 {
        self.to(MassUnit::TroyOunces)
    }

    /// Converts to troy pounds.
    pub fn to_troy_pounds(&self) -> f64 {
        self.to(MassUnit::TroyPounds)
    }

    /// Converts to tolas.
    pub fn to_tolas(&self) -> f64 {
        self.to(MassUnit::Tolas)
    }

    /// Converts to daltons.
    pub fn to_daltons(&self) -> f64 {
        self.to(MassUnit::Dalton)
    }

    /// Converts to solar masses.
    pub fn to_solar_masses(&self) -> f64 {
        self.to(MassUnit::SolarMasses)
    }
}

impl fmt::Display for Mass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Mass {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Mass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Mass {
    type Unit = MassUnit;

    fn new(value: f64, unit: Self::Unit) -> Self {
        Self { value, unit }
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> Self::Unit {
        self.unit
    }
}

// Arithmetic operations

impl Add for Mass {
    type Output = Mass;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Mass::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Mass {
    type Output = Mass;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Mass::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Mass {
    type Output = Mass;

    fn mul(self, rhs: f64) -> Self::Output {
        Mass::new(self.value * rhs, self.unit)
    }
}

impl Mul<Mass> for f64 {
    type Output = Mass;

    fn mul(self, rhs: Mass) -> Self::Output {
        Mass::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for Mass {
    type Output = Mass;

    fn div(self, rhs: f64) -> Self::Output {
        Mass::new(self.value / rhs, self.unit)
    }
}

impl Div<Mass> for Mass {
    type Output = f64;

    fn div(self, rhs: Mass) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Mass {
    type Output = Mass;

    fn neg(self) -> Self::Output {
        Mass::new(-self.value, self.unit)
    }
}

/// Dimension for Mass.
pub struct MassDimension;

impl Dimension for MassDimension {
    type Quantity = Mass;
    type Unit = MassUnit;

    fn name() -> &'static str {
        "Mass"
    }

    fn primary_unit() -> Self::Unit {
        MassUnit::Grams
    }

    fn si_unit() -> Self::Unit {
        MassUnit::Kilograms
    }

    fn units() -> &'static [Self::Unit] {
        MassUnit::ALL
    }
}

/// Extension trait for creating Mass quantities from numeric types.
pub trait MassConversions {
    /// Creates a Mass in nanograms.
    fn nanograms(self) -> Mass;
    /// Creates a Mass in micrograms.
    fn micrograms(self) -> Mass;
    /// Creates a Mass in milligrams.
    fn milligrams(self) -> Mass;
    /// Creates a Mass in grams.
    fn grams(self) -> Mass;
    /// Creates a Mass in kilograms.
    fn kilograms(self) -> Mass;
    /// Creates a Mass in tonnes.
    fn tonnes(self) -> Mass;
    /// Creates a Mass in ounces.
    fn ounces(self) -> Mass;
    /// Creates a Mass in pounds.
    fn pounds(self) -> Mass;
    /// Creates a Mass in kilopounds.
    fn kilopounds(self) -> Mass;
    /// Creates a Mass in megapounds.
    fn megapounds(self) -> Mass;
    /// Creates a Mass in stone.
    fn stone(self) -> Mass;
    /// Creates a Mass in troy grains.
    fn troy_grains(self) -> Mass;
    /// Creates a Mass in pennyweights.
    fn pennyweights(self) -> Mass;
    /// Creates a Mass in troy ounces.
    fn troy_ounces(self) -> Mass;
    /// Creates a Mass in troy pounds.
    fn troy_pounds(self) -> Mass;
    /// Creates a Mass in tolas.
    fn tolas(self) -> Mass;
    /// Creates a Mass in carats.
    fn carats(self) -> Mass;
    /// Creates a Mass in solar masses.
    fn solar_masses(self) -> Mass;
    /// Creates a Mass in daltons.
    fn daltons(self) -> Mass;
}

impl MassConversions for f64 {
    fn nanograms(self) -> Mass {
        Mass::nanograms(self)
    }
    fn micrograms(self) -> Mass {
        Mass::micrograms(self)
    }
    fn milligrams(self) -> Mass {
        Mass::milligrams(self)
    }
    fn grams(self) -> Mass {
        Mass::grams(self)
    }
    fn kilograms(self) -> Mass {
        Mass::kilograms(self)
    }
    fn tonnes(self) -> Mass {
        Mass::tonnes(self)
    }
    fn ounces(self) -> Mass {
        Mass::ounces(self)
    }
    fn pounds(self) -> Mass {
        Mass::pounds(self)
    }
    fn kilopounds(self) -> Mass {
        Mass::kilopounds(self)
    }
    fn megapounds(self) -> Mass {
        Mass::megapounds(self)
    }
    fn stone(self) -> Mass {
        Mass::stone(self)
    }
    fn troy_grains(self) -> Mass {
        Mass::troy_grains(self)
    }
    fn pennyweights(self) -> Mass {
        Mass::pennyweights(self)
    }
    fn troy_ounces(self) -> Mass {
        Mass::troy_ounces(self)
    }
    fn troy_pounds(self) -> Mass {
        Mass::troy_pounds(self)
    }
    fn tolas(self) -> Mass {
        Mass::tolas(self)
    }
    fn carats(self) -> Mass {
        Mass::carats(self)
    }
    fn solar_masses(self) -> Mass {
        Mass::solar_masses(self)
    }
    fn daltons(self) -> Mass {
        Mass::daltons(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
