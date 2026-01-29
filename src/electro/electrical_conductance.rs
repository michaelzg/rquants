//! Electrical conductance quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of electrical conductance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElectricalConductanceUnit {
    /// Siemens (S) - SI unit
    Siemens,
    /// Millisiemens (mS)
    Millisiemens,
    /// Microsiemens (µS)
    Microsiemens,
}

impl ElectricalConductanceUnit {
    /// All available electrical conductance units.
    pub const ALL: &'static [ElectricalConductanceUnit] = &[
        ElectricalConductanceUnit::Siemens,
        ElectricalConductanceUnit::Millisiemens,
        ElectricalConductanceUnit::Microsiemens,
    ];
}

impl fmt::Display for ElectricalConductanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for ElectricalConductanceUnit {
    fn symbol(&self) -> &'static str {
        match self {
            ElectricalConductanceUnit::Siemens => "S",
            ElectricalConductanceUnit::Millisiemens => "mS",
            ElectricalConductanceUnit::Microsiemens => "µS",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            ElectricalConductanceUnit::Siemens => 1.0,
            ElectricalConductanceUnit::Millisiemens => 1e-3,
            ElectricalConductanceUnit::Microsiemens => 1e-6,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            ElectricalConductanceUnit::Siemens
                | ElectricalConductanceUnit::Millisiemens
                | ElectricalConductanceUnit::Microsiemens
        )
    }
}

/// A quantity of electrical conductance.
///
/// Electrical conductance is the reciprocal of electrical resistance,
/// representing the ease with which electric current flows through a conductor.
/// G = 1 / R (conductance = 1 / resistance)
///
/// # Relationships
///
/// - Conductance = 1 / Resistance
/// - Resistance = 1 / Conductance
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let conductance = ElectricalConductance::siemens(0.01);
/// let resistance = ElectricalResistance::ohms(100.0);
///
/// // Conductance = 1 / Resistance
/// assert!((conductance.to_siemens() - 1.0 / resistance.to_ohms()).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ElectricalConductance {
    value: f64,
    unit: ElectricalConductanceUnit,
}

impl ElectricalConductance {
    /// Creates a new ElectricalConductance quantity.
    pub const fn new_const(value: f64, unit: ElectricalConductanceUnit) -> Self {
        Self { value, unit }
    }

    /// Creates an ElectricalConductance from resistance (G = 1/R).
    pub fn from_resistance(resistance: super::electrical_resistance::ElectricalResistance) -> Self {
        let siemens = 1.0 / resistance.to_ohms();
        Self::new(siemens, ElectricalConductanceUnit::Siemens)
    }

    // Constructors
    /// Creates an ElectricalConductance in siemens.
    pub fn siemens(value: f64) -> Self {
        Self::new(value, ElectricalConductanceUnit::Siemens)
    }

    /// Creates an ElectricalConductance in millisiemens.
    pub fn millisiemens(value: f64) -> Self {
        Self::new(value, ElectricalConductanceUnit::Millisiemens)
    }

    /// Creates an ElectricalConductance in microsiemens.
    pub fn microsiemens(value: f64) -> Self {
        Self::new(value, ElectricalConductanceUnit::Microsiemens)
    }

    // Conversion methods
    /// Converts to siemens.
    pub fn to_siemens(&self) -> f64 {
        self.to(ElectricalConductanceUnit::Siemens)
    }

    /// Converts to millisiemens.
    pub fn to_millisiemens(&self) -> f64 {
        self.to(ElectricalConductanceUnit::Millisiemens)
    }

    /// Converts to microsiemens.
    pub fn to_microsiemens(&self) -> f64 {
        self.to(ElectricalConductanceUnit::Microsiemens)
    }

    /// Converts to resistance (R = 1/G).
    pub fn to_resistance(&self) -> super::electrical_resistance::ElectricalResistance {
        use super::electrical_resistance::{ElectricalResistance, ElectricalResistanceUnit};
        let ohms = 1.0 / self.to_siemens();
        ElectricalResistance::new(ohms, ElectricalResistanceUnit::Ohms)
    }
}

