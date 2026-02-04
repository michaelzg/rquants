//! Magnetic flux density quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Mul};

/// Units of magnetic flux density measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MagneticFluxDensityUnit {
    /// Teslas (T) - SI unit
    Teslas,
    /// Gauss (G)
    Gauss,
}

impl MagneticFluxDensityUnit {
    /// All available magnetic flux density units.
    pub const ALL: &'static [MagneticFluxDensityUnit] =
        &[MagneticFluxDensityUnit::Teslas, MagneticFluxDensityUnit::Gauss];
}

impl_unit_display!(MagneticFluxDensityUnit);

impl UnitOfMeasure for MagneticFluxDensityUnit {
    fn symbol(&self) -> &'static str {
        match self {
            MagneticFluxDensityUnit::Teslas => "T",
            MagneticFluxDensityUnit::Gauss => "G",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            MagneticFluxDensityUnit::Teslas => 1.0,
            // 1 T = 10,000 Gauss
            MagneticFluxDensityUnit::Gauss => 1e-4,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, MagneticFluxDensityUnit::Teslas)
    }
}

/// A quantity of magnetic flux density.
///
/// Magnetic flux density (also known as magnetic induction or magnetic field strength)
/// is the measure of the magnetic field strength at a point in space.
/// B = Φ / A (magnetic flux density = flux / area)
///
/// # Relationships
///
/// - MagneticFluxDensity × Area = MagneticFlux (Φ = B·A)
/// - MagneticFluxDensity = MagneticFlux / Area (B = Φ/A)
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let density = MagneticFluxDensity::teslas(0.5);
/// let area = Area::square_meters(2.0);
///
/// // Magnetic flux = Density × Area
/// let flux = density * area;
/// assert!((flux.to_webers() - 1.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct MagneticFluxDensity {
    value: f64,
    unit: MagneticFluxDensityUnit,
}

impl MagneticFluxDensity {
    /// Creates a new MagneticFluxDensity quantity.
    pub const fn new_const(value: f64, unit: MagneticFluxDensityUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a MagneticFluxDensity in teslas.
    pub fn teslas(value: f64) -> Self {
        Self::new(value, MagneticFluxDensityUnit::Teslas)
    }

    /// Creates a MagneticFluxDensity in gauss.
    pub fn gauss(value: f64) -> Self {
        Self::new(value, MagneticFluxDensityUnit::Gauss)
    }

    // Conversion methods
    /// Converts to teslas.
    pub fn to_teslas(&self) -> f64 {
        self.to(MagneticFluxDensityUnit::Teslas)
    }

    /// Converts to gauss.
    pub fn to_gauss(&self) -> f64 {
        self.to(MagneticFluxDensityUnit::Gauss)
    }
}

impl_quantity!(MagneticFluxDensity, MagneticFluxDensityUnit);

// Cross-quantity operations
use super::magnetic_flux::{MagneticFlux, MagneticFluxUnit};
use crate::space::Area;

// MagneticFluxDensity * Area = MagneticFlux (Φ = B·A)
impl Mul<Area> for MagneticFluxDensity {
    type Output = MagneticFlux;

    fn mul(self, rhs: Area) -> Self::Output {
        let webers = self.to_teslas() * rhs.to_square_meters();
        MagneticFlux::new(webers, MagneticFluxUnit::Webers)
    }
}

// Area * MagneticFluxDensity = MagneticFlux
impl Mul<MagneticFluxDensity> for Area {
    type Output = MagneticFlux;

    fn mul(self, rhs: MagneticFluxDensity) -> Self::Output {
        let webers = self.to_square_meters() * rhs.to_teslas();
        MagneticFlux::new(webers, MagneticFluxUnit::Webers)
    }
}

impl_dimension!(
    MagneticFluxDensityDimension,
    MagneticFluxDensity,
    MagneticFluxDensityUnit,
    "MagneticFluxDensity",
    MagneticFluxDensityUnit::Teslas,
    MagneticFluxDensityUnit::Teslas
);

/// Extension trait for creating MagneticFluxDensity quantities from numeric types.
pub trait MagneticFluxDensityConversions {
    /// Creates a MagneticFluxDensity in teslas.
    fn teslas(self) -> MagneticFluxDensity;
    /// Creates a MagneticFluxDensity in gauss.
    fn gauss(self) -> MagneticFluxDensity;
}

impl MagneticFluxDensityConversions for f64 {
    fn teslas(self) -> MagneticFluxDensity {
        MagneticFluxDensity::teslas(self)
    }
    fn gauss(self) -> MagneticFluxDensity {
        MagneticFluxDensity::gauss(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_creation() {
        let b = MagneticFluxDensity::teslas(0.5);
        assert_eq!(b.value(), 0.5);
        assert_eq!(b.unit(), MagneticFluxDensityUnit::Teslas);
    }

    #[test]
    fn test_density_conversions() {
        let b = MagneticFluxDensity::teslas(1.0);
        assert_eq!(b.to_gauss(), 10000.0);

        let b2 = MagneticFluxDensity::gauss(5000.0);
        assert_eq!(b2.to_teslas(), 0.5);
    }

    #[test]
    fn test_density_times_area() {
        let b = MagneticFluxDensity::teslas(0.5);
        let area = Area::square_meters(2.0);
        let flux = b * area;
        assert!((flux.to_webers() - 1.0).abs() < 1e-10);
    }
}
