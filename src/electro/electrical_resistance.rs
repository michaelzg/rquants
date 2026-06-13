//! Electrical resistance quantity and units.
use crate::core::Quantity;
use std::ops::Mul;
crate::quantity! {
    /// A quantity of electrical resistance.
    ///
    /// Electrical resistance is a measure of the opposition to current flow in an electrical circuit.
    /// R = V / I (Ohm's law)
    ///
    /// # Relationships
    ///
    /// - Resistance × Current = Potential (V = IR)
    /// - Resistance = 1 / Conductance
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let resistance = ElectricalResistance::ohms(100.0);
    /// let current = ElectricCurrent::amperes(0.5);
    ///
    /// // Voltage = Resistance × Current
    /// let voltage = resistance * current;
    /// assert!((voltage.to_volts() - 50.0).abs() < 1e-10);
    /// ```
    pub quantity ElectricalResistance {
        unit: ElectricalResistanceUnit;
        dimension: ElectricalResistanceDimension;
        conversions: ElectricalResistanceConversions;
        name: "ElectricalResistance";
        primary: Ohms;
        si: Ohms;

        units {
            /// Ohms (Ω) - SI unit
            Ohms {
                symbol: "Ω",
                factor: 1.0,
                ctor: ohms,
                to: to_ohms,
                si: true
            },
            /// Milliohms (mΩ)
            Milliohms {
                symbol: "mΩ",
                factor: 1e-3,
                ctor: milliohms,
                to: to_milliohms,
                si: true
            },
            /// Kilohms (kΩ)
            Kilohms {
                symbol: "kΩ",
                factor: 1e3,
                ctor: kilohms,
                to: to_kilohms,
                si: true
            },
            /// Megohms (MΩ)
            Megohms {
                symbol: "MΩ",
                factor: 1e6,
                ctor: megohms,
                to: to_megohms,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use crate::space::Length;

// Resistance * Length = Resistivity
use super::resistivity::{Resistivity, ResistivityUnit};

impl Mul<Length> for ElectricalResistance {
    type Output = Resistivity;

    fn mul(self, rhs: Length) -> Self::Output {
        let ohm_meters = self.to_ohms() * rhs.to_meters();
        Resistivity::new(ohm_meters, ResistivityUnit::OhmMeters)
    }
}

// Length * Resistance = Resistivity
impl Mul<ElectricalResistance> for Length {
    type Output = Resistivity;

    fn mul(self, rhs: ElectricalResistance) -> Self::Output {
        let ohm_meters = self.to_meters() * rhs.to_ohms();
        Resistivity::new(ohm_meters, ResistivityUnit::OhmMeters)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_resistance_creation() {
        let r = ElectricalResistance::ohms(100.0);
        assert_eq!(r.value(), 100.0);
        assert_eq!(r.unit(), ElectricalResistanceUnit::Ohms);
    }

    #[test]
    fn test_resistance_conversions() {
        let r = ElectricalResistance::kilohms(1.0);
        assert_eq!(r.to_ohms(), 1000.0);

        let r2 = ElectricalResistance::milliohms(2000.0);
        assert_eq!(r2.to_ohms(), 2.0);
    }

    #[test]
    fn test_resistance_arithmetic() {
        let r1 = ElectricalResistance::ohms(100.0);
        let r2 = ElectricalResistance::ohms(50.0);
        let sum = r1 + r2;
        assert_eq!(sum.to_ohms(), 150.0);
    }
}
