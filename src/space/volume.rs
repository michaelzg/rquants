//! Volume quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use crate::systems::metric::{CENTI, DECI, KILO, MILLI};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of volume measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VolumeUnit {
    // SI metric units
    /// Cubic millimeters (mm³)
    CubicMillimeters,
    /// Cubic centimeters (cm³) - same as milliliters
    CubicCentimeters,
    /// Cubic meters (m³) - SI derived unit
    CubicMeters,
    /// Cubic kilometers (km³)
    CubicKilometers,

    // Liters
    /// Milliliters (mL)
    Milliliters,
    /// Liters (L)
    Liters,

    // Imperial/US units
    /// Cubic inches (in³)
    CubicInches,
    /// Cubic feet (ft³)
    CubicFeet,
    /// Cubic yards (yd³)
    CubicYards,

    // US fluid measures
    /// US fluid ounces
    UsFluidOunces,
    /// US cups
    UsCups,
    /// US pints
    UsPints,
    /// US quarts
    UsQuarts,
    /// US gallons
    UsGallons,
}

impl VolumeUnit {
    /// All available volume units.
    pub const ALL: &'static [VolumeUnit] = &[
        VolumeUnit::CubicMillimeters,
        VolumeUnit::CubicCentimeters,
        VolumeUnit::CubicMeters,
        VolumeUnit::CubicKilometers,
        VolumeUnit::Milliliters,
        VolumeUnit::Liters,
        VolumeUnit::CubicInches,
        VolumeUnit::CubicFeet,
        VolumeUnit::CubicYards,
        VolumeUnit::UsFluidOunces,
        VolumeUnit::UsCups,
        VolumeUnit::UsPints,
        VolumeUnit::UsQuarts,
        VolumeUnit::UsGallons,
    ];
}

/// Conversion factors
const CUBIC_INCH_TO_CUBIC_METER: f64 = 1.6387064e-5;
const CUBIC_FOOT_TO_CUBIC_METER: f64 = 0.028316846592;
const CUBIC_YARD_TO_CUBIC_METER: f64 = 0.764554857984;
const US_GALLON_TO_CUBIC_METER: f64 = 0.003785411784;
const US_QUART_TO_CUBIC_METER: f64 = US_GALLON_TO_CUBIC_METER / 4.0;
const US_PINT_TO_CUBIC_METER: f64 = US_GALLON_TO_CUBIC_METER / 8.0;
const US_CUP_TO_CUBIC_METER: f64 = US_GALLON_TO_CUBIC_METER / 16.0;
const US_FL_OZ_TO_CUBIC_METER: f64 = US_GALLON_TO_CUBIC_METER / 128.0;

