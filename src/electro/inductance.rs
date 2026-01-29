//! Inductance quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of inductance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InductanceUnit {
    /// Henrys (H) - SI unit
    Henrys,
    /// Microhenrys (µH)
    Microhenrys,
    /// Millihenrys (mH)
    Millihenrys,
}

impl InductanceUnit {
    /// All available inductance units.
    pub const ALL: &'static [InductanceUnit] = &[
        InductanceUnit::Henrys,
        InductanceUnit::Microhenrys,
        InductanceUnit::Millihenrys,
    ];
}

impl fmt::Display for InductanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for InductanceUnit {
    fn symbol(&self) -> &'static str {
        match self {
            InductanceUnit::Henrys => "H",
            InductanceUnit::Microhenrys => "µH",
            InductanceUnit::Millihenrys => "mH",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            InductanceUnit::Henrys => 1.0,
            InductanceUnit::Microhenrys => 1e-6,
            InductanceUnit::Millihenrys => 1e-3,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            InductanceUnit::Henrys | InductanceUnit::Microhenrys | InductanceUnit::Millihenrys
        )
    }
}

/// A quantity of inductance.
///
/// Inductance is the property of an electrical conductor by which a change in current
/// through it induces an electromotive force in both the conductor itself and in any
/// nearby conductors by mutual inductance.
///
/// # Relationships
///
/// - Inductance × Current = MagneticFlux (Φ = LI)
/// - Inductance = MagneticFlux / Current (L = Φ/I)
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let inductance = Inductance::millihenrys(50.0);
/// let current = ElectricCurrent::amperes(2.0);
///
/// // MagneticFlux = Inductance × Current
/// let flux = inductance * current;
/// assert!((flux.to_webers() - 0.1).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Inductance {
    value: f64,
    unit: InductanceUnit,
}

impl Inductance {
    /// Creates a new Inductance quantity.
    pub const fn new_const(value: f64, unit: InductanceUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates an Inductance in henrys.
    pub fn henrys(value: f64) -> Self {
        Self::new(value, InductanceUnit::Henrys)
    }

    /// Creates an Inductance in microhenrys.
    pub fn microhenrys(value: f64) -> Self {
        Self::new(value, InductanceUnit::Microhenrys)
    }

    /// Creates an Inductance in millihenrys.
    pub fn millihenrys(value: f64) -> Self {
        Self::new(value, InductanceUnit::Millihenrys)
    }

    // Conversion methods
    /// Converts to henrys.
    pub fn to_henrys(&self) -> f64 {
        self.to(InductanceUnit::Henrys)
    }

    /// Converts to microhenrys.
    pub fn to_microhenrys(&self) -> f64 {
        self.to(InductanceUnit::Microhenrys)
    }

    /// Converts to millihenrys.
    pub fn to_millihenrys(&self) -> f64 {
        self.to(InductanceUnit::Millihenrys)
    }
}

impl fmt::Display for Inductance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Inductance {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Inductance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Inductance {
    type Unit = InductanceUnit;

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

impl Add for Inductance {
    type Output = Inductance;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Inductance::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Inductance {
    type Output = Inductance;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Inductance::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Inductance {
    type Output = Inductance;

    fn mul(self, rhs: f64) -> Self::Output {
        Inductance::new(self.value * rhs, self.unit)
    }
}

impl Mul<Inductance> for f64 {
    type Output = Inductance;

    fn mul(self, rhs: Inductance) -> Self::Output {
        Inductance::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for Inductance {
    type Output = Inductance;

    fn div(self, rhs: f64) -> Self::Output {
        Inductance::new(self.value / rhs, self.unit)
    }
}

impl Div<Inductance> for Inductance {
    type Output = f64;

    fn div(self, rhs: Inductance) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Inductance {
    type Output = Inductance;

    fn neg(self) -> Self::Output {
        Inductance::new(-self.value, self.unit)
    }
}

// Cross-quantity operations
use super::electric_current::ElectricCurrent;
use super::magnetic_flux::{MagneticFlux, MagneticFluxUnit};

// Inductance * Current = MagneticFlux (Φ = LI)
impl Mul<ElectricCurrent> for Inductance {
    type Output = MagneticFlux;

    fn mul(self, rhs: ElectricCurrent) -> Self::Output {
        let webers = self.to_henrys() * rhs.to_amperes();
        MagneticFlux::new(webers, MagneticFluxUnit::Webers)
    }
}

// Current * Inductance = MagneticFlux
impl Mul<Inductance> for ElectricCurrent {
    type Output = MagneticFlux;

    fn mul(self, rhs: Inductance) -> Self::Output {
        let webers = self.to_amperes() * rhs.to_henrys();
        MagneticFlux::new(webers, MagneticFluxUnit::Webers)
    }
}

/// Dimension for Inductance.
pub struct InductanceDimension;

impl Dimension for InductanceDimension {
    type Quantity = Inductance;
    type Unit = InductanceUnit;

    fn name() -> &'static str {
        "Inductance"
    }

    fn primary_unit() -> Self::Unit {
        InductanceUnit::Henrys
    }

    fn si_unit() -> Self::Unit {
        InductanceUnit::Henrys
    }

    fn units() -> &'static [Self::Unit] {
        InductanceUnit::ALL
    }
}

/// Extension trait for creating Inductance quantities from numeric types.
pub trait InductanceConversions {
    /// Creates an Inductance in henrys.
    fn henrys(self) -> Inductance;
    /// Creates an Inductance in microhenrys.
    fn microhenrys(self) -> Inductance;
    /// Creates an Inductance in millihenrys.
    fn millihenrys(self) -> Inductance;
}

impl InductanceConversions for f64 {
    fn henrys(self) -> Inductance {
        Inductance::henrys(self)
    }
    fn microhenrys(self) -> Inductance {
        Inductance::microhenrys(self)
    }
    fn millihenrys(self) -> Inductance {
        Inductance::millihenrys(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inductance_creation() {
        let l = Inductance::henrys(1.0);
        assert_eq!(l.value(), 1.0);
        assert_eq!(l.unit(), InductanceUnit::Henrys);
    }

    #[test]
    fn test_inductance_conversions() {
        let l = Inductance::millihenrys(1.0);
        assert_eq!(l.to_henrys(), 0.001);

        let l2 = Inductance::microhenrys(1000.0);
        assert!((l2.to_millihenrys() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_inductance_times_current() {
        let l = Inductance::millihenrys(50.0);
        let i = ElectricCurrent::amperes(2.0);
        let flux = l * i;
        // 50 mH * 2 A = 0.1 Wb
        assert!((flux.to_webers() - 0.1).abs() < 1e-10);
    }
}
