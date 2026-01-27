//! Density quantity and units.

use super::mass::{Mass, MassUnit};
use crate::core::{Dimension, Quantity, UnitOfMeasure};
use crate::space::volume::{Volume, VolumeUnit};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of density measurement (mass per volume).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DensityUnit {
    /// Kilograms per cubic meter (kg/m³) - SI unit
    KilogramsPerCubicMeter,
    /// Kilograms per liter (kg/L)
    KilogramsPerLiter,
    /// Grams per liter (g/L)
    GramsPerLiter,
    /// Milligrams per liter (mg/L)
    MilligramsPerLiter,
    /// Grams per milliliter (g/mL)
    GramsPerMilliliter,
    /// Grams per cubic centimeter (g/cm³)
    GramsPerCubicCentimeter,
    /// Pounds per cubic foot (lb/ft³)
    PoundsPerCubicFoot,
    /// Pounds per gallon (lb/gal)
    PoundsPerGallon,
}

impl DensityUnit {
    /// All available density units.
    pub const ALL: &'static [DensityUnit] = &[
        DensityUnit::KilogramsPerCubicMeter,
        DensityUnit::KilogramsPerLiter,
        DensityUnit::GramsPerLiter,
        DensityUnit::MilligramsPerLiter,
        DensityUnit::GramsPerMilliliter,
        DensityUnit::GramsPerCubicCentimeter,
        DensityUnit::PoundsPerCubicFoot,
        DensityUnit::PoundsPerGallon,
    ];
}

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

