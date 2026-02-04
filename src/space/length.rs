//! Length quantity and units.

use super::area::{Area, AreaUnit};
use super::volume::{Volume, VolumeUnit};
use crate::core::{Dimension, Quantity, UnitOfMeasure};
use crate::systems::metric::{CENTI, DECI, HECTO, KILO, MICRO, MILLI, NANO, PICO};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of length measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LengthUnit {
    // SI metric units
    /// Angstroms (Å) - 10^-10 meters
    Angstroms,
    /// Nanometers (nm) - 10^-9 meters
    Nanometers,
    /// Micrometers/Microns (µm) - 10^-6 meters
    Micrometers,
    /// Millimeters (mm) - 10^-3 meters
    Millimeters,
    /// Centimeters (cm) - 10^-2 meters
    Centimeters,
    /// Decimeters (dm) - 10^-1 meters
    Decimeters,
    /// Meters (m) - SI base unit
    Meters,
    /// Hectometers (hm) - 10^2 meters
    Hectometers,
    /// Kilometers (km) - 10^3 meters
    Kilometers,

    // Imperial/US units
    /// Inches (in) - 0.0254 meters
    Inches,
    /// Feet (ft) - 0.3048 meters
    Feet,
    /// Yards (yd) - 0.9144 meters
    Yards,
    /// Miles (mi) - 1609.344 meters
    Miles,

    // Other units
    /// Nautical miles (nmi) - 1852 meters
    NauticalMiles,
    /// Astronomical units (au)
    AstronomicalUnits,
    /// Light years (ly)
    LightYears,
    /// Parsecs (pc)
    Parsecs,
}

impl LengthUnit {
    /// All available length units.
    pub const ALL: &'static [LengthUnit] = &[
        LengthUnit::Angstroms,
        LengthUnit::Nanometers,
        LengthUnit::Micrometers,
        LengthUnit::Millimeters,
        LengthUnit::Centimeters,
        LengthUnit::Decimeters,
        LengthUnit::Meters,
        LengthUnit::Hectometers,
        LengthUnit::Kilometers,
        LengthUnit::Inches,
        LengthUnit::Feet,
        LengthUnit::Yards,
        LengthUnit::Miles,
        LengthUnit::NauticalMiles,
        LengthUnit::AstronomicalUnits,
        LengthUnit::LightYears,
        LengthUnit::Parsecs,
    ];
}

/// Conversion factors for imperial units.
const FEET_TO_METERS: f64 = 0.3048;
const YARDS_TO_METERS: f64 = 0.9144;
const MILES_TO_METERS: f64 = 1609.344;
const NAUTICAL_MILES_TO_METERS: f64 = 1852.0;
const AU_TO_METERS: f64 = 1.495978707e11;
const LIGHT_YEAR_TO_METERS: f64 = 9.4607304725808e15;
const PARSEC_TO_METERS: f64 = 3.08567758149137e16;

