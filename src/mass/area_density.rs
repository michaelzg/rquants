//! Area density quantity and units (mass per area).

use super::mass::{Mass, MassUnit};
use crate::core::{Dimension, Quantity, UnitOfMeasure};
use crate::space::area::{Area, AreaUnit};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of area density measurement (mass per area).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AreaDensityUnit {
    /// Kilograms per square meter (kg/m²) - SI unit
    KilogramsPerSquareMeter,
    /// Kilograms per hectare (kg/ha)
    KilogramsPerHectare,
    /// Grams per square centimeter (g/cm²)
    GramsPerSquareCentimeter,
    /// Pounds per acre (lb/ac)
    PoundsPerAcre,
}

impl AreaDensityUnit {
    /// All available area density units.
    pub const ALL: &'static [AreaDensityUnit] = &[
        AreaDensityUnit::KilogramsPerSquareMeter,
        AreaDensityUnit::KilogramsPerHectare,
        AreaDensityUnit::GramsPerSquareCentimeter,
        AreaDensityUnit::PoundsPerAcre,
    ];
}

// Conversion factors to kg/m² (primary unit)
// 1 hectare = 10,000 m², so 1 kg/ha = 0.0001 kg/m²
const KG_PER_HECTARE_FACTOR: f64 = 0.0001;
// 1 cm² = 0.0001 m², 1 g = 0.001 kg, so 1 g/cm² = 0.001/0.0001 = 10 kg/m²
const G_PER_CM2_FACTOR: f64 = 10.0;
// 1 lb ≈ 0.4536 kg, 1 acre ≈ 4046.86 m², so 1 lb/ac ≈ 0.000112 kg/m²
const LB_PER_ACRE_FACTOR: f64 = 0.45359237 / 4046.8564224;

