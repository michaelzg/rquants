//! Molar energy quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use crate::mass::ChemicalAmount;
use std::ops::{Mul};

/// Units of molar energy measurement (energy per chemical amount).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MolarEnergyUnit {
    /// Joules per mole (J/mol) - SI unit
    JoulesPerMole,
    /// Kilojoules per mole (kJ/mol)
    KilojoulesPerMole,
}

impl MolarEnergyUnit {
    /// All available molar energy units.
    pub const ALL: &'static [MolarEnergyUnit] = &[
        MolarEnergyUnit::JoulesPerMole,
        MolarEnergyUnit::KilojoulesPerMole,
    ];
}

impl_unit_display!(MolarEnergyUnit);

impl UnitOfMeasure for MolarEnergyUnit {
    fn symbol(&self) -> &'static str {
        match self {
            MolarEnergyUnit::JoulesPerMole => "J/mol",
            MolarEnergyUnit::KilojoulesPerMole => "kJ/mol",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            MolarEnergyUnit::JoulesPerMole => 1.0,
            MolarEnergyUnit::KilojoulesPerMole => 1e3,
        }
    }

    fn is_si(&self) -> bool {
        true
    }
}

/// A quantity of molar energy (energy per chemical amount).
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let me = MolarEnergy::joules_per_mole(500.0);
/// let amount = ChemicalAmount::moles(2.0);
///
/// // Energy = MolarEnergy * ChemicalAmount
/// let energy = me * amount;
/// assert!((energy.to_joules() - 1000.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct MolarEnergy {
    value: f64,
    unit: MolarEnergyUnit,
}

impl MolarEnergy {
    /// Creates a new MolarEnergy quantity.
    pub const fn new_const(value: f64, unit: MolarEnergyUnit) -> Self {
        Self { value, unit }
    }

    /// Creates a MolarEnergy in joules per mole.
    pub fn joules_per_mole(value: f64) -> Self {
        Self::new(value, MolarEnergyUnit::JoulesPerMole)
    }

    /// Creates a MolarEnergy in kilojoules per mole.
    pub fn kilojoules_per_mole(value: f64) -> Self {
        Self::new(value, MolarEnergyUnit::KilojoulesPerMole)
    }

    /// Converts to joules per mole.
    pub fn to_joules_per_mole(&self) -> f64 {
        self.to(MolarEnergyUnit::JoulesPerMole)
    }

    /// Converts to kilojoules per mole.
    pub fn to_kilojoules_per_mole(&self) -> f64 {
        self.to(MolarEnergyUnit::KilojoulesPerMole)
    }
}

impl_quantity!(MolarEnergy, MolarEnergyUnit);

// Cross-quantity operations
use super::energy::{Energy, EnergyUnit};

// MolarEnergy * ChemicalAmount = Energy
impl Mul<ChemicalAmount> for MolarEnergy {
    type Output = Energy;

    fn mul(self, rhs: ChemicalAmount) -> Self::Output {
        let joules = self.to_joules_per_mole() * rhs.to_moles();
        Energy::new(joules, EnergyUnit::Joules)
    }
}

// ChemicalAmount * MolarEnergy = Energy
impl Mul<MolarEnergy> for ChemicalAmount {
    type Output = Energy;

    fn mul(self, rhs: MolarEnergy) -> Self::Output {
        let joules = rhs.to_joules_per_mole() * self.to_moles();
        Energy::new(joules, EnergyUnit::Joules)
    }
}

impl_dimension!(
    MolarEnergyDimension,
    MolarEnergy,
    MolarEnergyUnit,
    "MolarEnergy",
    MolarEnergyUnit::JoulesPerMole,
    MolarEnergyUnit::JoulesPerMole
);

/// Extension trait for creating MolarEnergy quantities from numeric types.
pub trait MolarEnergyConversions {
    /// Creates a MolarEnergy in joules per mole.
    fn joules_per_mole(self) -> MolarEnergy;
    /// Creates a MolarEnergy in kilojoules per mole.
    fn kilojoules_per_mole(self) -> MolarEnergy;
}

impl MolarEnergyConversions for f64 {
    fn joules_per_mole(self) -> MolarEnergy {
        MolarEnergy::joules_per_mole(self)
    }
    fn kilojoules_per_mole(self) -> MolarEnergy {
        MolarEnergy::kilojoules_per_mole(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_molar_energy_creation() {
        let me = MolarEnergy::joules_per_mole(500.0);
        assert_eq!(me.value(), 500.0);
        assert_eq!(me.unit(), MolarEnergyUnit::JoulesPerMole);
    }

    #[test]
    fn test_kj_per_mol_conversion() {
        let me = MolarEnergy::kilojoules_per_mole(1.0);
        assert!((me.to_joules_per_mole() - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_molar_energy_times_amount() {
        let me = MolarEnergy::joules_per_mole(500.0);
        let n = ChemicalAmount::moles(2.0);
        let e = me * n;
        assert!((e.to_joules() - 1000.0).abs() < 1e-10);
    }
}
