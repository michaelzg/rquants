//! Conductivity quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

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

impl fmt::Display for ConductivityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

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

impl fmt::Display for Conductivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Conductivity {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Conductivity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Conductivity {
    type Unit = ConductivityUnit;

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

impl Add for Conductivity {
    type Output = Conductivity;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Conductivity::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Conductivity {
    type Output = Conductivity;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Conductivity::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Conductivity {
    type Output = Conductivity;

    fn mul(self, rhs: f64) -> Self::Output {
        Conductivity::new(self.value * rhs, self.unit)
    }
}

impl Mul<Conductivity> for f64 {
    type Output = Conductivity;

    fn mul(self, rhs: Conductivity) -> Self::Output {
        Conductivity::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for Conductivity {
    type Output = Conductivity;

    fn div(self, rhs: f64) -> Self::Output {
        Conductivity::new(self.value / rhs, self.unit)
    }
}

impl Div<Conductivity> for Conductivity {
    type Output = f64;

    fn div(self, rhs: Conductivity) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Conductivity {
    type Output = Conductivity;

    fn neg(self) -> Self::Output {
        Conductivity::new(-self.value, self.unit)
    }
}

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

/// Dimension for Conductivity.
pub struct ConductivityDimension;

impl Dimension for ConductivityDimension {
    type Quantity = Conductivity;
    type Unit = ConductivityUnit;

    fn name() -> &'static str {
        "Conductivity"
    }

    fn primary_unit() -> Self::Unit {
        ConductivityUnit::SiemensPerMeter
    }

    fn si_unit() -> Self::Unit {
        ConductivityUnit::SiemensPerMeter
    }

    fn units() -> &'static [Self::Unit] {
        ConductivityUnit::ALL
    }
}

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
