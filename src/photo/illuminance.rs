//! Illuminance quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of illuminance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IlluminanceUnit {
    /// Lux (lx) - SI unit (lm/m²)
    Lux,
}

impl IlluminanceUnit {
    /// All available illuminance units.
    pub const ALL: &'static [IlluminanceUnit] = &[IlluminanceUnit::Lux];
}

impl fmt::Display for IlluminanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for IlluminanceUnit {
    fn symbol(&self) -> &'static str {
        match self {
            IlluminanceUnit::Lux => "lx",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            IlluminanceUnit::Lux => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, IlluminanceUnit::Lux)
    }
}

/// A quantity of illuminance.
///
/// Illuminance represents the luminous flux incident on a surface per unit area.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let illuminance = Illuminance::lux(500.0);
/// let area = Area::square_meters(4.0);
///
/// // Illuminance * Area = LuminousFlux
/// let flux = illuminance * area;
/// assert!((flux.to_lumens() - 2000.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Illuminance {
    value: f64,
    unit: IlluminanceUnit,
}

impl Illuminance {
    /// Creates a new Illuminance quantity.
    pub const fn new_const(value: f64, unit: IlluminanceUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates an Illuminance in lux.
    pub fn lux(value: f64) -> Self {
        Self::new(value, IlluminanceUnit::Lux)
    }

    // Conversion methods
    /// Converts to lux.
    pub fn to_lux(&self) -> f64 {
        self.to(IlluminanceUnit::Lux)
    }
}

impl fmt::Display for Illuminance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Illuminance {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Illuminance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Illuminance {
    type Unit = IlluminanceUnit;

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

impl Add for Illuminance {
    type Output = Illuminance;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Illuminance::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Illuminance {
    type Output = Illuminance;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Illuminance::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Illuminance {
    type Output = Illuminance;

    fn mul(self, rhs: f64) -> Self::Output {
        Illuminance::new(self.value * rhs, self.unit)
    }
}

impl Mul<Illuminance> for f64 {
    type Output = Illuminance;

    fn mul(self, rhs: Illuminance) -> Self::Output {
        Illuminance::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for Illuminance {
    type Output = Illuminance;

    fn div(self, rhs: f64) -> Self::Output {
        Illuminance::new(self.value / rhs, self.unit)
    }
}

impl Div<Illuminance> for Illuminance {
    type Output = f64;

    fn div(self, rhs: Illuminance) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Illuminance {
    type Output = Illuminance;

    fn neg(self) -> Self::Output {
        Illuminance::new(-self.value, self.unit)
    }
}

// Cross-quantity operations
use super::luminous_exposure::{LuminousExposure, LuminousExposureUnit};
use super::luminous_flux::{LuminousFlux, LuminousFluxUnit};
use crate::space::Area;
use crate::time::Time;

// Illuminance * Area = LuminousFlux
impl Mul<Area> for Illuminance {
    type Output = LuminousFlux;

    fn mul(self, rhs: Area) -> Self::Output {
        let lumens = self.to_lux() * rhs.to_square_meters();
        LuminousFlux::new(lumens, LuminousFluxUnit::Lumens)
    }
}

// Area * Illuminance = LuminousFlux
impl Mul<Illuminance> for Area {
    type Output = LuminousFlux;

    fn mul(self, rhs: Illuminance) -> Self::Output {
        let lumens = rhs.to_lux() * self.to_square_meters();
        LuminousFlux::new(lumens, LuminousFluxUnit::Lumens)
    }
}

// Illuminance * Time = LuminousExposure
impl Mul<Time> for Illuminance {
    type Output = LuminousExposure;

    fn mul(self, rhs: Time) -> Self::Output {
        let lx_s = self.to_lux() * rhs.to_seconds();
        LuminousExposure::new(lx_s, LuminousExposureUnit::LuxSeconds)
    }
}

// Time * Illuminance = LuminousExposure
impl Mul<Illuminance> for Time {
    type Output = LuminousExposure;

    fn mul(self, rhs: Illuminance) -> Self::Output {
        let lx_s = rhs.to_lux() * self.to_seconds();
        LuminousExposure::new(lx_s, LuminousExposureUnit::LuxSeconds)
    }
}

/// Dimension for Illuminance.
pub struct IlluminanceDimension;

impl Dimension for IlluminanceDimension {
    type Quantity = Illuminance;
    type Unit = IlluminanceUnit;

    fn name() -> &'static str {
        "Illuminance"
    }

    fn primary_unit() -> Self::Unit {
        IlluminanceUnit::Lux
    }

    fn si_unit() -> Self::Unit {
        IlluminanceUnit::Lux
    }

    fn units() -> &'static [Self::Unit] {
        IlluminanceUnit::ALL
    }
}

/// Extension trait for creating Illuminance quantities from numeric types.
pub trait IlluminanceConversions {
    /// Creates an Illuminance in lux.
    fn lux(self) -> Illuminance;
}

impl IlluminanceConversions for f64 {
    fn lux(self) -> Illuminance {
        Illuminance::lux(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_illuminance_creation() {
        let i = Illuminance::lux(500.0);
        assert_eq!(i.value(), 500.0);
        assert_eq!(i.unit(), IlluminanceUnit::Lux);
    }

    #[test]
    fn test_illuminance_times_area() {
        let i = Illuminance::lux(100.0);
        let a = Area::square_meters(5.0);
        let f = i * a;
        // 100 lx * 5 m² = 500 lm
        assert!((f.to_lumens() - 500.0).abs() < 1e-10);
    }

    #[test]
    fn test_illuminance_times_time() {
        let i = Illuminance::lux(200.0);
        let t = Time::seconds(3.0);
        let e = i * t;
        // 200 lx * 3 s = 600 lx·s
        assert!((e.to_lux_seconds() - 600.0).abs() < 1e-10);
    }
}
