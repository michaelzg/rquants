//! Pressure quantity and units.

use super::force::{Force, ForceUnit};
use crate::core::{Dimension, Quantity, UnitOfMeasure};
use crate::space::area::{Area, AreaUnit};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of pressure measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PressureUnit {
    /// Pascals (Pa) - SI unit (N/m²)
    Pascals,
    /// Kilopascals (kPa)
    Kilopascals,
    /// Megapascals (MPa)
    Megapascals,
    /// Bars (bar)
    Bars,
    /// Pounds per square inch (psi)
    PoundsPerSquareInch,
    /// Standard atmospheres (atm)
    Atmospheres,
    /// Millimeters of mercury (mmHg)
    MillimetersOfMercury,
    /// Inches of mercury (inHg)
    InchesOfMercury,
    /// Torr
    Torr,
}

impl PressureUnit {
    /// All available pressure units.
    pub const ALL: &'static [PressureUnit] = &[
        PressureUnit::Pascals,
        PressureUnit::Kilopascals,
        PressureUnit::Megapascals,
        PressureUnit::Bars,
        PressureUnit::PoundsPerSquareInch,
        PressureUnit::Atmospheres,
        PressureUnit::MillimetersOfMercury,
        PressureUnit::InchesOfMercury,
        PressureUnit::Torr,
    ];
}

// Conversion factors to Pascals
const BAR_TO_PA: f64 = 100_000.0;
const ATM_TO_PA: f64 = 101_325.0;
const PSI_TO_PA: f64 = 6894.757293168;
const MMHG_TO_PA: f64 = 133.322387415;
const INHG_TO_PA: f64 = 3386.389;
const TORR_TO_PA: f64 = ATM_TO_PA / 760.0;

impl fmt::Display for PressureUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for PressureUnit {
    fn symbol(&self) -> &'static str {
        match self {
            PressureUnit::Pascals => "Pa",
            PressureUnit::Kilopascals => "kPa",
            PressureUnit::Megapascals => "MPa",
            PressureUnit::Bars => "bar",
            PressureUnit::PoundsPerSquareInch => "psi",
            PressureUnit::Atmospheres => "atm",
            PressureUnit::MillimetersOfMercury => "mmHg",
            PressureUnit::InchesOfMercury => "inHg",
            PressureUnit::Torr => "Torr",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            PressureUnit::Pascals => 1.0,
            PressureUnit::Kilopascals => 1000.0,
            PressureUnit::Megapascals => 1_000_000.0,
            PressureUnit::Bars => BAR_TO_PA,
            PressureUnit::PoundsPerSquareInch => PSI_TO_PA,
            PressureUnit::Atmospheres => ATM_TO_PA,
            PressureUnit::MillimetersOfMercury => MMHG_TO_PA,
            PressureUnit::InchesOfMercury => INHG_TO_PA,
            PressureUnit::Torr => TORR_TO_PA,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            PressureUnit::Pascals | PressureUnit::Kilopascals | PressureUnit::Megapascals
        )
    }
}

/// A quantity of pressure (force per unit area).
///
/// Pressure represents force distributed over an area.
/// P = F / A
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let pressure = Pressure::atmospheres(1.0);
/// let area = Area::square_meters(1.0);
///
/// // Force = Pressure * Area
/// let force = pressure * area;
/// assert!((force.to_newtons() - 101325.0).abs() < 1.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Pressure {
    value: f64,
    unit: PressureUnit,
}

impl Pressure {
    /// Creates a new Pressure quantity.
    pub const fn new_const(value: f64, unit: PressureUnit) -> Self {
        Self { value, unit }
    }

    /// Creates a Pressure from force and area (P = F/A).
    pub fn from_force_and_area(force: Force, area: Area) -> Self {
        let pascals = force.to_newtons() / area.to_square_meters();
        Self::new(pascals, PressureUnit::Pascals)
    }

    // Constructors
    /// Creates a Pressure in Pascals.
    pub fn pascals(value: f64) -> Self {
        Self::new(value, PressureUnit::Pascals)
    }

    /// Creates a Pressure in kilopascals.
    pub fn kilopascals(value: f64) -> Self {
        Self::new(value, PressureUnit::Kilopascals)
    }

    /// Creates a Pressure in megapascals.
    pub fn megapascals(value: f64) -> Self {
        Self::new(value, PressureUnit::Megapascals)
    }

    /// Creates a Pressure in bars.
    pub fn bars(value: f64) -> Self {
        Self::new(value, PressureUnit::Bars)
    }

    /// Creates a Pressure in psi.
    pub fn psi(value: f64) -> Self {
        Self::new(value, PressureUnit::PoundsPerSquareInch)
    }

    /// Creates a Pressure in atmospheres.
    pub fn atmospheres(value: f64) -> Self {
        Self::new(value, PressureUnit::Atmospheres)
    }

    /// Creates a Pressure in mmHg.
    pub fn millimeters_of_mercury(value: f64) -> Self {
        Self::new(value, PressureUnit::MillimetersOfMercury)
    }

    /// Creates a Pressure in Torr.
    pub fn torr(value: f64) -> Self {
        Self::new(value, PressureUnit::Torr)
    }

    // Conversion methods
    /// Converts to Pascals.
    pub fn to_pascals(&self) -> f64 {
        self.to(PressureUnit::Pascals)
    }

    /// Converts to kilopascals.
    pub fn to_kilopascals(&self) -> f64 {
        self.to(PressureUnit::Kilopascals)
    }

    /// Converts to bars.
    pub fn to_bars(&self) -> f64 {
        self.to(PressureUnit::Bars)
    }

