//! Inductance quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Mul};

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

impl_unit_display!(InductanceUnit);

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

impl_quantity!(Inductance, InductanceUnit);

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

impl_dimension!(
    InductanceDimension,
    Inductance,
    InductanceUnit,
    "Inductance",
    InductanceUnit::Henrys,
    InductanceUnit::Henrys
);

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
