//! Radiance quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of radiance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RadianceUnit {
    /// Watts per steradian per square meter (W/(sr·m²)) - SI unit
    WattsPerSteradianPerSquareMeter,
}

impl RadianceUnit {
    /// All available radiance units.
    pub const ALL: &'static [RadianceUnit] = &[RadianceUnit::WattsPerSteradianPerSquareMeter];
}

impl fmt::Display for RadianceUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for RadianceUnit {
    fn symbol(&self) -> &'static str {
        match self {
            RadianceUnit::WattsPerSteradianPerSquareMeter => "W/(sr·m²)",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            RadianceUnit::WattsPerSteradianPerSquareMeter => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, RadianceUnit::WattsPerSteradianPerSquareMeter)
    }
}

/// A quantity of radiance.
///
/// Radiance represents power per unit solid angle per unit area.
/// SI unit: W/(sr·m²)
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
/// assert_eq!(rad.to_watts_per_steradian_per_square_meter(), 100.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Radiance {
    value: f64,
    unit: RadianceUnit,
}

impl Radiance {
    /// Creates a new Radiance quantity.
    pub const fn new_const(value: f64, unit: RadianceUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a Radiance in watts per steradian per square meter.
    pub fn watts_per_steradian_per_square_meter(value: f64) -> Self {
        Self::new(value, RadianceUnit::WattsPerSteradianPerSquareMeter)
    }

    // Conversion methods
    /// Converts to watts per steradian per square meter.
    pub fn to_watts_per_steradian_per_square_meter(&self) -> f64 {
        self.to(RadianceUnit::WattsPerSteradianPerSquareMeter)
    }
}

impl fmt::Display for Radiance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Radiance {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Radiance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Radiance {
    type Unit = RadianceUnit;

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

impl Add for Radiance {
    type Output = Radiance;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Radiance::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Radiance {
    type Output = Radiance;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Radiance::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Radiance {
    type Output = Radiance;

    fn mul(self, rhs: f64) -> Self::Output {
        Radiance::new(self.value * rhs, self.unit)
    }
}

impl Mul<Radiance> for f64 {
    type Output = Radiance;

    fn mul(self, rhs: Radiance) -> Self::Output {
        Radiance::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for Radiance {
    type Output = Radiance;

    fn div(self, rhs: f64) -> Self::Output {
        Radiance::new(self.value / rhs, self.unit)
    }
}

impl Div<Radiance> for Radiance {
    type Output = f64;

    fn div(self, rhs: Radiance) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Radiance {
    type Output = Radiance;

    fn neg(self) -> Self::Output {
        Radiance::new(-self.value, self.unit)
    }
}

// Cross-quantity operations
use super::radiant_intensity::{RadiantIntensity, RadiantIntensityUnit};
use crate::space::{Area, AreaUnit};

// Radiance * Area = RadiantIntensity
impl Mul<Area> for Radiance {
    type Output = RadiantIntensity;

    fn mul(self, rhs: Area) -> Self::Output {
        let wsr = self.to_watts_per_steradian_per_square_meter() * rhs.to_square_meters();
        RadiantIntensity::new(wsr, RadiantIntensityUnit::WattsPerSteradian)
    }
}

// Area * Radiance = RadiantIntensity
impl Mul<Radiance> for Area {
    type Output = RadiantIntensity;

    fn mul(self, rhs: Radiance) -> Self::Output {
        let wsr = rhs.to_watts_per_steradian_per_square_meter() * self.to_square_meters();
        RadiantIntensity::new(wsr, RadiantIntensityUnit::WattsPerSteradian)
    }
}

// RadiantIntensity / Radiance = Area
impl Div<Radiance> for RadiantIntensity {
    type Output = Area;

    fn div(self, rhs: Radiance) -> Self::Output {
        let m2 = self.to_watts_per_steradian() / rhs.to_watts_per_steradian_per_square_meter();
        Area::new(m2, AreaUnit::SquareMeters)
    }
}

/// Dimension for Radiance.
pub struct RadianceDimension;

impl Dimension for RadianceDimension {
    type Quantity = Radiance;
    type Unit = RadianceUnit;

    fn name() -> &'static str {
        "Radiance"
    }

    fn primary_unit() -> Self::Unit {
        RadianceUnit::WattsPerSteradianPerSquareMeter
    }

    fn si_unit() -> Self::Unit {
        RadianceUnit::WattsPerSteradianPerSquareMeter
    }

    fn units() -> &'static [Self::Unit] {
        RadianceUnit::ALL
    }
}

/// Extension trait for creating Radiance quantities from numeric types.
pub trait RadianceConversions {
    /// Creates a Radiance in watts per steradian per square meter.
    fn watts_per_steradian_per_square_meter(self) -> Radiance;
}

impl RadianceConversions for f64 {
    fn watts_per_steradian_per_square_meter(self) -> Radiance {
        Radiance::watts_per_steradian_per_square_meter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radiance_creation() {
        let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
        assert_eq!(rad.value(), 100.0);
        assert_eq!(rad.unit(), RadianceUnit::WattsPerSteradianPerSquareMeter);
    }

    #[test]
    fn test_radiance_conversions() {
        let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
        assert_eq!(rad.to_watts_per_steradian_per_square_meter(), 100.0);
    }

    #[test]
    fn test_radiance_times_area() {
        let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
        let area = Area::square_meters(2.0);
        let ri = rad * area;
        assert_eq!(ri.to_watts_per_steradian(), 200.0);
    }

    #[test]
    fn test_radiant_intensity_divided_by_radiance() {
        let ri = RadiantIntensity::watts_per_steradian(200.0);
        let rad = Radiance::watts_per_steradian_per_square_meter(100.0);
        let area = ri / rad;
        assert_eq!(area.to_square_meters(), 2.0);
    }
}
