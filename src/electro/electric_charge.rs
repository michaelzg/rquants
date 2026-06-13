//! Electric charge quantity and units.
use crate::core::Quantity;
use std::ops::{Div, Mul};
crate::quantity! {
    /// A quantity of electric charge.
    ///
    /// Electric charge is a fundamental property of matter that causes it to experience
    /// a force when placed in an electromagnetic field.
    /// Q = I × t (charge = current × time)
    ///
    /// # Relationships
    ///
    /// - Charge / Time = Current (I = Q/t)
    /// - Charge / Capacitance = Potential (V = Q/C)
    /// - Charge × Potential = Energy (E = QV)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let charge = ElectricCharge::coulombs(10.0);
    /// let time = Time::seconds(2.0);
    ///
    /// // Current = Charge / Time
    /// let current = charge / time;
    /// assert!((current.to_amperes() - 5.0).abs() < 1e-10);
    /// ```
    pub quantity ElectricCharge {
        unit: ElectricChargeUnit;
        dimension: ElectricChargeDimension;
        conversions: ElectricChargeConversions;
        name: "ElectricCharge";
        primary: Coulombs;
        si: Coulombs;

        units {
            /// Coulombs (C) - SI unit
            Coulombs {
                symbol: "C",
                factor: 1.0,
                ctor: coulombs,
                to: to_coulombs,
                si: true
            },
            /// Milliampere-hours (mAh)
            Milliamperehours {
                symbol: "mAh",
                factor: 3.6,
                ctor: milliamperehours,
                to: to_milliamperehours,
                si: false
            },
            /// Ampere-hours (Ah)
            Amperehours {
                symbol: "Ah",
                factor: 3600.0,
                ctor: amperehours,
                to: to_amperehours,
                si: false
            }
        }
    }
}
// Cross-quantity operations
use super::capacitance::{Capacitance, CapacitanceUnit};
use super::electric_current::{ElectricCurrent, ElectricCurrentUnit};
use super::electric_potential::{ElectricPotential, ElectricPotentialUnit};
use crate::energy::{Energy, EnergyUnit};
use crate::time::{Time, TimeUnit};

// Charge / Time = Current
impl Div<Time> for ElectricCharge {
    type Output = ElectricCurrent;

    fn div(self, rhs: Time) -> Self::Output {
        let amperes = self.to_coulombs() / rhs.to_seconds();
        ElectricCurrent::new(amperes, ElectricCurrentUnit::Amperes)
    }
}

// Charge / Current = Time
impl Div<ElectricCurrent> for ElectricCharge {
    type Output = Time;

    fn div(self, rhs: ElectricCurrent) -> Self::Output {
        let seconds = self.to_coulombs() / rhs.to_amperes();
        Time::new(seconds, TimeUnit::Seconds)
    }
}

// Charge / Capacitance = Potential (V = Q/C)
impl Div<Capacitance> for ElectricCharge {
    type Output = ElectricPotential;

    fn div(self, rhs: Capacitance) -> Self::Output {
        let volts = self.to_coulombs() / rhs.to_farads();
        ElectricPotential::new(volts, ElectricPotentialUnit::Volts)
    }
}

// Charge / Potential = Capacitance (C = Q/V)
impl Div<ElectricPotential> for ElectricCharge {
    type Output = Capacitance;

    fn div(self, rhs: ElectricPotential) -> Self::Output {
        let farads = self.to_coulombs() / rhs.to_volts();
        Capacitance::new(farads, CapacitanceUnit::Farads)
    }
}

// Charge * Potential = Energy (E = QV)
impl Mul<ElectricPotential> for ElectricCharge {
    type Output = Energy;

    fn mul(self, rhs: ElectricPotential) -> Self::Output {
        let joules = self.to_coulombs() * rhs.to_volts();
        Energy::new(joules, EnergyUnit::Joules)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_charge_creation() {
        let q = ElectricCharge::coulombs(10.0);
        assert_eq!(q.value(), 10.0);
        assert_eq!(q.unit(), ElectricChargeUnit::Coulombs);
    }

    #[test]
    fn test_charge_conversions() {
        let q = ElectricCharge::amperehours(1.0);
        assert_eq!(q.to_coulombs(), 3600.0);

        let q2 = ElectricCharge::coulombs(3600.0);
        assert_eq!(q2.to_amperehours(), 1.0);

        let q3 = ElectricCharge::milliamperehours(1000.0);
        assert!((q3.to_amperehours() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_charge_divided_by_time() {
        let q = ElectricCharge::coulombs(20.0);
        let t = Time::seconds(10.0);
        let i = q / t;
        assert_eq!(i.to_amperes(), 2.0);
    }

    #[test]
    fn test_charge_divided_by_current() {
        let q = ElectricCharge::coulombs(20.0);
        let i = ElectricCurrent::amperes(2.0);
        let t = q / i;
        assert_eq!(t.to_seconds(), 10.0);
    }

    #[test]
    fn test_charge_times_potential() {
        let q = ElectricCharge::coulombs(5.0);
        let v = ElectricPotential::volts(10.0);
        let e = q * v;
        assert_eq!(e.to_joules(), 50.0);
    }
}
