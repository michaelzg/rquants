//! Conductivity quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Mul};

/// Units of conductivity measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConductivityUnit {
    /// Siemens per meter (S/m) - SI unit
    SiemensPerMeter,
}

impl ConductivityUnit {
    /// All available conductivity units.
    pub const ALL: &'static [ConductivityUnit] = &[ConductivityUnit::SiemensPerMeter];
}

impl_unit_display!(ConductivityUnit);

impl UnitOfMeasure for ConductivityUnit {
    fn symbol(&self) -> &'static str {
        match self {
            ConductivityUnit::SiemensPerMeter => "S/m",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            ConductivityUnit::SiemensPerMeter => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, ConductivityUnit::SiemensPerMeter)
    }
}

/// A quantity of conductivity.
///
/// Conductivity is an intrinsic property of a material that quantifies how easily
/// it conducts electric current. It is the inverse of resistivity.
/// σ = 1/ρ (conductivity = 1/resistivity)
///
/// # Relationships
///
/// - Conductivity = Conductance / Length
/// - Conductivity × Length = Conductance
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let conductivity = Conductivity::siemens_per_meter(5.96e7); // Copper
/// let length = Length::meters(10.0);
///
/// // Conductance = Conductivity × Length
/// let conductance = conductivity * length;
/// assert!((conductance.to_siemens() - 5.96e8).abs() < 1e2);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Conductivity {
    value: f64,
    unit: ConductivityUnit,
}

impl Conductivity {
    /// Creates a new Conductivity quantity.
    pub const fn new_const(value: f64, unit: ConductivityUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a Conductivity in siemens per meter.
    pub fn siemens_per_meter(value: f64) -> Self {
        Self::new(value, ConductivityUnit::SiemensPerMeter)
    }

    // Conversion methods
    /// Converts to siemens per meter.
    pub fn to_siemens_per_meter(&self) -> f64 {
        self.to(ConductivityUnit::SiemensPerMeter)
    }
}

impl_quantity!(Conductivity, ConductivityUnit);

// Cross-quantity operations
use super::electrical_conductance::{ElectricalConductance, ElectricalConductanceUnit};
use crate::space::Length;

// Conductivity * Length = Conductance
impl Mul<Length> for Conductivity {
    type Output = ElectricalConductance;

    fn mul(self, rhs: Length) -> Self::Output {
        let siemens = self.to_siemens_per_meter() * rhs.to_meters();
        ElectricalConductance::new(siemens, ElectricalConductanceUnit::Siemens)
    }
}

// Length * Conductivity = Conductance
impl Mul<Conductivity> for Length {
    type Output = ElectricalConductance;

    fn mul(self, rhs: Conductivity) -> Self::Output {
        let siemens = self.to_meters() * rhs.to_siemens_per_meter();
        ElectricalConductance::new(siemens, ElectricalConductanceUnit::Siemens)
    }
}

impl_dimension!(
    ConductivityDimension,
    Conductivity,
    ConductivityUnit,
    "Conductivity",
    ConductivityUnit::SiemensPerMeter,
    ConductivityUnit::SiemensPerMeter
);

/// Extension trait for creating Conductivity quantities from numeric types.
pub trait ConductivityConversions {
    /// Creates a Conductivity in siemens per meter.
    fn siemens_per_meter(self) -> Conductivity;
}

impl ConductivityConversions for f64 {
    fn siemens_per_meter(self) -> Conductivity {
        Conductivity::siemens_per_meter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conductivity_creation() {
        let sigma = Conductivity::siemens_per_meter(5.96e7);
        assert_eq!(sigma.value(), 5.96e7);
        assert_eq!(sigma.unit(), ConductivityUnit::SiemensPerMeter);
    }

    #[test]
    fn test_conductivity_times_length() {
        let sigma = Conductivity::siemens_per_meter(5.96e7);
        let length = Length::meters(10.0);
        let g = sigma * length;
        assert!((g.to_siemens() - 5.96e8).abs() < 1e2);
    }
}
