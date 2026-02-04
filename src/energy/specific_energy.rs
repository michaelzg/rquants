//! Specific energy quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use crate::mass::Mass;
use std::ops::{Mul};

/// Units of specific energy measurement (energy per unit mass).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecificEnergyUnit {
    /// Grays (Gy) - SI unit (J/kg)
    Grays,
    /// Rads (rad) - CGS unit
    Rads,
    /// Ergs per gram (erg/g)
    ErgsPerGram,
}

impl SpecificEnergyUnit {
    /// All available specific energy units.
    pub const ALL: &'static [SpecificEnergyUnit] = &[
        SpecificEnergyUnit::Grays,
        SpecificEnergyUnit::Rads,
        SpecificEnergyUnit::ErgsPerGram,
    ];
}

impl_unit_display!(SpecificEnergyUnit);

impl UnitOfMeasure for SpecificEnergyUnit {
    fn symbol(&self) -> &'static str {
        match self {
            SpecificEnergyUnit::Grays => "Gy",
            SpecificEnergyUnit::Rads => "rad",
            SpecificEnergyUnit::ErgsPerGram => "erg/g",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            SpecificEnergyUnit::Grays => 1.0,
            SpecificEnergyUnit::Rads => 0.01,     // 1 rad = 0.01 Gy
            SpecificEnergyUnit::ErgsPerGram => 0.0001, // 1 erg/g = 1e-4 Gy
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, SpecificEnergyUnit::Grays)
    }
}

/// A quantity of specific energy (energy per unit mass).
///
/// Specific energy is the energy per unit mass of a substance.
/// Also used for absorbed radiation dose (Gray, Rad).
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let se = SpecificEnergy::grays(10.0);
/// let mass = Mass::kilograms(5.0);
///
/// // Energy = SpecificEnergy * Mass
/// let energy = se * mass;
/// assert!((energy.to_joules() - 50.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SpecificEnergy {
    value: f64,
    unit: SpecificEnergyUnit,
}

impl SpecificEnergy {
    /// Creates a new SpecificEnergy quantity.
    pub const fn new_const(value: f64, unit: SpecificEnergyUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a SpecificEnergy in grays.
    pub fn grays(value: f64) -> Self {
        Self::new(value, SpecificEnergyUnit::Grays)
    }

    /// Creates a SpecificEnergy in rads.
    pub fn rads(value: f64) -> Self {
        Self::new(value, SpecificEnergyUnit::Rads)
    }

    /// Creates a SpecificEnergy in ergs per gram.
    pub fn ergs_per_gram(value: f64) -> Self {
        Self::new(value, SpecificEnergyUnit::ErgsPerGram)
    }

    // Conversion methods
    /// Converts to grays.
    pub fn to_grays(&self) -> f64 {
        self.to(SpecificEnergyUnit::Grays)
    }

    /// Converts to rads.
    pub fn to_rads(&self) -> f64 {
        self.to(SpecificEnergyUnit::Rads)
    }

    /// Converts to ergs per gram.
    pub fn to_ergs_per_gram(&self) -> f64 {
        self.to(SpecificEnergyUnit::ErgsPerGram)
    }
}

impl_quantity!(SpecificEnergy, SpecificEnergyUnit);

// Cross-quantity operations
use super::energy::{Energy, EnergyUnit};

// SpecificEnergy * Mass = Energy
impl Mul<Mass> for SpecificEnergy {
    type Output = Energy;

    fn mul(self, rhs: Mass) -> Self::Output {
        let joules = self.to_grays() * rhs.to_kilograms();
        Energy::new(joules, EnergyUnit::Joules)
    }
}

// Mass * SpecificEnergy = Energy
impl Mul<SpecificEnergy> for Mass {
    type Output = Energy;

    fn mul(self, rhs: SpecificEnergy) -> Self::Output {
        let joules = rhs.to_grays() * self.to_kilograms();
        Energy::new(joules, EnergyUnit::Joules)
    }
}

impl_dimension!(
    SpecificEnergyDimension,
    SpecificEnergy,
    SpecificEnergyUnit,
    "SpecificEnergy",
    SpecificEnergyUnit::Grays,
    SpecificEnergyUnit::Grays
);

/// Extension trait for creating SpecificEnergy quantities from numeric types.
pub trait SpecificEnergyConversions {
    /// Creates a SpecificEnergy in grays.
    fn grays(self) -> SpecificEnergy;
    /// Creates a SpecificEnergy in rads.
    fn rads(self) -> SpecificEnergy;
    /// Creates a SpecificEnergy in ergs per gram.
    fn ergs_per_gram(self) -> SpecificEnergy;
}

impl SpecificEnergyConversions for f64 {
    fn grays(self) -> SpecificEnergy {
        SpecificEnergy::grays(self)
    }
    fn rads(self) -> SpecificEnergy {
        SpecificEnergy::rads(self)
    }
    fn ergs_per_gram(self) -> SpecificEnergy {
        SpecificEnergy::ergs_per_gram(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_energy_creation() {
        let se = SpecificEnergy::grays(10.0);
        assert_eq!(se.value(), 10.0);
        assert_eq!(se.unit(), SpecificEnergyUnit::Grays);
    }

    #[test]
    fn test_rad_conversion() {
        let se = SpecificEnergy::rads(100.0);
        // 100 rad = 1 Gy
        assert!((se.to_grays() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_specific_energy_times_mass() {
        let se = SpecificEnergy::grays(10.0);
        let m = Mass::kilograms(5.0);
        let e = se * m;
        assert!((e.to_joules() - 50.0).abs() < 1e-10);
    }
}
