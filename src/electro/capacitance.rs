//! Capacitance quantity and units.
use crate::core::Quantity;
use std::ops::Mul;
crate::quantity! {
    /// A quantity of capacitance.
    ///
    /// Capacitance is the ability of a component or circuit to store electrical charge.
    /// C = Q / V (capacitance = charge / voltage)
    ///
    /// # Relationships
    ///
    /// - Capacitance × Potential = Charge (Q = CV)
    /// - Capacitance = Charge / Potential (C = Q/V)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let capacitance = Capacitance::microfarads(100.0);
    /// let voltage = ElectricPotential::volts(10.0);
    ///
    /// // Charge = Capacitance × Voltage
    /// let charge = capacitance * voltage;
    /// assert!((charge.to_coulombs() - 0.001).abs() < 1e-10);
    /// ```
    pub quantity Capacitance {
        unit: CapacitanceUnit;
        dimension: CapacitanceDimension;
        conversions: CapacitanceConversions;
        name: "Capacitance";
        primary: Farads;
        si: Farads;

        units {
            /// Farads (F) - SI unit
            Farads {
                symbol: "F",
                factor: 1.0,
                ctor: farads,
                to: to_farads,
                si: true
            },
            /// Picofarads (pF)
            Picofarads {
                symbol: "pF",
                factor: 1e-12,
                ctor: picofarads,
                to: to_picofarads,
                si: true
            },
            /// Nanofarads (nF)
            Nanofarads {
                symbol: "nF",
                factor: 1e-9,
                ctor: nanofarads,
                to: to_nanofarads,
                si: true
            },
            /// Microfarads (µF)
            Microfarads {
                symbol: "µF",
                factor: 1e-6,
                ctor: microfarads,
                to: to_microfarads,
                si: true
            },
            /// Millifarads (mF)
            Millifarads {
                symbol: "mF",
                factor: 1e-3,
                ctor: millifarads,
                to: to_millifarads,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use super::electric_charge::{ElectricCharge, ElectricChargeUnit};
use super::electric_potential::ElectricPotential;

// Capacitance * Potential = Charge (Q = CV)
impl Mul<ElectricPotential> for Capacitance {
    type Output = ElectricCharge;

    fn mul(self, rhs: ElectricPotential) -> Self::Output {
        let coulombs = self.to_farads() * rhs.to_volts();
        ElectricCharge::new(coulombs, ElectricChargeUnit::Coulombs)
    }
}

// Potential * Capacitance = Charge
impl Mul<Capacitance> for ElectricPotential {
    type Output = ElectricCharge;

    fn mul(self, rhs: Capacitance) -> Self::Output {
        let coulombs = self.to_volts() * rhs.to_farads();
        ElectricCharge::new(coulombs, ElectricChargeUnit::Coulombs)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_capacitance_creation() {
        let c = Capacitance::farads(1.0);
        assert_eq!(c.value(), 1.0);
        assert_eq!(c.unit(), CapacitanceUnit::Farads);
    }

    #[test]
    fn test_capacitance_conversions() {
        let c = Capacitance::microfarads(1.0);
        assert_eq!(c.to_farads(), 1e-6);

        let c2 = Capacitance::nanofarads(1000.0);
        assert!((c2.to_microfarads() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_capacitance_times_voltage() {
        let c = Capacitance::microfarads(100.0);
        let v = ElectricPotential::volts(10.0);
        let q = c * v;
        // 100 µF * 10 V = 0.001 C
        assert!((q.to_coulombs() - 0.001).abs() < 1e-10);
    }
}