impl fmt::Display for AreaDensityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for AreaDensityUnit {
    fn symbol(&self) -> &'static str {
        match self {
            AreaDensityUnit::KilogramsPerSquareMeter => "kg/m²",
            AreaDensityUnit::KilogramsPerHectare => "kg/ha",
            AreaDensityUnit::GramsPerSquareCentimeter => "g/cm²",
            AreaDensityUnit::PoundsPerAcre => "lb/ac",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            AreaDensityUnit::KilogramsPerSquareMeter => 1.0,
            AreaDensityUnit::KilogramsPerHectare => KG_PER_HECTARE_FACTOR,
            AreaDensityUnit::GramsPerSquareCentimeter => G_PER_CM2_FACTOR,
            AreaDensityUnit::PoundsPerAcre => LB_PER_ACRE_FACTOR,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            AreaDensityUnit::KilogramsPerSquareMeter | AreaDensityUnit::GramsPerSquareCentimeter
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct AreaDensity {
    value: f64,
    unit: AreaDensityUnit,
}

impl AreaDensity {
    /// Creates a new AreaDensity quantity.
    pub const fn new_const(value: f64, unit: AreaDensityUnit) -> Self {
        Self { value, unit }
    }

    /// Creates an AreaDensity from mass and area.
    pub fn from_mass_and_area(mass: Mass, area: Area) -> Self {
        let kg_per_m2 = mass.to_kilograms() / area.to_square_meters();
        Self::new(kg_per_m2, AreaDensityUnit::KilogramsPerSquareMeter)
    }

    // Constructors
    /// Creates an AreaDensity in kg/m².
    pub fn kilograms_per_square_meter(value: f64) -> Self {
        Self::new(value, AreaDensityUnit::KilogramsPerSquareMeter)
    }

    /// Creates an AreaDensity in kg/ha.
    pub fn kilograms_per_hectare(value: f64) -> Self {
        Self::new(value, AreaDensityUnit::KilogramsPerHectare)
    }

    /// Creates an AreaDensity in g/cm².
    pub fn grams_per_square_centimeter(value: f64) -> Self {
        Self::new(value, AreaDensityUnit::GramsPerSquareCentimeter)
    }

    /// Creates an AreaDensity in lb/ac.
    pub fn pounds_per_acre(value: f64) -> Self {
        Self::new(value, AreaDensityUnit::PoundsPerAcre)
    }

    // Conversion methods
    /// Converts to kg/m².
    pub fn to_kilograms_per_square_meter(&self) -> f64 {
        self.to(AreaDensityUnit::KilogramsPerSquareMeter)
    }

    /// Converts to kg/ha.
    pub fn to_kilograms_per_hectare(&self) -> f64 {
        self.to(AreaDensityUnit::KilogramsPerHectare)
    }

    /// Converts to g/cm².
    pub fn to_grams_per_square_centimeter(&self) -> f64 {
        self.to(AreaDensityUnit::GramsPerSquareCentimeter)
    }

    /// Converts to lb/ac.
    pub fn to_pounds_per_acre(&self) -> f64 {
        self.to(AreaDensityUnit::PoundsPerAcre)
    }
}

impl fmt::Display for AreaDensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for AreaDensity {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for AreaDensity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for AreaDensity {
    type Unit = AreaDensityUnit;

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

impl Add for AreaDensity {
    type Output = AreaDensity;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        AreaDensity::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for AreaDensity {
    type Output = AreaDensity;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        AreaDensity::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for AreaDensity {
    type Output = AreaDensity;

    fn mul(self, rhs: f64) -> Self::Output {
        AreaDensity::new(self.value * rhs, self.unit)
    }
}

impl Mul<AreaDensity> for f64 {
    type Output = AreaDensity;

    fn mul(self, rhs: AreaDensity) -> Self::Output {
        AreaDensity::new(self * rhs.value, rhs.unit)
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

impl Div<f64> for AreaDensity {
    type Output = AreaDensity;

    fn div(self, rhs: f64) -> Self::Output {
        AreaDensity::new(self.value / rhs, self.unit)
    }
}

impl Div<AreaDensity> for AreaDensity {
    type Output = f64;

    fn div(self, rhs: AreaDensity) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for AreaDensity {
    type Output = AreaDensity;

    fn neg(self) -> Self::Output {
        AreaDensity::new(-self.value, self.unit)
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

/// Dimension for AreaDensity.
pub struct AreaDensityDimension;

impl Dimension for AreaDensityDimension {
    type Quantity = AreaDensity;
    type Unit = AreaDensityUnit;

    fn name() -> &'static str {
        "AreaDensity"
    }

    fn primary_unit() -> Self::Unit {
        AreaDensityUnit::KilogramsPerSquareMeter
    }

    fn si_unit() -> Self::Unit {
        AreaDensityUnit::KilogramsPerSquareMeter
    }

    fn units() -> &'static [Self::Unit] {
        AreaDensityUnit::ALL
    }
}

/// Extension trait for creating AreaDensity quantities from numeric types.
pub trait AreaDensityConversions {
    /// Creates an AreaDensity in kg/m².
    fn kilograms_per_square_meter(self) -> AreaDensity;
    /// Creates an AreaDensity in kg/ha.
    fn kilograms_per_hectare(self) -> AreaDensity;
    /// Creates an AreaDensity in g/cm².
    fn grams_per_square_centimeter(self) -> AreaDensity;
    /// Creates an AreaDensity in lb/ac.
    fn pounds_per_acre(self) -> AreaDensity;
}

impl AreaDensityConversions for f64 {
    fn kilograms_per_square_meter(self) -> AreaDensity {
        AreaDensity::kilograms_per_square_meter(self)
    }
    fn kilograms_per_hectare(self) -> AreaDensity {
        AreaDensity::kilograms_per_hectare(self)
    }
    fn grams_per_square_centimeter(self) -> AreaDensity {
        AreaDensity::grams_per_square_centimeter(self)
    }
    fn pounds_per_acre(self) -> AreaDensity {
        AreaDensity::pounds_per_acre(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
