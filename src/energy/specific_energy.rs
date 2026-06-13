//! Specific energy quantity and units.

use crate::core::Quantity;
use crate::mass::Mass;
use std::ops::Mul;
crate::quantity! {
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
    pub quantity SpecificEnergy {
        unit: SpecificEnergyUnit;
        dimension: SpecificEnergyDimension;
        conversions: SpecificEnergyConversions;
        name: "SpecificEnergy";
        primary: Grays;
        si: Grays;

        units {
            /// Grays (Gy) - SI unit (J/kg)
            Grays {
                symbol: "Gy",
                factor: 1.0,
                ctor: grays,
                to: to_grays,
                si: true
            },
            /// Rads (rad) - CGS unit
            Rads {
                symbol: "rad",
                factor: 0.01,
                ctor: rads,
                to: to_rads,
                si: false
            },
            /// Ergs per gram (erg/g)
            ErgsPerGram {
                symbol: "erg/g",
                factor: 0.0001,
                ctor: ergs_per_gram,
                to: to_ergs_per_gram,
                si: false
            }
        }
    }
}
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

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
