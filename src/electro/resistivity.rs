//! Resistivity quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Div};

/// Units of resistivity measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResistivityUnit {
    /// Ohm-meters (Ω·m) - SI unit
    OhmMeters,
}

impl ResistivityUnit {
    /// All available resistivity units.
    pub const ALL: &'static [ResistivityUnit] = &[ResistivityUnit::OhmMeters];
}

impl_unit_display!(ResistivityUnit);

impl UnitOfMeasure for ResistivityUnit {
    fn symbol(&self) -> &'static str {
        match self {
            ResistivityUnit::OhmMeters => "Ω·m",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            ResistivityUnit::OhmMeters => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, ResistivityUnit::OhmMeters)
    }
}

/// A quantity of resistivity.
///
/// Resistivity is an intrinsic property of a material that quantifies how strongly
/// it resists electric current. It is the inverse of conductivity.
/// ρ = R × A / L (resistivity = resistance × area / length)
///
/// # Relationships
///
/// - Resistivity = Resistance × Length
/// - Resistivity / Length = Resistance
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let resistivity = Resistivity::ohm_meters(1.68e-8); // Copper
/// let length = Length::meters(10.0);
///
/// // Resistance = Resistivity / Length
/// let resistance = resistivity / length;
/// assert!((resistance.to_ohms() - 1.68e-9).abs() < 1e-15);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Resistivity {
    value: f64,
    unit: ResistivityUnit,
}

impl Resistivity {
    /// Creates a new Resistivity quantity.
    pub const fn new_const(value: f64, unit: ResistivityUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a Resistivity in ohm-meters.
    pub fn ohm_meters(value: f64) -> Self {
        Self::new(value, ResistivityUnit::OhmMeters)
    }

    // Conversion methods
    /// Converts to ohm-meters.
    pub fn to_ohm_meters(&self) -> f64 {
        self.to(ResistivityUnit::OhmMeters)
    }
}

impl_quantity!(Resistivity, ResistivityUnit);

// Cross-quantity operations
use super::electrical_resistance::{ElectricalResistance, ElectricalResistanceUnit};
use crate::space::Length;

// Resistivity / Length = Resistance
impl Div<Length> for Resistivity {
    type Output = ElectricalResistance;

    fn div(self, rhs: Length) -> Self::Output {
        let ohms = self.to_ohm_meters() / rhs.to_meters();
        ElectricalResistance::new(ohms, ElectricalResistanceUnit::Ohms)
    }
}

impl_dimension!(
    ResistivityDimension,
    Resistivity,
    ResistivityUnit,
    "Resistivity",
    ResistivityUnit::OhmMeters,
    ResistivityUnit::OhmMeters
);

/// Extension trait for creating Resistivity quantities from numeric types.
pub trait ResistivityConversions {
    /// Creates a Resistivity in ohm-meters.
    fn ohm_meters(self) -> Resistivity;
}

impl ResistivityConversions for f64 {
    fn ohm_meters(self) -> Resistivity {
        Resistivity::ohm_meters(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resistivity_creation() {
        let rho = Resistivity::ohm_meters(1.68e-8);
        assert_eq!(rho.value(), 1.68e-8);
        assert_eq!(rho.unit(), ResistivityUnit::OhmMeters);
    }

    #[test]
    fn test_resistivity_divided_by_length() {
        let rho = Resistivity::ohm_meters(1.68e-8);
        let length = Length::meters(10.0);
        let r = rho / length;
        assert!((r.to_ohms() - 1.68e-9).abs() < 1e-15);
    }
}
