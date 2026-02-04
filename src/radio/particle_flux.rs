//! Particle flux quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};

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

impl_unit_display!(ParticleFluxUnit);

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

impl_quantity!(ParticleFlux, ParticleFluxUnit);

impl_dimension!(
    ParticleFluxDimension,
    ParticleFlux,
    ParticleFluxUnit,
    "ParticleFlux",
    ParticleFluxUnit::BecquerelsPerSquareMeterSecond,
    ParticleFluxUnit::BecquerelsPerSquareMeterSecond
);

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