impl fmt::Display for VolumeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for VolumeUnit {
    fn symbol(&self) -> &'static str {
        match self {
            VolumeUnit::CubicMillimeters => "mm³",
            VolumeUnit::CubicCentimeters => "cm³",
            VolumeUnit::CubicMeters => "m³",
            VolumeUnit::CubicKilometers => "km³",
            VolumeUnit::Milliliters => "mL",
            VolumeUnit::Liters => "L",
            VolumeUnit::CubicInches => "in³",
            VolumeUnit::CubicFeet => "ft³",
            VolumeUnit::CubicYards => "yd³",
            VolumeUnit::UsFluidOunces => "fl oz",
            VolumeUnit::UsCups => "cup",
            VolumeUnit::UsPints => "pt",
            VolumeUnit::UsQuarts => "qt",
            VolumeUnit::UsGallons => "gal",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            VolumeUnit::CubicMillimeters => MILLI * MILLI * MILLI,
            VolumeUnit::CubicCentimeters => CENTI * CENTI * CENTI, // = 1e-6
            VolumeUnit::CubicMeters => 1.0,
            VolumeUnit::CubicKilometers => KILO * KILO * KILO,
            VolumeUnit::Milliliters => CENTI * CENTI * CENTI, // 1 mL = 1 cm³
            VolumeUnit::Liters => DECI * DECI * DECI,         // 1 L = 1 dm³ = 0.001 m³
            VolumeUnit::CubicInches => CUBIC_INCH_TO_CUBIC_METER,
            VolumeUnit::CubicFeet => CUBIC_FOOT_TO_CUBIC_METER,
            VolumeUnit::CubicYards => CUBIC_YARD_TO_CUBIC_METER,
            VolumeUnit::UsFluidOunces => US_FL_OZ_TO_CUBIC_METER,
            VolumeUnit::UsCups => US_CUP_TO_CUBIC_METER,
            VolumeUnit::UsPints => US_PINT_TO_CUBIC_METER,
            VolumeUnit::UsQuarts => US_QUART_TO_CUBIC_METER,
            VolumeUnit::UsGallons => US_GALLON_TO_CUBIC_METER,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            VolumeUnit::CubicMillimeters
                | VolumeUnit::CubicCentimeters
                | VolumeUnit::CubicMeters
                | VolumeUnit::CubicKilometers
                | VolumeUnit::Milliliters
                | VolumeUnit::Liters
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct Volume {
    value: f64,
    unit: VolumeUnit,
}

impl Volume {
    /// Creates a new Volume quantity.
    pub const fn new_const(value: f64, unit: VolumeUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a Volume in cubic millimeters.
    pub fn cubic_millimeters(value: f64) -> Self {
        Self::new(value, VolumeUnit::CubicMillimeters)
    }

    /// Creates a Volume in cubic centimeters.
    pub fn cubic_centimeters(value: f64) -> Self {
        Self::new(value, VolumeUnit::CubicCentimeters)
    }

    /// Creates a Volume in cubic meters.
    pub fn cubic_meters(value: f64) -> Self {
        Self::new(value, VolumeUnit::CubicMeters)
    }

    /// Creates a Volume in cubic kilometers.
    pub fn cubic_kilometers(value: f64) -> Self {
        Self::new(value, VolumeUnit::CubicKilometers)
    }

    /// Creates a Volume in milliliters.
    pub fn milliliters(value: f64) -> Self {
        Self::new(value, VolumeUnit::Milliliters)
    }

    /// Creates a Volume in liters.
    pub fn liters(value: f64) -> Self {
        Self::new(value, VolumeUnit::Liters)
    }

    /// Creates a Volume in cubic inches.
    pub fn cubic_inches(value: f64) -> Self {
        Self::new(value, VolumeUnit::CubicInches)
    }

    /// Creates a Volume in cubic feet.
    pub fn cubic_feet(value: f64) -> Self {
        Self::new(value, VolumeUnit::CubicFeet)
    }

    /// Creates a Volume in cubic yards.
    pub fn cubic_yards(value: f64) -> Self {
        Self::new(value, VolumeUnit::CubicYards)
    }

    /// Creates a Volume in US gallons.
    pub fn us_gallons(value: f64) -> Self {
        Self::new(value, VolumeUnit::UsGallons)
    }

    // Conversion methods
    /// Converts to cubic meters.
    pub fn to_cubic_meters(&self) -> f64 {
        self.to(VolumeUnit::CubicMeters)
    }

    /// Converts to liters.
    pub fn to_liters(&self) -> f64 {
        self.to(VolumeUnit::Liters)
    }

    /// Converts to milliliters.
    pub fn to_milliliters(&self) -> f64 {
        self.to(VolumeUnit::Milliliters)
    }

    /// Converts to cubic feet.
    pub fn to_cubic_feet(&self) -> f64 {
        self.to(VolumeUnit::CubicFeet)
    }

    /// Converts to US gallons.
    pub fn to_us_gallons(&self) -> f64 {
        self.to(VolumeUnit::UsGallons)
    }
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Volume {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Volume {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Volume {
    type Unit = VolumeUnit;

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

impl Add for Volume {
    type Output = Volume;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Volume::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Volume {
    type Output = Volume;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Volume::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Volume {
    type Output = Volume;

    fn mul(self, rhs: f64) -> Self::Output {
        Volume::new(self.value * rhs, self.unit)
    }
}

impl Mul<Volume> for f64 {
    type Output = Volume;

    fn mul(self, rhs: Volume) -> Self::Output {
        Volume::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for Volume {
    type Output = Volume;

    fn div(self, rhs: f64) -> Self::Output {
        Volume::new(self.value / rhs, self.unit)
    }
}

impl Div<Volume> for Volume {
    type Output = f64;

    fn div(self, rhs: Volume) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Volume {
    type Output = Volume;

    fn neg(self) -> Self::Output {
        Volume::new(-self.value, self.unit)
    }
}

/// Dimension for Volume.
pub struct VolumeDimension;

impl Dimension for VolumeDimension {
    type Quantity = Volume;
    type Unit = VolumeUnit;

    fn name() -> &'static str {
        "Volume"
    }

    fn primary_unit() -> Self::Unit {
        VolumeUnit::CubicMeters
    }

    fn si_unit() -> Self::Unit {
        VolumeUnit::CubicMeters
    }

    fn units() -> &'static [Self::Unit] {
        VolumeUnit::ALL
    }
}

/// Extension trait for creating Volume quantities from numeric types.
pub trait VolumeConversions {
    /// Creates a Volume in cubic meters.
    fn cubic_meters(self) -> Volume;
    /// Creates a Volume in liters.
    fn liters(self) -> Volume;
    /// Creates a Volume in milliliters.
    fn milliliters(self) -> Volume;
    /// Creates a Volume in cubic feet.
    fn cubic_feet(self) -> Volume;
    /// Creates a Volume in US gallons.
    fn us_gallons(self) -> Volume;
}

impl VolumeConversions for f64 {
    fn cubic_meters(self) -> Volume {
        Volume::cubic_meters(self)
    }
    fn liters(self) -> Volume {
        Volume::liters(self)
    }
    fn milliliters(self) -> Volume {
        Volume::milliliters(self)
    }
    fn cubic_feet(self) -> Volume {
        Volume::cubic_feet(self)
    }
    fn us_gallons(self) -> Volume {
        Volume::us_gallons(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
