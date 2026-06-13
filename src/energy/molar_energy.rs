//! Molar energy quantity and units.

use crate::core::Quantity;
use crate::mass::ChemicalAmount;
use std::ops::Mul;
crate::quantity! {
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
    pub quantity MolarEnergy {
        unit: MolarEnergyUnit;
        dimension: MolarEnergyDimension;
        conversions: MolarEnergyConversions;
        name: "MolarEnergy";
        primary: JoulesPerMole;
        si: JoulesPerMole;

        units {
            /// Joules per mole (J/mol) - SI unit
            JoulesPerMole {
                symbol: "J/mol",
                factor: 1.0,
                ctor: joules_per_mole,
                to: to_joules_per_mole,
                si: false
            },
            /// Kilojoules per mole (kJ/mol)
            KilojoulesPerMole {
                symbol: "kJ/mol",
                factor: 1e3,
                ctor: kilojoules_per_mole,
                to: to_kilojoules_per_mole,
                si: false
            }
        }
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

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