impl fmt::Display for DensityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for DensityUnit {
    fn symbol(&self) -> &'static str {
        match self {
            DensityUnit::KilogramsPerCubicMeter => "kg/m³",
            DensityUnit::KilogramsPerLiter => "kg/L",
            DensityUnit::GramsPerLiter => "g/L",
            DensityUnit::MilligramsPerLiter => "mg/L",
            DensityUnit::GramsPerMilliliter => "g/mL",
            DensityUnit::GramsPerCubicCentimeter => "g/cm³",
            DensityUnit::PoundsPerCubicFoot => "lb/ft³",
            DensityUnit::PoundsPerGallon => "lb/gal",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            DensityUnit::KilogramsPerCubicMeter => 1.0,
            DensityUnit::KilogramsPerLiter => KG_PER_LITER_FACTOR,
            DensityUnit::GramsPerLiter => G_PER_LITER_FACTOR,
            DensityUnit::MilligramsPerLiter => MG_PER_LITER_FACTOR,
            DensityUnit::GramsPerMilliliter => G_PER_ML_FACTOR,
            DensityUnit::GramsPerCubicCentimeter => G_PER_ML_FACTOR, // same as g/mL
            DensityUnit::PoundsPerCubicFoot => LB_PER_CUFT_FACTOR,
            DensityUnit::PoundsPerGallon => LB_PER_GAL_FACTOR,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            DensityUnit::KilogramsPerCubicMeter
                | DensityUnit::KilogramsPerLiter
                | DensityUnit::GramsPerLiter
                | DensityUnit::MilligramsPerLiter
                | DensityUnit::GramsPerMilliliter
                | DensityUnit::GramsPerCubicCentimeter
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct Density {
    value: f64,
    unit: DensityUnit,
}

impl Density {
    /// Creates a new Density quantity.
    pub const fn new_const(value: f64, unit: DensityUnit) -> Self {
        Self { value, unit }
    }

    /// Creates a Density from mass and volume.
    pub fn from_mass_and_volume(mass: Mass, volume: Volume) -> Self {
        let kg_per_m3 = mass.to_kilograms() / volume.to_cubic_meters();
        Self::new(kg_per_m3, DensityUnit::KilogramsPerCubicMeter)
    }

    // Constructors
    /// Creates a Density in kg/m³.
    pub fn kilograms_per_cubic_meter(value: f64) -> Self {
        Self::new(value, DensityUnit::KilogramsPerCubicMeter)
    }

    /// Creates a Density in kg/L.
    pub fn kilograms_per_liter(value: f64) -> Self {
        Self::new(value, DensityUnit::KilogramsPerLiter)
    }

    /// Creates a Density in g/L.
    pub fn grams_per_liter(value: f64) -> Self {
        Self::new(value, DensityUnit::GramsPerLiter)
    }

    /// Creates a Density in mg/L.
    pub fn milligrams_per_liter(value: f64) -> Self {
        Self::new(value, DensityUnit::MilligramsPerLiter)
    }

    /// Creates a Density in g/mL.
    pub fn grams_per_milliliter(value: f64) -> Self {
        Self::new(value, DensityUnit::GramsPerMilliliter)
    }

    /// Creates a Density in g/cm³.
    pub fn grams_per_cubic_centimeter(value: f64) -> Self {
        Self::new(value, DensityUnit::GramsPerCubicCentimeter)
    }

    /// Creates a Density in lb/ft³.
    pub fn pounds_per_cubic_foot(value: f64) -> Self {
        Self::new(value, DensityUnit::PoundsPerCubicFoot)
    }

    // Conversion methods
    /// Converts to kg/m³.
    pub fn to_kilograms_per_cubic_meter(&self) -> f64 {
        self.to(DensityUnit::KilogramsPerCubicMeter)
    }

    /// Converts to kg/L.
    pub fn to_kilograms_per_liter(&self) -> f64 {
        self.to(DensityUnit::KilogramsPerLiter)
    }

    /// Converts to g/L.
    pub fn to_grams_per_liter(&self) -> f64 {
        self.to(DensityUnit::GramsPerLiter)
    }

    /// Converts to g/mL.
    pub fn to_grams_per_milliliter(&self) -> f64 {
        self.to(DensityUnit::GramsPerMilliliter)
    }

    /// Converts to g/cm³.
    pub fn to_grams_per_cubic_centimeter(&self) -> f64 {
        self.to(DensityUnit::GramsPerCubicCentimeter)
    }
}

impl fmt::Display for Density {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Density {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Density {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Density {
    type Unit = DensityUnit;

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

impl Add for Density {
    type Output = Density;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Density::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Density {
    type Output = Density;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Density::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Density {
    type Output = Density;

    fn mul(self, rhs: f64) -> Self::Output {
        Density::new(self.value * rhs, self.unit)
    }
}

impl Mul<Density> for f64 {
    type Output = Density;

    fn mul(self, rhs: Density) -> Self::Output {
        Density::new(self * rhs.value, rhs.unit)
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

impl Div<f64> for Density {
    type Output = Density;

    fn div(self, rhs: f64) -> Self::Output {
        Density::new(self.value / rhs, self.unit)
    }
}

impl Div<Density> for Density {
    type Output = f64;

    fn div(self, rhs: Density) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Density {
    type Output = Density;

    fn neg(self) -> Self::Output {
        Density::new(-self.value, self.unit)
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

/// Dimension for Density.
pub struct DensityDimension;

impl Dimension for DensityDimension {
    type Quantity = Density;
    type Unit = DensityUnit;

    fn name() -> &'static str {
        "Density"
    }

    fn primary_unit() -> Self::Unit {
        DensityUnit::KilogramsPerCubicMeter
    }

    fn si_unit() -> Self::Unit {
        DensityUnit::KilogramsPerCubicMeter
    }

    fn units() -> &'static [Self::Unit] {
        DensityUnit::ALL
    }
}

/// Extension trait for creating Density quantities from numeric types.
pub trait DensityConversions {
    /// Creates a Density in kg/m³.
    fn kilograms_per_cubic_meter(self) -> Density;
    /// Creates a Density in kg/L.
    fn kilograms_per_liter(self) -> Density;
    /// Creates a Density in g/L.
    fn grams_per_liter(self) -> Density;
    /// Creates a Density in g/mL.
    fn grams_per_milliliter(self) -> Density;
}

impl DensityConversions for f64 {
    fn kilograms_per_cubic_meter(self) -> Density {
        Density::kilograms_per_cubic_meter(self)
    }
    fn kilograms_per_liter(self) -> Density {
        Density::kilograms_per_liter(self)
    }
    fn grams_per_liter(self) -> Density {
        Density::grams_per_liter(self)
    }
    fn grams_per_milliliter(self) -> Density {
        Density::grams_per_milliliter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