impl fmt::Display for ElectricalConductance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for ElectricalConductance {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for ElectricalConductance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for ElectricalConductance {
    type Unit = ElectricalConductanceUnit;

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

impl Add for ElectricalConductance {
    type Output = ElectricalConductance;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        ElectricalConductance::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for ElectricalConductance {
    type Output = ElectricalConductance;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        ElectricalConductance::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for ElectricalConductance {
    type Output = ElectricalConductance;

    fn mul(self, rhs: f64) -> Self::Output {
        ElectricalConductance::new(self.value * rhs, self.unit)
    }
}

impl Mul<ElectricalConductance> for f64 {
    type Output = ElectricalConductance;

    fn mul(self, rhs: ElectricalConductance) -> Self::Output {
        ElectricalConductance::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for ElectricalConductance {
    type Output = ElectricalConductance;

    fn div(self, rhs: f64) -> Self::Output {
        ElectricalConductance::new(self.value / rhs, self.unit)
    }
}

impl Div<ElectricalConductance> for ElectricalConductance {
    type Output = f64;

    fn div(self, rhs: ElectricalConductance) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for ElectricalConductance {
    type Output = ElectricalConductance;

    fn neg(self) -> Self::Output {
        ElectricalConductance::new(-self.value, self.unit)
    }
}

// Cross-quantity operations
use crate::space::Length;

// Conductance / Length = Conductivity
use super::conductivity::{Conductivity, ConductivityUnit};

impl Div<Length> for ElectricalConductance {
    type Output = Conductivity;

    fn div(self, rhs: Length) -> Self::Output {
        let s_per_m = self.to_siemens() / rhs.to_meters();
        Conductivity::new(s_per_m, ConductivityUnit::SiemensPerMeter)
    }
}

/// Dimension for ElectricalConductance.
pub struct ElectricalConductanceDimension;

impl Dimension for ElectricalConductanceDimension {
    type Quantity = ElectricalConductance;
    type Unit = ElectricalConductanceUnit;

    fn name() -> &'static str {
        "ElectricalConductance"
    }

    fn primary_unit() -> Self::Unit {
        ElectricalConductanceUnit::Siemens
    }

    fn si_unit() -> Self::Unit {
        ElectricalConductanceUnit::Siemens
    }

    fn units() -> &'static [Self::Unit] {
        ElectricalConductanceUnit::ALL
    }
}

/// Extension trait for creating ElectricalConductance quantities from numeric types.
pub trait ElectricalConductanceConversions {
    /// Creates an ElectricalConductance in siemens.
    fn siemens(self) -> ElectricalConductance;
    /// Creates an ElectricalConductance in millisiemens.
    fn millisiemens(self) -> ElectricalConductance;
    /// Creates an ElectricalConductance in microsiemens.
    fn microsiemens(self) -> ElectricalConductance;
}

impl ElectricalConductanceConversions for f64 {
    fn siemens(self) -> ElectricalConductance {
        ElectricalConductance::siemens(self)
    }
    fn millisiemens(self) -> ElectricalConductance {
        ElectricalConductance::millisiemens(self)
    }
    fn microsiemens(self) -> ElectricalConductance {
        ElectricalConductance::microsiemens(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::electrical_resistance::ElectricalResistance;

    #[test]
    fn test_conductance_creation() {
        let g = ElectricalConductance::siemens(0.01);
        assert_eq!(g.value(), 0.01);
        assert_eq!(g.unit(), ElectricalConductanceUnit::Siemens);
    }

    #[test]
    fn test_conductance_conversions() {
        let g = ElectricalConductance::siemens(1.0);
        assert_eq!(g.to_millisiemens(), 1000.0);

        let g2 = ElectricalConductance::millisiemens(500.0);
        assert_eq!(g2.to_siemens(), 0.5);
    }

    #[test]
    fn test_conductance_from_resistance() {
        let r = ElectricalResistance::ohms(100.0);
        let g = ElectricalConductance::from_resistance(r);
        assert_eq!(g.to_siemens(), 0.01);
    }

    #[test]
    fn test_conductance_to_resistance() {
        let g = ElectricalConductance::siemens(0.01);
        let r = g.to_resistance();
        assert_eq!(r.to_ohms(), 100.0);
    }
}
