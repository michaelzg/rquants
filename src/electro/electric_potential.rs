//! Electric potential (voltage) quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Div, Mul};

/// Units of electric potential measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElectricPotentialUnit {
    /// Volts (V) - SI unit
    Volts,
    /// Microvolts (µV)
    Microvolts,
    /// Millivolts (mV)
    Millivolts,
    /// Kilovolts (kV)
    Kilovolts,
    /// Megavolts (MV)
    Megavolts,
}

impl ElectricPotentialUnit {
    /// All available electric potential units.
    pub const ALL: &'static [ElectricPotentialUnit] = &[
        ElectricPotentialUnit::Volts,
        ElectricPotentialUnit::Microvolts,
        ElectricPotentialUnit::Millivolts,
        ElectricPotentialUnit::Kilovolts,
        ElectricPotentialUnit::Megavolts,
    ];
}

impl_unit_display!(ElectricPotentialUnit);

impl UnitOfMeasure for ElectricPotentialUnit {
    fn symbol(&self) -> &'static str {
        match self {
            ElectricPotentialUnit::Volts => "V",
            ElectricPotentialUnit::Microvolts => "µV",
            ElectricPotentialUnit::Millivolts => "mV",
            ElectricPotentialUnit::Kilovolts => "kV",
            ElectricPotentialUnit::Megavolts => "MV",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            ElectricPotentialUnit::Volts => 1.0,
            ElectricPotentialUnit::Microvolts => 1e-6,
            ElectricPotentialUnit::Millivolts => 1e-3,
            ElectricPotentialUnit::Kilovolts => 1e3,
            ElectricPotentialUnit::Megavolts => 1e6,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            ElectricPotentialUnit::Volts
                | ElectricPotentialUnit::Microvolts
                | ElectricPotentialUnit::Millivolts
                | ElectricPotentialUnit::Kilovolts
                | ElectricPotentialUnit::Megavolts
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct ElectricPotential {
    value: f64,
    unit: ElectricPotentialUnit,
}

impl ElectricPotential {
    /// Creates a new ElectricPotential quantity.
    pub const fn new_const(value: f64, unit: ElectricPotentialUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates an ElectricPotential in volts.
    pub fn volts(value: f64) -> Self {
        Self::new(value, ElectricPotentialUnit::Volts)
    }

    /// Creates an ElectricPotential in microvolts.
    pub fn microvolts(value: f64) -> Self {
        Self::new(value, ElectricPotentialUnit::Microvolts)
    }

    /// Creates an ElectricPotential in millivolts.
    pub fn millivolts(value: f64) -> Self {
        Self::new(value, ElectricPotentialUnit::Millivolts)
    }

    /// Creates an ElectricPotential in kilovolts.
    pub fn kilovolts(value: f64) -> Self {
        Self::new(value, ElectricPotentialUnit::Kilovolts)
    }

    /// Creates an ElectricPotential in megavolts.
    pub fn megavolts(value: f64) -> Self {
        Self::new(value, ElectricPotentialUnit::Megavolts)
    }

    // Conversion methods
    /// Converts to volts.
    pub fn to_volts(&self) -> f64 {
        self.to(ElectricPotentialUnit::Volts)
    }

    /// Converts to microvolts.
    pub fn to_microvolts(&self) -> f64 {
        self.to(ElectricPotentialUnit::Microvolts)
    }

    /// Converts to millivolts.
    pub fn to_millivolts(&self) -> f64 {
        self.to(ElectricPotentialUnit::Millivolts)
    }

    /// Converts to kilovolts.
    pub fn to_kilovolts(&self) -> f64 {
        self.to(ElectricPotentialUnit::Kilovolts)
    }

    /// Converts to megavolts.
    pub fn to_megavolts(&self) -> f64 {
        self.to(ElectricPotentialUnit::Megavolts)
    }
}

impl_quantity!(ElectricPotential, ElectricPotentialUnit);

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

impl_dimension!(
    ElectricPotentialDimension,
    ElectricPotential,
    ElectricPotentialUnit,
    "ElectricPotential",
    ElectricPotentialUnit::Volts,
    ElectricPotentialUnit::Volts
);

/// Extension trait for creating ElectricPotential quantities from numeric types.
pub trait ElectricPotentialConversions {
    /// Creates an ElectricPotential in volts.
    fn volts(self) -> ElectricPotential;
    /// Creates an ElectricPotential in microvolts.
    fn microvolts(self) -> ElectricPotential;
    /// Creates an ElectricPotential in millivolts.
    fn millivolts(self) -> ElectricPotential;
    /// Creates an ElectricPotential in kilovolts.
    fn kilovolts(self) -> ElectricPotential;
    /// Creates an ElectricPotential in megavolts.
    fn megavolts(self) -> ElectricPotential;
}

impl ElectricPotentialConversions for f64 {
    fn volts(self) -> ElectricPotential {
        ElectricPotential::volts(self)
    }
    fn microvolts(self) -> ElectricPotential {
        ElectricPotential::microvolts(self)
    }
    fn millivolts(self) -> ElectricPotential {
        ElectricPotential::millivolts(self)
    }
    fn kilovolts(self) -> ElectricPotential {
        ElectricPotential::kilovolts(self)
    }
    fn megavolts(self) -> ElectricPotential {
        ElectricPotential::megavolts(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
