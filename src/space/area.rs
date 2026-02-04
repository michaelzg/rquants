//! Area quantity and units.

use super::length::Length;
use super::volume::{Volume, VolumeUnit};
use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use crate::systems::metric::{CENTI, HECTO, KILO, MILLI};
use std::ops::{Mul};

/// Units of area measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AreaUnit {
    // SI metric units
    /// Square millimeters (mm²)
    SquareMillimeters,
    /// Square centimeters (cm²)
    SquareCentimeters,
    /// Square meters (m²) - SI derived unit
    SquareMeters,
    /// Square kilometers (km²)
    SquareKilometers,
    /// Hectares (ha) - 10,000 m²
    Hectares,

    // Imperial/US units
    /// Square inches (in²)
    SquareInches,
    /// Square feet (ft²)
    SquareFeet,
    /// Square yards (yd²)
    SquareYards,
    /// Square miles (mi²)
    SquareMiles,
    /// Acres
    Acres,
}

impl AreaUnit {
    /// All available area units.
    pub const ALL: &'static [AreaUnit] = &[
        AreaUnit::SquareMillimeters,
        AreaUnit::SquareCentimeters,
        AreaUnit::SquareMeters,
        AreaUnit::SquareKilometers,
        AreaUnit::Hectares,
        AreaUnit::SquareInches,
        AreaUnit::SquareFeet,
        AreaUnit::SquareYards,
        AreaUnit::SquareMiles,
        AreaUnit::Acres,
    ];
}

/// Conversion factors
const SQ_FOOT_TO_SQ_METER: f64 = 0.09290304;
const SQ_YARD_TO_SQ_METER: f64 = 0.83612736;
const SQ_MILE_TO_SQ_METER: f64 = 2_589_988.110336;
const ACRE_TO_SQ_METER: f64 = 4046.8564224;
const SQ_INCH_TO_SQ_METER: f64 = 0.00064516;

impl_unit_display!(AreaUnit);

