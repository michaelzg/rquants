//! Particle flux quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of particle flux measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParticleFluxUnit {
    /// Becquerels per square meter per second (Bq/(m²·s)) - SI unit
    BecquerelsPerSquareMeterSecond,
}

impl ParticleFluxUnit {
    /// All available particle flux units.
    pub const ALL: &'static [ParticleFluxUnit] =
        &[ParticleFluxUnit::BecquerelsPerSquareMeterSecond];
}

impl fmt::Display for ParticleFluxUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for ParticleFluxUnit {
    fn symbol(&self) -> &'static str {
        match self {
            ParticleFluxUnit::BecquerelsPerSquareMeterSecond => "Bq/(m²·s)",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            ParticleFluxUnit::BecquerelsPerSquareMeterSecond => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, ParticleFluxUnit::BecquerelsPerSquareMeterSecond)
    }
}

/// A quantity of particle flux.
///
/// Particle flux represents the number of particles crossing a unit area per unit time.
/// SI unit: Bq/(m²·s) = particles/(m²·s)
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let flux = ParticleFlux::becquerels_per_square_meter_second(1000.0);
/// assert_eq!(flux.to_becquerels_per_square_meter_second(), 1000.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ParticleFlux {
    value: f64,
    unit: ParticleFluxUnit,
}

impl ParticleFlux {
    /// Creates a new ParticleFlux quantity.
    pub const fn new_const(value: f64, unit: ParticleFluxUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a ParticleFlux in becquerels per square meter per second.
    pub fn becquerels_per_square_meter_second(value: f64) -> Self {
        Self::new(value, ParticleFluxUnit::BecquerelsPerSquareMeterSecond)
    }

    // Conversion methods
    /// Converts to becquerels per square meter per second.
    pub fn to_becquerels_per_square_meter_second(&self) -> f64 {
        self.to(ParticleFluxUnit::BecquerelsPerSquareMeterSecond)
    }
}

impl fmt::Display for ParticleFlux {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for ParticleFlux {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for ParticleFlux {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for ParticleFlux {
    type Unit = ParticleFluxUnit;

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

impl Add for ParticleFlux {
    type Output = ParticleFlux;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        ParticleFlux::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for ParticleFlux {
    type Output = ParticleFlux;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        ParticleFlux::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for ParticleFlux {
    type Output = ParticleFlux;

    fn mul(self, rhs: f64) -> Self::Output {
        ParticleFlux::new(self.value * rhs, self.unit)
    }
}

impl Mul<ParticleFlux> for f64 {
    type Output = ParticleFlux;

    fn mul(self, rhs: ParticleFlux) -> Self::Output {
        ParticleFlux::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for ParticleFlux {
    type Output = ParticleFlux;

    fn div(self, rhs: f64) -> Self::Output {
        ParticleFlux::new(self.value / rhs, self.unit)
    }
}

impl Div<ParticleFlux> for ParticleFlux {
    type Output = f64;

    fn div(self, rhs: ParticleFlux) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for ParticleFlux {
    type Output = ParticleFlux;

    fn neg(self) -> Self::Output {
        ParticleFlux::new(-self.value, self.unit)
    }
}

/// Dimension for ParticleFlux.
pub struct ParticleFluxDimension;

impl Dimension for ParticleFluxDimension {
    type Quantity = ParticleFlux;
    type Unit = ParticleFluxUnit;

    fn name() -> &'static str {
        "ParticleFlux"
    }

    fn primary_unit() -> Self::Unit {
        ParticleFluxUnit::BecquerelsPerSquareMeterSecond
    }

    fn si_unit() -> Self::Unit {
        ParticleFluxUnit::BecquerelsPerSquareMeterSecond
    }

    fn units() -> &'static [Self::Unit] {
        ParticleFluxUnit::ALL
    }
}

/// Extension trait for creating ParticleFlux quantities from numeric types.
pub trait ParticleFluxConversions {
    /// Creates a ParticleFlux in becquerels per square meter per second.
    fn becquerels_per_square_meter_second(self) -> ParticleFlux;
}

impl ParticleFluxConversions for f64 {
    fn becquerels_per_square_meter_second(self) -> ParticleFlux {
        ParticleFlux::becquerels_per_square_meter_second(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_flux_creation() {
        let pf = ParticleFlux::becquerels_per_square_meter_second(1000.0);
        assert_eq!(pf.value(), 1000.0);
        assert_eq!(
            pf.unit(),
            ParticleFluxUnit::BecquerelsPerSquareMeterSecond
        );
    }

    #[test]
    fn test_particle_flux_conversions() {
        let pf = ParticleFlux::becquerels_per_square_meter_second(1000.0);
        assert_eq!(pf.to_becquerels_per_square_meter_second(), 1000.0);
    }

    #[test]
    fn test_particle_flux_arithmetic() {
        let pf1 = ParticleFlux::becquerels_per_square_meter_second(1000.0);
        let pf2 = ParticleFlux::becquerels_per_square_meter_second(500.0);
        let sum = pf1 + pf2;
        assert_eq!(sum.to_becquerels_per_square_meter_second(), 1500.0);
    }
}
