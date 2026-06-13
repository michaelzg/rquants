//! Electric current quantity and units.
use crate::core::Quantity;
use std::ops::Mul;
crate::quantity! {
    /// A quantity of electric current.
    ///
    /// Electric current is the flow of electric charge through a conductor.
    /// I = Q / t (current = charge / time)
    ///
    /// # Relationships
    ///
    /// - Current × Time = Charge (Q = It)
    /// - Current × Resistance = Potential (V = IR, Ohm's law)
    /// - Current × Potential = Power (P = IV)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let current = ElectricCurrent::amperes(2.0);
    /// let resistance = ElectricalResistance::ohms(5.0);
    ///
    /// // Ohm's law: V = IR
    /// let voltage = current * resistance;
    /// assert!((voltage.to_volts() - 10.0).abs() < 1e-10);
    /// ```
    pub quantity ElectricCurrent {
        unit: ElectricCurrentUnit;
        dimension: ElectricCurrentDimension;
        conversions: ElectricCurrentConversions;
        name: "ElectricCurrent";
        primary: Amperes;
        si: Amperes;

        units {
            /// Amperes (A) - SI unit
            Amperes {
                symbol: "A",
                factor: 1.0,
                ctor: amperes,
                to: to_amperes,
                si: true
            },
            /// Milliamperes (mA)
            Milliamperes {
                symbol: "mA",
                factor: 1e-3,
                ctor: milliamperes,
                to: to_milliamperes,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use super::electric_charge::{ElectricCharge, ElectricChargeUnit};
use super::electric_potential::{ElectricPotential, ElectricPotentialUnit};
use super::electrical_resistance::ElectricalResistance;
use crate::energy::{Power, PowerUnit};
use crate::time::Time;

// Current * Time = Charge
impl Mul<Time> for ElectricCurrent {
    type Output = ElectricCharge;

    fn mul(self, rhs: Time) -> Self::Output {
        let coulombs = self.to_amperes() * rhs.to_seconds();
        ElectricCharge::new(coulombs, ElectricChargeUnit::Coulombs)
    }
}

// Time * Current = Charge
impl Mul<ElectricCurrent> for Time {
    type Output = ElectricCharge;

    fn mul(self, rhs: ElectricCurrent) -> Self::Output {
        let coulombs = rhs.to_amperes() * self.to_seconds();
        ElectricCharge::new(coulombs, ElectricChargeUnit::Coulombs)
    }
}

// Current * Resistance = Potential (Ohm's law: V = IR)
impl Mul<ElectricalResistance> for ElectricCurrent {
    type Output = ElectricPotential;

    fn mul(self, rhs: ElectricalResistance) -> Self::Output {
        let volts = self.to_amperes() * rhs.to_ohms();
        ElectricPotential::new(volts, ElectricPotentialUnit::Volts)
    }
}

// Resistance * Current = Potential
impl Mul<ElectricCurrent> for ElectricalResistance {
    type Output = ElectricPotential;

    fn mul(self, rhs: ElectricCurrent) -> Self::Output {
        let volts = self.to_ohms() * rhs.to_amperes();
        ElectricPotential::new(volts, ElectricPotentialUnit::Volts)
    }
}

// Current * Potential = Power (P = IV)
impl Mul<ElectricPotential> for ElectricCurrent {
    type Output = Power;

    fn mul(self, rhs: ElectricPotential) -> Self::Output {
        let watts = self.to_amperes() * rhs.to_volts();
        Power::new(watts, PowerUnit::Watts)
    }
}

// Potential * Current = Power
impl Mul<ElectricCurrent> for ElectricPotential {
    type Output = Power;

    fn mul(self, rhs: ElectricCurrent) -> Self::Output {
        let watts = self.to_volts() * rhs.to_amperes();
        Power::new(watts, PowerUnit::Watts)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_current_creation() {
        let i = ElectricCurrent::amperes(5.0);
        assert_eq!(i.value(), 5.0);
        assert_eq!(i.unit(), ElectricCurrentUnit::Amperes);
    }

    #[test]
    fn test_current_conversions() {
        let i = ElectricCurrent::amperes(1.0);
        assert_eq!(i.to_milliamperes(), 1000.0);

        let i2 = ElectricCurrent::milliamperes(500.0);
        assert_eq!(i2.to_amperes(), 0.5);
    }

    #[test]
    fn test_current_times_time() {
        let i = ElectricCurrent::amperes(2.0);
        let t = Time::seconds(10.0);
        let q = i * t;
        assert_eq!(q.to_coulombs(), 20.0);
    }

    #[test]
    fn test_ohms_law() {
        let i = ElectricCurrent::amperes(2.0);
        let r = ElectricalResistance::ohms(5.0);
        let v = i * r;
        assert_eq!(v.to_volts(), 10.0);
    }

    #[test]
    fn test_power_law() {
        let i = ElectricCurrent::amperes(2.0);
        let v = ElectricPotential::volts(10.0);
        let p = i * v;
        assert_eq!(p.to_watts(), 20.0);
    }
}
