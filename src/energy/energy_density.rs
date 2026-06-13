//! Energy density quantity and units.
use crate::core::Quantity;
use std::ops::Mul;
crate::quantity! {
    /// A quantity of energy density (energy per unit volume).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let ed = EnergyDensity::joules_per_cubic_meter(1000.0);
    /// let volume = Volume::cubic_meters(2.0);
    ///
    /// // Energy = EnergyDensity * Volume
    /// let energy = ed * volume;
    /// assert!((energy.to_joules() - 2000.0).abs() < 1e-10);
    /// ```
    pub quantity EnergyDensity {
        unit: EnergyDensityUnit;
        dimension: EnergyDensityDimension;
        conversions: EnergyDensityConversions;
        name: "EnergyDensity";
        primary: JoulesPerCubicMeter;
        si: JoulesPerCubicMeter;

        units {
            /// Joules per cubic meter (J/m³) - SI unit
            JoulesPerCubicMeter {
                symbol: "J/m³",
                factor: 1.0,
                ctor: joules_per_cubic_meter,
                to: to_joules_per_cubic_meter,
                si: false
            }
        }
    }
}
// Cross-quantity operations
use super::energy::{Energy, EnergyUnit};
use crate::space::Volume;

// EnergyDensity * Volume = Energy
impl Mul<Volume> for EnergyDensity {
    type Output = Energy;

    fn mul(self, rhs: Volume) -> Self::Output {
        let joules = self.to_joules_per_cubic_meter() * rhs.to_cubic_meters();
        Energy::new(joules, EnergyUnit::Joules)
    }
}

// Volume * EnergyDensity = Energy
impl Mul<EnergyDensity> for Volume {
    type Output = Energy;

    fn mul(self, rhs: EnergyDensity) -> Self::Output {
        let joules = rhs.to_joules_per_cubic_meter() * self.to_cubic_meters();
        Energy::new(joules, EnergyUnit::Joules)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_energy_density_creation() {
        let ed = EnergyDensity::joules_per_cubic_meter(1000.0);
        assert_eq!(ed.value(), 1000.0);
        assert_eq!(ed.unit(), EnergyDensityUnit::JoulesPerCubicMeter);
    }

    #[test]
    fn test_energy_density_times_volume() {
        let ed = EnergyDensity::joules_per_cubic_meter(500.0);
        let v = Volume::cubic_meters(4.0);
        let e = ed * v;
        assert!((e.to_joules() - 2000.0).abs() < 1e-10);
    }
}
