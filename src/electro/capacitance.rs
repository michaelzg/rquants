//! Capacitance quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Mul};

/// Units of capacitance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapacitanceUnit {
    /// Farads (F) - SI unit
    Farads,
    /// Picofarads (pF)
    Picofarads,
    /// Nanofarads (nF)
    Nanofarads,
    /// Microfarads (µF)
    Microfarads,
    /// Millifarads (mF)
    Millifarads,
}

impl CapacitanceUnit {
    /// All available capacitance units.
    pub const ALL: &'static [CapacitanceUnit] = &[
        CapacitanceUnit::Farads,
        CapacitanceUnit::Picofarads,
        CapacitanceUnit::Nanofarads,
        CapacitanceUnit::Microfarads,
        CapacitanceUnit::Millifarads,
    ];
}

impl_unit_display!(CapacitanceUnit);

impl UnitOfMeasure for CapacitanceUnit {
    fn symbol(&self) -> &'static str {
        match self {
            CapacitanceUnit::Farads => "F",
            CapacitanceUnit::Picofarads => "pF",
            CapacitanceUnit::Nanofarads => "nF",
            CapacitanceUnit::Microfarads => "µF",
            CapacitanceUnit::Millifarads => "mF",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            CapacitanceUnit::Farads => 1.0,
            CapacitanceUnit::Picofarads => 1e-12,
            CapacitanceUnit::Nanofarads => 1e-9,
            CapacitanceUnit::Microfarads => 1e-6,
            CapacitanceUnit::Millifarads => 1e-3,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            CapacitanceUnit::Farads
                | CapacitanceUnit::Picofarads
                | CapacitanceUnit::Nanofarads
                | CapacitanceUnit::Microfarads
                | CapacitanceUnit::Millifarads
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct Capacitance {
    value: f64,
    unit: CapacitanceUnit,
}

impl Capacitance {
    /// Creates a new Capacitance quantity.
    pub const fn new_const(value: f64, unit: CapacitanceUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a Capacitance in farads.
    pub fn farads(value: f64) -> Self {
        Self::new(value, CapacitanceUnit::Farads)
    }

    /// Creates a Capacitance in picofarads.
    pub fn picofarads(value: f64) -> Self {
        Self::new(value, CapacitanceUnit::Picofarads)
    }

    /// Creates a Capacitance in nanofarads.
    pub fn nanofarads(value: f64) -> Self {
        Self::new(value, CapacitanceUnit::Nanofarads)
    }

    /// Creates a Capacitance in microfarads.
    pub fn microfarads(value: f64) -> Self {
        Self::new(value, CapacitanceUnit::Microfarads)
    }

    /// Creates a Capacitance in millifarads.
    pub fn millifarads(value: f64) -> Self {
        Self::new(value, CapacitanceUnit::Millifarads)
    }

    // Conversion methods
    /// Converts to farads.
    pub fn to_farads(&self) -> f64 {
        self.to(CapacitanceUnit::Farads)
    }

    /// Converts to picofarads.
    pub fn to_picofarads(&self) -> f64 {
        self.to(CapacitanceUnit::Picofarads)
    }

    /// Converts to nanofarads.
    pub fn to_nanofarads(&self) -> f64 {
        self.to(CapacitanceUnit::Nanofarads)
    }

    /// Converts to microfarads.
    pub fn to_microfarads(&self) -> f64 {
        self.to(CapacitanceUnit::Microfarads)
    }

    /// Converts to millifarads.
    pub fn to_millifarads(&self) -> f64 {
        self.to(CapacitanceUnit::Millifarads)
    }
}

impl_quantity!(Capacitance, CapacitanceUnit);

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

impl_dimension!(
    CapacitanceDimension,
    Capacitance,
    CapacitanceUnit,
    "Capacitance",
    CapacitanceUnit::Farads,
    CapacitanceUnit::Farads
);

/// Extension trait for creating Capacitance quantities from numeric types.
pub trait CapacitanceConversions {
    /// Creates a Capacitance in farads.
    fn farads(self) -> Capacitance;
    /// Creates a Capacitance in picofarads.
    fn picofarads(self) -> Capacitance;
    /// Creates a Capacitance in nanofarads.
    fn nanofarads(self) -> Capacitance;
    /// Creates a Capacitance in microfarads.
    fn microfarads(self) -> Capacitance;
    /// Creates a Capacitance in millifarads.
    fn millifarads(self) -> Capacitance;
}

impl CapacitanceConversions for f64 {
    fn farads(self) -> Capacitance {
        Capacitance::farads(self)
    }
    fn picofarads(self) -> Capacitance {
        Capacitance::picofarads(self)
    }
    fn nanofarads(self) -> Capacitance {
        Capacitance::nanofarads(self)
    }
    fn microfarads(self) -> Capacitance {
        Capacitance::microfarads(self)
    }
    fn millifarads(self) -> Capacitance {
        Capacitance::millifarads(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