impl UnitOfMeasure for AreaUnit {
    fn symbol(&self) -> &'static str {
        match self {
            AreaUnit::SquareMillimeters => "mm²",
            AreaUnit::SquareCentimeters => "cm²",
            AreaUnit::SquareMeters => "m²",
            AreaUnit::SquareKilometers => "km²",
            AreaUnit::Hectares => "ha",
            AreaUnit::SquareInches => "in²",
            AreaUnit::SquareFeet => "ft²",
            AreaUnit::SquareYards => "yd²",
            AreaUnit::SquareMiles => "mi²",
            AreaUnit::Acres => "ac",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            AreaUnit::SquareMillimeters => MILLI * MILLI,
            AreaUnit::SquareCentimeters => CENTI * CENTI,
            AreaUnit::SquareMeters => 1.0,
            AreaUnit::SquareKilometers => KILO * KILO,
            AreaUnit::Hectares => HECTO * HECTO,
            AreaUnit::SquareInches => SQ_INCH_TO_SQ_METER,
            AreaUnit::SquareFeet => SQ_FOOT_TO_SQ_METER,
            AreaUnit::SquareYards => SQ_YARD_TO_SQ_METER,
            AreaUnit::SquareMiles => SQ_MILE_TO_SQ_METER,
            AreaUnit::Acres => ACRE_TO_SQ_METER,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            AreaUnit::SquareMillimeters
                | AreaUnit::SquareCentimeters
                | AreaUnit::SquareMeters
                | AreaUnit::SquareKilometers
                | AreaUnit::Hectares
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct Area {
    value: f64,
    unit: AreaUnit,
}

impl Area {
    /// Creates a new Area quantity.
    pub const fn new_const(value: f64, unit: AreaUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates an Area in square millimeters.
    pub fn square_millimeters(value: f64) -> Self {
        Self::new(value, AreaUnit::SquareMillimeters)
    }

    /// Creates an Area in square centimeters.
    pub fn square_centimeters(value: f64) -> Self {
        Self::new(value, AreaUnit::SquareCentimeters)
    }

    /// Creates an Area in square meters.
    pub fn square_meters(value: f64) -> Self {
        Self::new(value, AreaUnit::SquareMeters)
    }

    /// Creates an Area in square kilometers.
    pub fn square_kilometers(value: f64) -> Self {
        Self::new(value, AreaUnit::SquareKilometers)
    }

    /// Creates an Area in hectares.
    pub fn hectares(value: f64) -> Self {
        Self::new(value, AreaUnit::Hectares)
    }

    /// Creates an Area in square inches.
    pub fn square_inches(value: f64) -> Self {
        Self::new(value, AreaUnit::SquareInches)
    }

    /// Creates an Area in square feet.
    pub fn square_feet(value: f64) -> Self {
        Self::new(value, AreaUnit::SquareFeet)
    }

    /// Creates an Area in square yards.
    pub fn square_yards(value: f64) -> Self {
        Self::new(value, AreaUnit::SquareYards)
    }

    /// Creates an Area in square miles.
    pub fn square_miles(value: f64) -> Self {
        Self::new(value, AreaUnit::SquareMiles)
    }

    /// Creates an Area in acres.
    pub fn acres(value: f64) -> Self {
        Self::new(value, AreaUnit::Acres)
    }

    // Conversion methods
    /// Converts to square meters.
    pub fn to_square_meters(&self) -> f64 {
        self.to(AreaUnit::SquareMeters)
    }

    /// Converts to square kilometers.
    pub fn to_square_kilometers(&self) -> f64 {
        self.to(AreaUnit::SquareKilometers)
    }

    /// Converts to hectares.
    pub fn to_hectares(&self) -> f64 {
        self.to(AreaUnit::Hectares)
    }

    /// Converts to square feet.
    pub fn to_square_feet(&self) -> f64 {
        self.to(AreaUnit::SquareFeet)
    }

    /// Converts to acres.
    pub fn to_acres(&self) -> f64 {
        self.to(AreaUnit::Acres)
    }

    /// Converts to square millimeters.
    pub fn to_square_millimeters(&self) -> f64 {
        self.to(AreaUnit::SquareMillimeters)
    }

    /// Converts to square centimeters.
    pub fn to_square_centimeters(&self) -> f64 {
        self.to(AreaUnit::SquareCentimeters)
    }

    /// Converts to square inches.
    pub fn to_square_inches(&self) -> f64 {
        self.to(AreaUnit::SquareInches)
    }

    /// Converts to square yards.
    pub fn to_square_yards(&self) -> f64 {
        self.to(AreaUnit::SquareYards)
    }

    /// Converts to square miles.
    pub fn to_square_miles(&self) -> f64 {
        self.to(AreaUnit::SquareMiles)
    }
}

impl_quantity!(Area, AreaUnit);

// Area * Length = Volume
impl Mul<Length> for Area {
    type Output = Volume;

    fn mul(self, rhs: Length) -> Self::Output {
        let volume_meters_cu = self.to_square_meters() * rhs.to_meters();
        Volume::new(volume_meters_cu, VolumeUnit::CubicMeters)
    }
}

impl_dimension!(
    AreaDimension,
    Area,
    AreaUnit,
    "Area",
    AreaUnit::SquareMeters,
    AreaUnit::SquareMeters
);

/// Extension trait for creating Area quantities from numeric types.
pub trait AreaConversions {
    /// Creates an Area in square millimeters.
    fn square_millimeters(self) -> Area;
    /// Creates an Area in square centimeters.
    fn square_centimeters(self) -> Area;
    /// Creates an Area in square meters.
    fn square_meters(self) -> Area;
    /// Creates an Area in square kilometers.
    fn square_kilometers(self) -> Area;
    /// Creates an Area in hectares.
    fn hectares(self) -> Area;
    /// Creates an Area in square inches.
    fn square_inches(self) -> Area;
    /// Creates an Area in square feet.
    fn square_feet(self) -> Area;
    /// Creates an Area in square yards.
    fn square_yards(self) -> Area;
    /// Creates an Area in square miles.
    fn square_miles(self) -> Area;
    /// Creates an Area in acres.
    fn acres(self) -> Area;
}

impl AreaConversions for f64 {
    fn square_millimeters(self) -> Area {
        Area::square_millimeters(self)
    }
    fn square_centimeters(self) -> Area {
        Area::square_centimeters(self)
    }
    fn square_meters(self) -> Area {
        Area::square_meters(self)
    }
    fn square_kilometers(self) -> Area {
        Area::square_kilometers(self)
    }
    fn hectares(self) -> Area {
        Area::hectares(self)
    }
    fn square_inches(self) -> Area {
        Area::square_inches(self)
    }
    fn square_feet(self) -> Area {
        Area::square_feet(self)
    }
    fn square_yards(self) -> Area {
        Area::square_yards(self)
    }
    fn square_miles(self) -> Area {
        Area::square_miles(self)
    }
    fn acres(self) -> Area {
        Area::acres(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
