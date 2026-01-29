//! Molar energy quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use crate::mass::ChemicalAmount;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

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

impl fmt::Display for MolarEnergyUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

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

impl fmt::Display for MolarEnergy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for MolarEnergy {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for MolarEnergy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for MolarEnergy {
    type Unit = MolarEnergyUnit;

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

impl Add for MolarEnergy {
    type Output = MolarEnergy;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        MolarEnergy::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for MolarEnergy {
    type Output = MolarEnergy;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        MolarEnergy::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for MolarEnergy {
    type Output = MolarEnergy;

    fn mul(self, rhs: f64) -> Self::Output {
        MolarEnergy::new(self.value * rhs, self.unit)
    }
}

impl Mul<MolarEnergy> for f64 {
    type Output = MolarEnergy;

    fn mul(self, rhs: MolarEnergy) -> Self::Output {
        MolarEnergy::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for MolarEnergy {
    type Output = MolarEnergy;

    fn div(self, rhs: f64) -> Self::Output {
        MolarEnergy::new(self.value / rhs, self.unit)
    }
}

impl Div<MolarEnergy> for MolarEnergy {
    type Output = f64;

    fn div(self, rhs: MolarEnergy) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for MolarEnergy {
    type Output = MolarEnergy;

    fn neg(self) -> Self::Output {
        MolarEnergy::new(-self.value, self.unit)
    }
}

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

/// Dimension for MolarEnergy.
pub struct MolarEnergyDimension;

impl Dimension for MolarEnergyDimension {
    type Quantity = MolarEnergy;
    type Unit = MolarEnergyUnit;

    fn name() -> &'static str {
        "MolarEnergy"
    }

    fn primary_unit() -> Self::Unit {
        MolarEnergyUnit::JoulesPerMole
    }

    fn si_unit() -> Self::Unit {
        MolarEnergyUnit::JoulesPerMole
    }

    fn units() -> &'static [Self::Unit] {
        MolarEnergyUnit::ALL
    }
}

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
