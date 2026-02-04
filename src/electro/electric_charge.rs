//! Electric charge quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Div, Mul};

/// Units of electric charge measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElectricChargeUnit {
    /// Coulombs (C) - SI unit
    Coulombs,
    /// Milliampere-hours (mAh)
    Milliamperehours,
    /// Ampere-hours (Ah)
    Amperehours,
}

impl ElectricChargeUnit {
    /// All available electric charge units.
    pub const ALL: &'static [ElectricChargeUnit] = &[
        ElectricChargeUnit::Coulombs,
        ElectricChargeUnit::Milliamperehours,
        ElectricChargeUnit::Amperehours,
    ];
}

impl_unit_display!(ElectricChargeUnit);

impl UnitOfMeasure for ElectricChargeUnit {
    fn symbol(&self) -> &'static str {
        match self {
            ElectricChargeUnit::Coulombs => "C",
            ElectricChargeUnit::Milliamperehours => "mAh",
            ElectricChargeUnit::Amperehours => "Ah",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            ElectricChargeUnit::Coulombs => 1.0,
            // 1 Ah = 3600 C (1 A * 3600 s)
            ElectricChargeUnit::Amperehours => 3600.0,
            // 1 mAh = 3.6 C (0.001 A * 3600 s)
            ElectricChargeUnit::Milliamperehours => 3.6,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, ElectricChargeUnit::Coulombs)
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct ElectricCharge {
    value: f64,
    unit: ElectricChargeUnit,
}

impl ElectricCharge {
    /// Creates a new ElectricCharge quantity.
    pub const fn new_const(value: f64, unit: ElectricChargeUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates an ElectricCharge in coulombs.
    pub fn coulombs(value: f64) -> Self {
        Self::new(value, ElectricChargeUnit::Coulombs)
    }

    /// Creates an ElectricCharge in milliampere-hours.
    pub fn milliamperehours(value: f64) -> Self {
        Self::new(value, ElectricChargeUnit::Milliamperehours)
    }

    /// Creates an ElectricCharge in ampere-hours.
    pub fn amperehours(value: f64) -> Self {
        Self::new(value, ElectricChargeUnit::Amperehours)
    }

    // Conversion methods
    /// Converts to coulombs.
    pub fn to_coulombs(&self) -> f64 {
        self.to(ElectricChargeUnit::Coulombs)
    }

    /// Converts to milliampere-hours.
    pub fn to_milliamperehours(&self) -> f64 {
        self.to(ElectricChargeUnit::Milliamperehours)
    }

    /// Converts to ampere-hours.
    pub fn to_amperehours(&self) -> f64 {
        self.to(ElectricChargeUnit::Amperehours)
    }
}

impl_quantity!(ElectricCharge, ElectricChargeUnit);

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

impl_dimension!(
    ElectricChargeDimension,
    ElectricCharge,
    ElectricChargeUnit,
    "ElectricCharge",
    ElectricChargeUnit::Coulombs,
    ElectricChargeUnit::Coulombs
);

/// Extension trait for creating ElectricCharge quantities from numeric types.
pub trait ElectricChargeConversions {
    /// Creates an ElectricCharge in coulombs.
    fn coulombs(self) -> ElectricCharge;
    /// Creates an ElectricCharge in milliampere-hours.
    fn milliamperehours(self) -> ElectricCharge;
    /// Creates an ElectricCharge in ampere-hours.
    fn amperehours(self) -> ElectricCharge;
}

impl ElectricChargeConversions for f64 {
    fn coulombs(self) -> ElectricCharge {
        ElectricCharge::coulombs(self)
    }
    fn milliamperehours(self) -> ElectricCharge {
        ElectricCharge::milliamperehours(self)
    }
    fn amperehours(self) -> ElectricCharge {
        ElectricCharge::amperehours(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
