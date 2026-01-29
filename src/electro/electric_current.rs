//! Electric current quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of electric current measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElectricCurrentUnit {
    /// Amperes (A) - SI unit
    Amperes,
    /// Milliamperes (mA)
    Milliamperes,
}

impl ElectricCurrentUnit {
    /// All available electric current units.
    pub const ALL: &'static [ElectricCurrentUnit] =
        &[ElectricCurrentUnit::Amperes, ElectricCurrentUnit::Milliamperes];
}

impl fmt::Display for ElectricCurrentUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for ElectricCurrentUnit {
    fn symbol(&self) -> &'static str {
        match self {
            ElectricCurrentUnit::Amperes => "A",
            ElectricCurrentUnit::Milliamperes => "mA",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            ElectricCurrentUnit::Amperes => 1.0,
            ElectricCurrentUnit::Milliamperes => 1e-3,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            ElectricCurrentUnit::Amperes | ElectricCurrentUnit::Milliamperes
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct ElectricCurrent {
    value: f64,
    unit: ElectricCurrentUnit,
}

impl ElectricCurrent {
    /// Creates a new ElectricCurrent quantity.
    pub const fn new_const(value: f64, unit: ElectricCurrentUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates an ElectricCurrent in amperes.
    pub fn amperes(value: f64) -> Self {
        Self::new(value, ElectricCurrentUnit::Amperes)
    }

    /// Creates an ElectricCurrent in milliamperes.
    pub fn milliamperes(value: f64) -> Self {
        Self::new(value, ElectricCurrentUnit::Milliamperes)
    }

    // Conversion methods
    /// Converts to amperes.
    pub fn to_amperes(&self) -> f64 {
        self.to(ElectricCurrentUnit::Amperes)
    }

    /// Converts to milliamperes.
    pub fn to_milliamperes(&self) -> f64 {
        self.to(ElectricCurrentUnit::Milliamperes)
    }
}

impl fmt::Display for ElectricCurrent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for ElectricCurrent {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for ElectricCurrent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for ElectricCurrent {
    type Unit = ElectricCurrentUnit;

    fn new(value: f64, unit: Self::Unit) -> Self {
        Self { value, unit }
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> Self::Unit {
        self.unit
    }
}

// Arithmetic operations

impl Add for ElectricCurrent {
    type Output = ElectricCurrent;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        ElectricCurrent::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for ElectricCurrent {
    type Output = ElectricCurrent;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        ElectricCurrent::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for ElectricCurrent {
    type Output = ElectricCurrent;

    fn mul(self, rhs: f64) -> Self::Output {
        ElectricCurrent::new(self.value * rhs, self.unit)
    }
}

impl Mul<ElectricCurrent> for f64 {
    type Output = ElectricCurrent;

    fn mul(self, rhs: ElectricCurrent) -> Self::Output {
        ElectricCurrent::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for ElectricCurrent {
    type Output = ElectricCurrent;

    fn div(self, rhs: f64) -> Self::Output {
        ElectricCurrent::new(self.value / rhs, self.unit)
    }
}

impl Div<ElectricCurrent> for ElectricCurrent {
    type Output = f64;

    fn div(self, rhs: ElectricCurrent) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for ElectricCurrent {
    type Output = ElectricCurrent;

    fn neg(self) -> Self::Output {
        ElectricCurrent::new(-self.value, self.unit)
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

/// Dimension for ElectricCurrent.
pub struct ElectricCurrentDimension;

impl Dimension for ElectricCurrentDimension {
    type Quantity = ElectricCurrent;
    type Unit = ElectricCurrentUnit;

    fn name() -> &'static str {
        "ElectricCurrent"
    }

    fn primary_unit() -> Self::Unit {
        ElectricCurrentUnit::Amperes
    }

    fn si_unit() -> Self::Unit {
        ElectricCurrentUnit::Amperes
    }

    fn units() -> &'static [Self::Unit] {
        ElectricCurrentUnit::ALL
    }
}

/// Extension trait for creating ElectricCurrent quantities from numeric types.
pub trait ElectricCurrentConversions {
    /// Creates an ElectricCurrent in amperes.
    fn amperes(self) -> ElectricCurrent;
    /// Creates an ElectricCurrent in milliamperes.
    fn milliamperes(self) -> ElectricCurrent;
}

impl ElectricCurrentConversions for f64 {
    fn amperes(self) -> ElectricCurrent {
        ElectricCurrent::amperes(self)
    }
    fn milliamperes(self) -> ElectricCurrent {
        ElectricCurrent::milliamperes(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
