//! Luminance quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Mul};

/// Units of luminance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LuminanceUnit {
    /// Candelas per square meter (cd/m²) - SI unit
    CandelasPerSquareMeter,
}

impl LuminanceUnit {
    /// All available luminance units.
    pub const ALL: &'static [LuminanceUnit] = &[LuminanceUnit::CandelasPerSquareMeter];
}

impl_unit_display!(LuminanceUnit);

impl UnitOfMeasure for LuminanceUnit {
    fn symbol(&self) -> &'static str {
        match self {
            LuminanceUnit::CandelasPerSquareMeter => "cd/m²",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            LuminanceUnit::CandelasPerSquareMeter => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, LuminanceUnit::CandelasPerSquareMeter)
    }
}

/// A quantity of luminance.
///
/// Luminance represents the luminous intensity per unit area of light traveling
/// in a given direction.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let luminance = Luminance::candelas_per_square_meter(1000.0);
/// let area = Area::square_meters(0.5);
///
/// // Luminance * Area = LuminousIntensity
/// let intensity = luminance * area;
/// assert!((intensity.to_candelas() - 500.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Luminance {
    value: f64,
    unit: LuminanceUnit,
}

impl Luminance {
    /// Creates a new Luminance quantity.
    pub const fn new_const(value: f64, unit: LuminanceUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a Luminance in candelas per square meter.
    pub fn candelas_per_square_meter(value: f64) -> Self {
        Self::new(value, LuminanceUnit::CandelasPerSquareMeter)
    }

    // Conversion methods
    /// Converts to candelas per square meter.
    pub fn to_candelas_per_square_meter(&self) -> f64 {
        self.to(LuminanceUnit::CandelasPerSquareMeter)
    }
}

impl_quantity!(Luminance, LuminanceUnit);

// Cross-quantity operations
use super::luminous_intensity::{LuminousIntensity, LuminousIntensityUnit};
use crate::space::Area;

// Luminance * Area = LuminousIntensity
impl Mul<Area> for Luminance {
    type Output = LuminousIntensity;

    fn mul(self, rhs: Area) -> Self::Output {
        let cd = self.to_candelas_per_square_meter() * rhs.to_square_meters();
        LuminousIntensity::new(cd, LuminousIntensityUnit::Candelas)
    }
}

// Area * Luminance = LuminousIntensity
impl Mul<Luminance> for Area {
    type Output = LuminousIntensity;

    fn mul(self, rhs: Luminance) -> Self::Output {
        let cd = rhs.to_candelas_per_square_meter() * self.to_square_meters();
        LuminousIntensity::new(cd, LuminousIntensityUnit::Candelas)
    }
}

impl_dimension!(
    LuminanceDimension,
    Luminance,
    LuminanceUnit,
    "Luminance",
    LuminanceUnit::CandelasPerSquareMeter,
    LuminanceUnit::CandelasPerSquareMeter
);

/// Extension trait for creating Luminance quantities from numeric types.
pub trait LuminanceConversions {
    /// Creates a Luminance in candelas per square meter.
    fn candelas_per_square_meter(self) -> Luminance;
}

impl LuminanceConversions for f64 {
    fn candelas_per_square_meter(self) -> Luminance {
        Luminance::candelas_per_square_meter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luminance_creation() {
        let l = Luminance::candelas_per_square_meter(5000.0);
        assert_eq!(l.value(), 5000.0);
        assert_eq!(l.unit(), LuminanceUnit::CandelasPerSquareMeter);
    }

    #[test]
    fn test_luminance_times_area() {
        let l = Luminance::candelas_per_square_meter(200.0);
        let a = Area::square_meters(3.0);
        let i = l * a;
        // 200 cd/m² * 3 m² = 600 cd
        assert!((i.to_candelas() - 600.0).abs() < 1e-10);
    }
}