    /// Converts to psi.
    pub fn to_psi(&self) -> f64 {
        self.to(PressureUnit::PoundsPerSquareInch)
    }

    /// Converts to atmospheres.
    pub fn to_atmospheres(&self) -> f64 {
        self.to(PressureUnit::Atmospheres)
    }

    /// Converts to mmHg.
    pub fn to_millimeters_of_mercury(&self) -> f64 {
        self.to(PressureUnit::MillimetersOfMercury)
    }

    /// Converts to Torr.
    pub fn to_torr(&self) -> f64 {
        self.to(PressureUnit::Torr)
    }
}

impl fmt::Display for Pressure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Pressure {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Pressure {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Pressure {
    type Unit = PressureUnit;

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

impl Add for Pressure {
    type Output = Pressure;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Pressure::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Pressure {
    type Output = Pressure;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Pressure::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Pressure {
    type Output = Pressure;

    fn mul(self, rhs: f64) -> Self::Output {
        Pressure::new(self.value * rhs, self.unit)
    }
}

impl Mul<Pressure> for f64 {
    type Output = Pressure;

    fn mul(self, rhs: Pressure) -> Self::Output {
        Pressure::new(self * rhs.value, rhs.unit)
    }
}

// Pressure * Area = Force
impl Mul<Area> for Pressure {
    type Output = Force;

    fn mul(self, rhs: Area) -> Self::Output {
        let newtons = self.to_pascals() * rhs.to_square_meters();
        Force::new(newtons, ForceUnit::Newtons)
    }
}

impl Div<f64> for Pressure {
    type Output = Pressure;

    fn div(self, rhs: f64) -> Self::Output {
        Pressure::new(self.value / rhs, self.unit)
    }
}

impl Div<Pressure> for Pressure {
    type Output = f64;

    fn div(self, rhs: Pressure) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Pressure {
    type Output = Pressure;

    fn neg(self) -> Self::Output {
        Pressure::new(-self.value, self.unit)
    }
}

// Force / Area = Pressure
impl Div<Area> for Force {
    type Output = Pressure;

    fn div(self, rhs: Area) -> Self::Output {
        Pressure::from_force_and_area(self, rhs)
    }
}

// Force / Pressure = Area
impl Div<Pressure> for Force {
    type Output = Area;

    fn div(self, rhs: Pressure) -> Self::Output {
        let sqm = self.to_newtons() / rhs.to_pascals();
        Area::new(sqm, AreaUnit::SquareMeters)
    }
}

/// Dimension for Pressure.
pub struct PressureDimension;

impl Dimension for PressureDimension {
    type Quantity = Pressure;
    type Unit = PressureUnit;

    fn name() -> &'static str {
        "Pressure"
    }

    fn primary_unit() -> Self::Unit {
        PressureUnit::Pascals
    }

    fn si_unit() -> Self::Unit {
        PressureUnit::Pascals
    }

    fn units() -> &'static [Self::Unit] {
        PressureUnit::ALL
    }
}

/// Extension trait for creating Pressure quantities from numeric types.
pub trait PressureConversions {
    /// Creates a Pressure in Pascals.
    fn pascals(self) -> Pressure;
    /// Creates a Pressure in kilopascals.
    fn kilopascals(self) -> Pressure;
    /// Creates a Pressure in bars.
    fn bars(self) -> Pressure;
    /// Creates a Pressure in psi.
    fn psi(self) -> Pressure;
    /// Creates a Pressure in atmospheres.
    fn atmospheres(self) -> Pressure;
}

impl PressureConversions for f64 {
    fn pascals(self) -> Pressure {
        Pressure::pascals(self)
    }
    fn kilopascals(self) -> Pressure {
        Pressure::kilopascals(self)
    }
    fn bars(self) -> Pressure {
        Pressure::bars(self)
    }
    fn psi(self) -> Pressure {
        Pressure::psi(self)
    }
    fn atmospheres(self) -> Pressure {
        Pressure::atmospheres(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_creation() {
        let p = Pressure::pascals(101325.0);
        assert_eq!(p.value(), 101325.0);
        assert_eq!(p.unit(), PressureUnit::Pascals);
    }

    #[test]
    fn test_atmosphere_conversion() {
        let p = Pressure::atmospheres(1.0);
        assert!((p.to_pascals() - 101325.0).abs() < 1.0);
    }

    #[test]
    fn test_bar_conversion() {
        let p = Pressure::bars(1.0);
        assert_eq!(p.to_pascals(), 100000.0);
    }

    #[test]
    fn test_psi_conversion() {
        let p = Pressure::psi(14.696);
        // 14.696 psi ≈ 1 atm ≈ 101325 Pa
        assert!((p.to_pascals() - 101325.0).abs() < 100.0);
    }

    #[test]
    fn test_torr_conversion() {
        let p = Pressure::torr(760.0);
        // 760 Torr = 1 atm
        assert!((p.to_atmospheres() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_pressure_times_area() {
        let p = Pressure::pascals(1000.0);
        let a = Area::square_meters(10.0);
        let f = p * a;
        assert_eq!(f.to_newtons(), 10000.0);
    }

    #[test]
    fn test_force_divided_by_area() {
        let f = Force::newtons(1000.0);
        let a = Area::square_meters(10.0);
        let p = f / a;
        assert_eq!(p.to_pascals(), 100.0);
    }

    #[test]
    fn test_force_divided_by_pressure() {
        let f = Force::newtons(1000.0);
        let p = Pressure::pascals(100.0);
        let a = f / p;
        assert_eq!(a.to_square_meters(), 10.0);
    }
}
