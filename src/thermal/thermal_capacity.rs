//! Thermal capacity (entropy) quantity and units.
use crate::core::Quantity;
use std::ops::Mul;
crate::quantity! {
    /// A quantity of thermal capacity (also represents entropy).
    ///
    /// Thermal capacity represents the ability of a substance to store thermal energy
    /// per unit of temperature change.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::thermal::thermal_capacity::{ThermalCapacity, ThermalCapacityUnit};
    /// use rquants::thermal::temperature::Temperature;
    /// use rquants::core::Quantity;
    ///
    /// let tc = ThermalCapacity::joules_per_kelvin(4186.0); // water, ~1 kg
    /// let temp = Temperature::kelvin(300.0);
    ///
    /// // Energy = ThermalCapacity * Temperature
    /// let energy = tc * temp;
    /// assert!((energy.to_joules() - 1_255_800.0).abs() < 1.0);
    /// ```
    pub quantity ThermalCapacity {
        unit: ThermalCapacityUnit;
        dimension: ThermalCapacityDimension;
        conversions: ThermalCapacityConversions;
        name: "ThermalCapacity";
        primary: JoulesPerKelvin;
        si: JoulesPerKelvin;

        units {
            /// Joules per kelvin (J/K) - SI unit
            JoulesPerKelvin {
                symbol: "J/K",
                factor: 1.0,
                ctor: joules_per_kelvin,
                to: to_joules_per_kelvin,
                si: false
            }
        }
    }
}
// Cross-quantity: ThermalCapacity * Temperature = Energy
use super::temperature::Temperature;
use crate::energy::{Energy, EnergyUnit};

impl Mul<Temperature> for ThermalCapacity {
    type Output = Energy;

    fn mul(self, rhs: Temperature) -> Self::Output {
        let joules = self.to_joules_per_kelvin() * rhs.to_kelvin_scale();
        Energy::new(joules, EnergyUnit::Joules)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_thermal_capacity_creation() {
        let tc = ThermalCapacity::joules_per_kelvin(100.0);
        assert_eq!(tc.value(), 100.0);
        assert_eq!(tc.unit(), ThermalCapacityUnit::JoulesPerKelvin);
    }

    #[test]
    fn test_thermal_capacity_times_temperature() {
        let tc = ThermalCapacity::joules_per_kelvin(100.0);
        let t = Temperature::kelvin(300.0);
        let e = tc * t;
        assert!((e.to_joules() - 30000.0).abs() < 1e-10);
    }

    #[test]
    fn test_temperature_times_thermal_capacity() {
        let t = Temperature::kelvin(300.0);
        let tc = ThermalCapacity::joules_per_kelvin(100.0);
        let e = t * tc;
        assert!((e.to_joules() - 30000.0).abs() < 1e-10);
    }
}
