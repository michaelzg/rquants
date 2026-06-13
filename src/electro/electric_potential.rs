//! Electric potential (voltage) quantity and units.
use crate::core::Quantity;
use std::ops::{Div, Mul};
crate::quantity! {
    /// A quantity of electric potential (voltage).
    ///
    /// Electric potential is the electric potential energy per unit charge.
    /// It represents the work needed to move a unit charge from a reference point
    /// to a specific point in an electric field.
    ///
    /// # Relationships
    ///
    /// - Potential / Current = Resistance (R = V/I, Ohm's law)
    /// - Potential / Resistance = Current (I = V/R)
    /// - Potential × Current = Power (P = VI)
    /// - Potential × Charge = Energy (E = VQ)
    /// - Potential × Time = MagneticFlux (Wb = V·s)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let voltage = ElectricPotential::volts(12.0);
    /// let current = ElectricCurrent::amperes(2.0);
    ///
    /// // Power = Voltage × Current
    /// let power = voltage * current;
    /// assert!((power.to_watts() - 24.0).abs() < 1e-10);
    /// ```
    pub quantity ElectricPotential {
        unit: ElectricPotentialUnit;
        dimension: ElectricPotentialDimension;
        conversions: ElectricPotentialConversions;
        name: "ElectricPotential";
        primary: Volts;
        si: Volts;

        units {
            /// Volts (V) - SI unit
            Volts {
                symbol: "V",
                factor: 1.0,
                ctor: volts,
                to: to_volts,
                si: true
            },
            /// Microvolts (µV)
            Microvolts {
                symbol: "µV",
                factor: 1e-6,
                ctor: microvolts,
                to: to_microvolts,
                si: true
            },
            /// Millivolts (mV)
            Millivolts {
                symbol: "mV",
                factor: 1e-3,
                ctor: millivolts,
                to: to_millivolts,
                si: true
            },
            /// Kilovolts (kV)
            Kilovolts {
                symbol: "kV",
                factor: 1e3,
                ctor: kilovolts,
                to: to_kilovolts,
                si: true
            },
            /// Megavolts (MV)
            Megavolts {
                symbol: "MV",
                factor: 1e6,
                ctor: megavolts,
                to: to_megavolts,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use super::electric_charge::{ElectricCharge, ElectricChargeUnit};
use super::electric_current::{ElectricCurrent, ElectricCurrentUnit};
use super::electrical_resistance::{ElectricalResistance, ElectricalResistanceUnit};
use super::magnetic_flux::{MagneticFlux, MagneticFluxUnit};
use crate::energy::{Energy, EnergyUnit, Power};
use crate::time::Time;

// Potential / Current = Resistance (R = V/I, Ohm's law)
impl Div<ElectricCurrent> for ElectricPotential {
    type Output = ElectricalResistance;

    fn div(self, rhs: ElectricCurrent) -> Self::Output {
        let ohms = self.to_volts() / rhs.to_amperes();
        ElectricalResistance::new(ohms, ElectricalResistanceUnit::Ohms)
    }
}

// Potential / Resistance = Current (I = V/R)
impl Div<ElectricalResistance> for ElectricPotential {
    type Output = ElectricCurrent;

    fn div(self, rhs: ElectricalResistance) -> Self::Output {
        let amperes = self.to_volts() / rhs.to_ohms();
        ElectricCurrent::new(amperes, ElectricCurrentUnit::Amperes)
    }
}

// Potential / Power = Current (I = P/V)
impl Div<Power> for ElectricPotential {
    type Output = ElectricCurrent;

    fn div(self, rhs: Power) -> Self::Output {
        let amperes = rhs.to_watts() / self.to_volts();
        ElectricCurrent::new(amperes, ElectricCurrentUnit::Amperes)
    }
}

// Potential / Energy = Charge (Q = E/V)
impl Div<Energy> for ElectricPotential {
    type Output = ElectricCharge;

    fn div(self, rhs: Energy) -> Self::Output {
        let coulombs = rhs.to_joules() / self.to_volts();
        ElectricCharge::new(coulombs, ElectricChargeUnit::Coulombs)
    }
}

// Potential * Charge = Energy (E = VQ)
impl Mul<ElectricCharge> for ElectricPotential {
    type Output = Energy;

    fn mul(self, rhs: ElectricCharge) -> Self::Output {
        let joules = self.to_volts() * rhs.to_coulombs();
        Energy::new(joules, EnergyUnit::Joules)
    }
}

// Potential * Time = MagneticFlux (Wb = V·s)
impl Mul<Time> for ElectricPotential {
    type Output = MagneticFlux;

    fn mul(self, rhs: Time) -> Self::Output {
        let webers = self.to_volts() * rhs.to_seconds();
        MagneticFlux::new(webers, MagneticFluxUnit::Webers)
    }
}

// Time * Potential = MagneticFlux
impl Mul<ElectricPotential> for Time {
    type Output = MagneticFlux;

    fn mul(self, rhs: ElectricPotential) -> Self::Output {
        let webers = self.to_seconds() * rhs.to_volts();
        MagneticFlux::new(webers, MagneticFluxUnit::Webers)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_potential_creation() {
        let v = ElectricPotential::volts(12.0);
        assert_eq!(v.value(), 12.0);
        assert_eq!(v.unit(), ElectricPotentialUnit::Volts);
    }

    #[test]
    fn test_potential_conversions() {
        let v = ElectricPotential::kilovolts(1.0);
        assert_eq!(v.to_volts(), 1000.0);

        let v2 = ElectricPotential::millivolts(5000.0);
        assert_eq!(v2.to_volts(), 5.0);
    }

    #[test]
    fn test_ohms_law_resistance() {
        let v = ElectricPotential::volts(10.0);
        let i = ElectricCurrent::amperes(2.0);
        let r = v / i;
        assert_eq!(r.to_ohms(), 5.0);
    }

    #[test]
    fn test_ohms_law_current() {
        let v = ElectricPotential::volts(12.0);
        let r = ElectricalResistance::ohms(3.0);
        let i = v / r;
        assert_eq!(i.to_amperes(), 4.0);
    }

    #[test]
    fn test_potential_times_charge() {
        let v = ElectricPotential::volts(10.0);
        let q = ElectricCharge::coulombs(5.0);
        let e = v * q;
        assert_eq!(e.to_joules(), 50.0);
    }

    #[test]
    fn test_potential_times_time() {
        let v = ElectricPotential::volts(10.0);
        let t = Time::seconds(5.0);
        let flux = v * t;
        assert_eq!(flux.to_webers(), 50.0);
    }
}