impl fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for LengthUnit {
    fn symbol(&self) -> &'static str {
        match self {
            LengthUnit::Angstroms => "Å",
            LengthUnit::Nanometers => "nm",
            LengthUnit::Micrometers => "µm",
            LengthUnit::Millimeters => "mm",
            LengthUnit::Centimeters => "cm",
            LengthUnit::Decimeters => "dm",
            LengthUnit::Meters => "m",
            LengthUnit::Hectometers => "hm",
            LengthUnit::Kilometers => "km",
            LengthUnit::Inches => "in",
            LengthUnit::Feet => "ft",
            LengthUnit::Yards => "yd",
            LengthUnit::Miles => "mi",
            LengthUnit::NauticalMiles => "nmi",
            LengthUnit::AstronomicalUnits => "au",
            LengthUnit::LightYears => "ly",
            LengthUnit::Parsecs => "pc",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            LengthUnit::Angstroms => 100.0 * PICO,
            LengthUnit::Nanometers => NANO,
            LengthUnit::Micrometers => MICRO,
            LengthUnit::Millimeters => MILLI,
            LengthUnit::Centimeters => CENTI,
            LengthUnit::Decimeters => DECI,
            LengthUnit::Meters => 1.0,
            LengthUnit::Hectometers => HECTO,
            LengthUnit::Kilometers => KILO,
            LengthUnit::Inches => FEET_TO_METERS / 12.0,
            LengthUnit::Feet => FEET_TO_METERS,
            LengthUnit::Yards => YARDS_TO_METERS,
            LengthUnit::Miles => MILES_TO_METERS,
            LengthUnit::NauticalMiles => NAUTICAL_MILES_TO_METERS,
            LengthUnit::AstronomicalUnits => AU_TO_METERS,
            LengthUnit::LightYears => LIGHT_YEAR_TO_METERS,
            LengthUnit::Parsecs => PARSEC_TO_METERS,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            LengthUnit::Nanometers
                | LengthUnit::Micrometers
                | LengthUnit::Millimeters
                | LengthUnit::Centimeters
                | LengthUnit::Decimeters
                | LengthUnit::Meters
                | LengthUnit::Hectometers
                | LengthUnit::Kilometers
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct Length {
    value: f64,
    unit: LengthUnit,
}

impl Length {
    /// Creates a new Length quantity.
    pub const fn new_const(value: f64, unit: LengthUnit) -> Self {
        Self { value, unit }
    }

    // SI metric constructors
    /// Creates a Length in angstroms.
    pub fn angstroms(value: f64) -> Self {
        Self::new(value, LengthUnit::Angstroms)
    }

    /// Creates a Length in nanometers.
    pub fn nanometers(value: f64) -> Self {
        Self::new(value, LengthUnit::Nanometers)
    }

    /// Creates a Length in micrometers (microns).
    pub fn micrometers(value: f64) -> Self {
        Self::new(value, LengthUnit::Micrometers)
    }

    /// Creates a Length in millimeters.
    pub fn millimeters(value: f64) -> Self {
        Self::new(value, LengthUnit::Millimeters)
    }

    /// Creates a Length in centimeters.
    pub fn centimeters(value: f64) -> Self {
        Self::new(value, LengthUnit::Centimeters)
    }

    /// Creates a Length in decimeters.
    pub fn decimeters(value: f64) -> Self {
        Self::new(value, LengthUnit::Decimeters)
    }

    /// Creates a Length in meters.
    pub fn meters(value: f64) -> Self {
        Self::new(value, LengthUnit::Meters)
    }

    /// Creates a Length in hectometers.
    pub fn hectometers(value: f64) -> Self {
        Self::new(value, LengthUnit::Hectometers)
    }

    /// Creates a Length in kilometers.
    pub fn kilometers(value: f64) -> Self {
        Self::new(value, LengthUnit::Kilometers)
    }

    // Imperial constructors
    /// Creates a Length in inches.
    pub fn inches(value: f64) -> Self {
        Self::new(value, LengthUnit::Inches)
    }

    /// Creates a Length in feet.
    pub fn feet(value: f64) -> Self {
        Self::new(value, LengthUnit::Feet)
    }

    /// Creates a Length in yards.
    pub fn yards(value: f64) -> Self {
        Self::new(value, LengthUnit::Yards)
    }

    /// Creates a Length in miles.
    pub fn miles(value: f64) -> Self {
        Self::new(value, LengthUnit::Miles)
    }

    /// Creates a Length in nautical miles.
    pub fn nautical_miles(value: f64) -> Self {
        Self::new(value, LengthUnit::NauticalMiles)
    }

    /// Creates a Length in astronomical units.
    pub fn astronomical_units(value: f64) -> Self {
        Self::new(value, LengthUnit::AstronomicalUnits)
    }

    /// Creates a Length in light years.
    pub fn light_years(value: f64) -> Self {
        Self::new(value, LengthUnit::LightYears)
    }

    /// Creates a Length in parsecs.
    pub fn parsecs(value: f64) -> Self {
        Self::new(value, LengthUnit::Parsecs)
    }

    // Conversion methods
    /// Converts to meters.
    pub fn to_meters(&self) -> f64 {
        self.to(LengthUnit::Meters)
    }

    /// Converts to kilometers.
    pub fn to_kilometers(&self) -> f64 {
        self.to(LengthUnit::Kilometers)
    }

    /// Converts to centimeters.
    pub fn to_centimeters(&self) -> f64 {
        self.to(LengthUnit::Centimeters)
    }

    /// Converts to millimeters.
    pub fn to_millimeters(&self) -> f64 {
        self.to(LengthUnit::Millimeters)
    }

    /// Converts to feet.
    pub fn to_feet(&self) -> f64 {
        self.to(LengthUnit::Feet)
    }

    /// Converts to inches.
    pub fn to_inches(&self) -> f64 {
        self.to(LengthUnit::Inches)
    }

    /// Converts to yards.
    pub fn to_yards(&self) -> f64 {
        self.to(LengthUnit::Yards)
    }

    /// Converts to miles.
    pub fn to_miles(&self) -> f64 {
        self.to(LengthUnit::Miles)
    }

    /// Converts to angstroms.
    pub fn to_angstroms(&self) -> f64 {
        self.to(LengthUnit::Angstroms)
    }

    /// Converts to nanometers.
    pub fn to_nanometers(&self) -> f64 {
        self.to(LengthUnit::Nanometers)
    }

    /// Converts to micrometers.
    pub fn to_micrometers(&self) -> f64 {
        self.to(LengthUnit::Micrometers)
    }

    /// Converts to decimeters.
    pub fn to_decimeters(&self) -> f64 {
        self.to(LengthUnit::Decimeters)
    }

    /// Converts to hectometers.
    pub fn to_hectometers(&self) -> f64 {
        self.to(LengthUnit::Hectometers)
    }

    /// Converts to nautical miles.
    pub fn to_nautical_miles(&self) -> f64 {
        self.to(LengthUnit::NauticalMiles)
    }

    /// Converts to astronomical units.
    pub fn to_astronomical_units(&self) -> f64 {
        self.to(LengthUnit::AstronomicalUnits)
    }

    /// Converts to light years.
    pub fn to_light_years(&self) -> f64 {
        self.to(LengthUnit::LightYears)
    }

    /// Converts to parsecs.
    pub fn to_parsecs(&self) -> f64 {
        self.to(LengthUnit::Parsecs)
    }

    /// Returns the squared length (this * this).
    pub fn squared(&self) -> Area {
        *self * *self
    }

    /// Returns the cubed length (this * this * this).
    pub fn cubed(&self) -> Volume {
        self.squared() * *self
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Length {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Length {
    type Unit = LengthUnit;

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

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Length::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Length {
    type Output = Length;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Length::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Length {
    type Output = Length;

    fn mul(self, rhs: f64) -> Self::Output {
        Length::new(self.value * rhs, self.unit)
    }
}

impl Mul<Length> for f64 {
    type Output = Length;

    fn mul(self, rhs: Length) -> Self::Output {
        Length::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for Length {
    type Output = Length;

    fn div(self, rhs: f64) -> Self::Output {
        Length::new(self.value / rhs, self.unit)
    }
}

impl Div<Length> for Length {
    type Output = f64;

    fn div(self, rhs: Length) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Length {
    type Output = Length;

    fn neg(self) -> Self::Output {
        Length::new(-self.value, self.unit)
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

/// Dimension for Length.
pub struct LengthDimension;

impl Dimension for LengthDimension {
    type Quantity = Length;
    type Unit = LengthUnit;

    fn name() -> &'static str {
        "Length"
    }

    fn primary_unit() -> Self::Unit {
        LengthUnit::Meters
    }

    fn si_unit() -> Self::Unit {
        LengthUnit::Meters
    }

    fn units() -> &'static [Self::Unit] {
        LengthUnit::ALL
    }
}

/// Extension trait for creating Length quantities from numeric types.
pub trait LengthConversions {
    /// Creates a Length in angstroms.
    fn angstroms(self) -> Length;
    /// Creates a Length in nanometers.
    fn nanometers(self) -> Length;
    /// Creates a Length in micrometers.
    fn micrometers(self) -> Length;
    /// Creates a Length in millimeters.
    fn millimeters(self) -> Length;
    /// Creates a Length in centimeters.
    fn centimeters(self) -> Length;
    /// Creates a Length in decimeters.
    fn decimeters(self) -> Length;
    /// Creates a Length in meters.
    fn meters(self) -> Length;
    /// Creates a Length in hectometers.
    fn hectometers(self) -> Length;
    /// Creates a Length in kilometers.
    fn kilometers(self) -> Length;
    /// Creates a Length in inches.
    fn inches(self) -> Length;
    /// Creates a Length in feet.
    fn feet(self) -> Length;
    /// Creates a Length in yards.
    fn yards(self) -> Length;
    /// Creates a Length in miles.
    fn miles(self) -> Length;
    /// Creates a Length in nautical miles.
    fn nautical_miles(self) -> Length;
    /// Creates a Length in astronomical units.
    fn astronomical_units(self) -> Length;
    /// Creates a Length in light years.
    fn light_years(self) -> Length;
    /// Creates a Length in parsecs.
    fn parsecs(self) -> Length;
}

impl LengthConversions for f64 {
    fn angstroms(self) -> Length {
        Length::angstroms(self)
    }
    fn nanometers(self) -> Length {
        Length::nanometers(self)
    }
    fn micrometers(self) -> Length {
        Length::micrometers(self)
    }
    fn millimeters(self) -> Length {
        Length::millimeters(self)
    }
    fn centimeters(self) -> Length {
        Length::centimeters(self)
    }
    fn decimeters(self) -> Length {
        Length::decimeters(self)
    }
    fn meters(self) -> Length {
        Length::meters(self)
    }
    fn hectometers(self) -> Length {
        Length::hectometers(self)
    }
    fn kilometers(self) -> Length {
        Length::kilometers(self)
    }
    fn inches(self) -> Length {
        Length::inches(self)
    }
    fn feet(self) -> Length {
        Length::feet(self)
    }
    fn yards(self) -> Length {
        Length::yards(self)
    }
    fn miles(self) -> Length {
        Length::miles(self)
    }
    fn nautical_miles(self) -> Length {
        Length::nautical_miles(self)
    }
    fn astronomical_units(self) -> Length {
        Length::astronomical_units(self)
    }
    fn light_years(self) -> Length {
        Length::light_years(self)
    }
    fn parsecs(self) -> Length {
        Length::parsecs(self)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
