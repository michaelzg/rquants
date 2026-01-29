//! Electrical resistance quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of electrical resistance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElectricalResistanceUnit {
    /// Ohms (Ω) - SI unit
    Ohms,
    /// Milliohms (mΩ)
    Milliohms,
    /// Kilohms (kΩ)
    Kilohms,
    /// Megohms (MΩ)
    Megohms,
}

impl ElectricalResistanceUnit {
    /// All available electrical resistance units.
    pub const ALL: &'static [ElectricalResistanceUnit] = &[
        ElectricalResistanceUnit::Ohms,
        ElectricalResistanceUnit::Milliohms,
        ElectricalResistanceUnit::Kilohms,
        ElectricalResistanceUnit::Megohms,
    ];
}

impl fmt::Display for ElectricalResistanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for ElectricalResistanceUnit {
    fn symbol(&self) -> &'static str {
        match self {
            ElectricalResistanceUnit::Ohms => "Ω",
            ElectricalResistanceUnit::Milliohms => "mΩ",
            ElectricalResistanceUnit::Kilohms => "kΩ",
            ElectricalResistanceUnit::Megohms => "MΩ",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            ElectricalResistanceUnit::Ohms => 1.0,
            ElectricalResistanceUnit::Milliohms => 1e-3,
            ElectricalResistanceUnit::Kilohms => 1e3,
            ElectricalResistanceUnit::Megohms => 1e6,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            ElectricalResistanceUnit::Ohms
                | ElectricalResistanceUnit::Milliohms
                | ElectricalResistanceUnit::Kilohms
                | ElectricalResistanceUnit::Megohms
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct ElectricalResistance {
    value: f64,
    unit: ElectricalResistanceUnit,
}

impl ElectricalResistance {
    /// Creates a new ElectricalResistance quantity.
    pub const fn new_const(value: f64, unit: ElectricalResistanceUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates an ElectricalResistance in ohms.
    pub fn ohms(value: f64) -> Self {
        Self::new(value, ElectricalResistanceUnit::Ohms)
    }

    /// Creates an ElectricalResistance in milliohms.
    pub fn milliohms(value: f64) -> Self {
        Self::new(value, ElectricalResistanceUnit::Milliohms)
    }

    /// Creates an ElectricalResistance in kilohms.
    pub fn kilohms(value: f64) -> Self {
        Self::new(value, ElectricalResistanceUnit::Kilohms)
    }

    /// Creates an ElectricalResistance in megohms.
    pub fn megohms(value: f64) -> Self {
        Self::new(value, ElectricalResistanceUnit::Megohms)
    }

    // Conversion methods
    /// Converts to ohms.
    pub fn to_ohms(&self) -> f64 {
        self.to(ElectricalResistanceUnit::Ohms)
    }

    /// Converts to milliohms.
    pub fn to_milliohms(&self) -> f64 {
        self.to(ElectricalResistanceUnit::Milliohms)
    }

    /// Converts to kilohms.
    pub fn to_kilohms(&self) -> f64 {
        self.to(ElectricalResistanceUnit::Kilohms)
    }

    /// Converts to megohms.
    pub fn to_megohms(&self) -> f64 {
        self.to(ElectricalResistanceUnit::Megohms)
    }
}

impl fmt::Display for ElectricalResistance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for ElectricalResistance {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for ElectricalResistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for ElectricalResistance {
    type Unit = ElectricalResistanceUnit;

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

impl Add for ElectricalResistance {
    type Output = ElectricalResistance;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        ElectricalResistance::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for ElectricalResistance {
    type Output = ElectricalResistance;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        ElectricalResistance::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for ElectricalResistance {
    type Output = ElectricalResistance;

    fn mul(self, rhs: f64) -> Self::Output {
        ElectricalResistance::new(self.value * rhs, self.unit)
    }
}

impl Mul<ElectricalResistance> for f64 {
    type Output = ElectricalResistance;

    fn mul(self, rhs: ElectricalResistance) -> Self::Output {
        ElectricalResistance::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for ElectricalResistance {
    type Output = ElectricalResistance;

    fn div(self, rhs: f64) -> Self::Output {
        ElectricalResistance::new(self.value / rhs, self.unit)
    }
}

impl Div<ElectricalResistance> for ElectricalResistance {
    type Output = f64;

    fn div(self, rhs: ElectricalResistance) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for ElectricalResistance {
    type Output = ElectricalResistance;

    fn neg(self) -> Self::Output {
        ElectricalResistance::new(-self.value, self.unit)
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

/// Dimension for ElectricalResistance.
pub struct ElectricalResistanceDimension;

impl Dimension for ElectricalResistanceDimension {
    type Quantity = ElectricalResistance;
    type Unit = ElectricalResistanceUnit;

    fn name() -> &'static str {
        "ElectricalResistance"
    }

    fn primary_unit() -> Self::Unit {
        ElectricalResistanceUnit::Ohms
    }

    fn si_unit() -> Self::Unit {
        ElectricalResistanceUnit::Ohms
    }

    fn units() -> &'static [Self::Unit] {
        ElectricalResistanceUnit::ALL
    }
}

/// Extension trait for creating ElectricalResistance quantities from numeric types.
pub trait ElectricalResistanceConversions {
    /// Creates an ElectricalResistance in ohms.
    fn ohms(self) -> ElectricalResistance;
    /// Creates an ElectricalResistance in milliohms.
    fn milliohms(self) -> ElectricalResistance;
    /// Creates an ElectricalResistance in kilohms.
    fn kilohms(self) -> ElectricalResistance;
    /// Creates an ElectricalResistance in megohms.
    fn megohms(self) -> ElectricalResistance;
}

impl ElectricalResistanceConversions for f64 {
    fn ohms(self) -> ElectricalResistance {
        ElectricalResistance::ohms(self)
    }
    fn milliohms(self) -> ElectricalResistance {
        ElectricalResistance::milliohms(self)
    }
    fn kilohms(self) -> ElectricalResistance {
        ElectricalResistance::kilohms(self)
    }
    fn megohms(self) -> ElectricalResistance {
        ElectricalResistance::megohms(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
