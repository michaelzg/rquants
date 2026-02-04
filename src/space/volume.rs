//! Volume quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use crate::systems::metric::{CENTI, DECI, KILO, MILLI};

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

impl_unit_display!(VolumeUnit);

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

    /// Creates a Volume in US fluid ounces.
    pub fn us_fluid_ounces(value: f64) -> Self {
        Self::new(value, VolumeUnit::UsFluidOunces)
    }

    /// Creates a Volume in US cups.
    pub fn us_cups(value: f64) -> Self {
        Self::new(value, VolumeUnit::UsCups)
    }

    /// Creates a Volume in US pints.
    pub fn us_pints(value: f64) -> Self {
        Self::new(value, VolumeUnit::UsPints)
    }

    /// Creates a Volume in US quarts.
    pub fn us_quarts(value: f64) -> Self {
        Self::new(value, VolumeUnit::UsQuarts)
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

    /// Converts to cubic millimeters.
    pub fn to_cubic_millimeters(&self) -> f64 {
        self.to(VolumeUnit::CubicMillimeters)
    }

    /// Converts to cubic centimeters.
    pub fn to_cubic_centimeters(&self) -> f64 {
        self.to(VolumeUnit::CubicCentimeters)
    }

    /// Converts to cubic kilometers.
    pub fn to_cubic_kilometers(&self) -> f64 {
        self.to(VolumeUnit::CubicKilometers)
    }

    /// Converts to cubic inches.
    pub fn to_cubic_inches(&self) -> f64 {
        self.to(VolumeUnit::CubicInches)
    }

    /// Converts to cubic yards.
    pub fn to_cubic_yards(&self) -> f64 {
        self.to(VolumeUnit::CubicYards)
    }

    /// Converts to US fluid ounces.
    pub fn to_us_fluid_ounces(&self) -> f64 {
        self.to(VolumeUnit::UsFluidOunces)
    }

    /// Converts to US cups.
    pub fn to_us_cups(&self) -> f64 {
        self.to(VolumeUnit::UsCups)
    }

    /// Converts to US pints.
    pub fn to_us_pints(&self) -> f64 {
        self.to(VolumeUnit::UsPints)
    }

    /// Converts to US quarts.
    pub fn to_us_quarts(&self) -> f64 {
        self.to(VolumeUnit::UsQuarts)
    }
}

impl_quantity!(Volume, VolumeUnit);

impl_dimension!(
    VolumeDimension,
    Volume,
    VolumeUnit,
    "Volume",
    VolumeUnit::CubicMeters,
    VolumeUnit::CubicMeters
);

/// Extension trait for creating Volume quantities from numeric types.
pub trait VolumeConversions {
    /// Creates a Volume in cubic millimeters.
    fn cubic_millimeters(self) -> Volume;
    /// Creates a Volume in cubic centimeters.
    fn cubic_centimeters(self) -> Volume;
    /// Creates a Volume in cubic meters.
    fn cubic_meters(self) -> Volume;
    /// Creates a Volume in cubic kilometers.
    fn cubic_kilometers(self) -> Volume;
    /// Creates a Volume in milliliters.
    fn milliliters(self) -> Volume;
    /// Creates a Volume in liters.
    fn liters(self) -> Volume;
    /// Creates a Volume in cubic inches.
    fn cubic_inches(self) -> Volume;
    /// Creates a Volume in cubic feet.
    fn cubic_feet(self) -> Volume;
    /// Creates a Volume in cubic yards.
    fn cubic_yards(self) -> Volume;
    /// Creates a Volume in US fluid ounces.
    fn us_fluid_ounces(self) -> Volume;
    /// Creates a Volume in US cups.
    fn us_cups(self) -> Volume;
    /// Creates a Volume in US pints.
    fn us_pints(self) -> Volume;
    /// Creates a Volume in US quarts.
    fn us_quarts(self) -> Volume;
    /// Creates a Volume in US gallons.
    fn us_gallons(self) -> Volume;
}

impl VolumeConversions for f64 {
    fn cubic_millimeters(self) -> Volume {
        Volume::cubic_millimeters(self)
    }
    fn cubic_centimeters(self) -> Volume {
        Volume::cubic_centimeters(self)
    }
    fn cubic_meters(self) -> Volume {
        Volume::cubic_meters(self)
    }
    fn cubic_kilometers(self) -> Volume {
        Volume::cubic_kilometers(self)
    }
    fn milliliters(self) -> Volume {
        Volume::milliliters(self)
    }
    fn liters(self) -> Volume {
        Volume::liters(self)
    }
    fn cubic_inches(self) -> Volume {
        Volume::cubic_inches(self)
    }
    fn cubic_feet(self) -> Volume {
        Volume::cubic_feet(self)
    }
    fn cubic_yards(self) -> Volume {
        Volume::cubic_yards(self)
    }
    fn us_fluid_ounces(self) -> Volume {
        Volume::us_fluid_ounces(self)
    }
    fn us_cups(self) -> Volume {
        Volume::us_cups(self)
    }
    fn us_pints(self) -> Volume {
        Volume::us_pints(self)
    }
    fn us_quarts(self) -> Volume {
        Volume::us_quarts(self)
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
