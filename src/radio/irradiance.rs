//! Irradiance quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Div, Mul};

/// Units of irradiance measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrradianceUnit {
    /// Watts per square meter (W/m²) - SI unit
    WattsPerSquareMeter,
}

impl IrradianceUnit {
    /// All available irradiance units.
    pub const ALL: &'static [IrradianceUnit] = &[IrradianceUnit::WattsPerSquareMeter];
}

impl_unit_display!(IrradianceUnit);

impl UnitOfMeasure for IrradianceUnit {
    fn symbol(&self) -> &'static str {
        match self {
            IrradianceUnit::WattsPerSquareMeter => "W/m²",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            IrradianceUnit::WattsPerSquareMeter => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, IrradianceUnit::WattsPerSquareMeter)
    }
}

/// A quantity of irradiance.
///
/// Irradiance represents power per unit area.
/// SI unit: W/m²
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let irr = Irradiance::watts_per_square_meter(1000.0);
/// let area = Area::square_meters(2.0);
///
/// // Irradiance * Area = Power
/// let power = irr * area;
/// assert_eq!(power.to_watts(), 2000.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Irradiance {
    value: f64,
    unit: IrradianceUnit,
}

impl Irradiance {
    /// Creates a new Irradiance quantity.
    pub const fn new_const(value: f64, unit: IrradianceUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates an Irradiance in watts per square meter.
    pub fn watts_per_square_meter(value: f64) -> Self {
        Self::new(value, IrradianceUnit::WattsPerSquareMeter)
    }

    // Conversion methods
    /// Converts to watts per square meter.
    pub fn to_watts_per_square_meter(&self) -> f64 {
        self.to(IrradianceUnit::WattsPerSquareMeter)
    }
}

impl_quantity!(Irradiance, IrradianceUnit);

// Cross-quantity operations
use crate::energy::{Power, PowerUnit};
use crate::space::{Area, AreaUnit};

// Irradiance * Area = Power
impl Mul<Area> for Irradiance {
    type Output = Power;

    fn mul(self, rhs: Area) -> Self::Output {
        let watts = self.to_watts_per_square_meter() * rhs.to_square_meters();
        Power::new(watts, PowerUnit::Watts)
    }
}

// Area * Irradiance = Power
impl Mul<Irradiance> for Area {
    type Output = Power;

    fn mul(self, rhs: Irradiance) -> Self::Output {
        let watts = rhs.to_watts_per_square_meter() * self.to_square_meters();
        Power::new(watts, PowerUnit::Watts)
    }
}

// Power / Irradiance = Area
impl Div<Irradiance> for Power {
    type Output = Area;

    fn div(self, rhs: Irradiance) -> Self::Output {
        let m2 = self.to_watts() / rhs.to_watts_per_square_meter();
        Area::new(m2, AreaUnit::SquareMeters)
    }
}

impl_dimension!(
    IrradianceDimension,
    Irradiance,
    IrradianceUnit,
    "Irradiance",
    IrradianceUnit::WattsPerSquareMeter,
    IrradianceUnit::WattsPerSquareMeter
);

/// Extension trait for creating Irradiance quantities from numeric types.
pub trait IrradianceConversions {
    /// Creates an Irradiance in watts per square meter.
    fn watts_per_square_meter(self) -> Irradiance;
}

impl IrradianceConversions for f64 {
    fn watts_per_square_meter(self) -> Irradiance {
        Irradiance::watts_per_square_meter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irradiance_creation() {
        let irr = Irradiance::watts_per_square_meter(1000.0);
        assert_eq!(irr.value(), 1000.0);
        assert_eq!(irr.unit(), IrradianceUnit::WattsPerSquareMeter);
    }

    #[test]
    fn test_irradiance_conversions() {
        let irr = Irradiance::watts_per_square_meter(1000.0);
        assert_eq!(irr.to_watts_per_square_meter(), 1000.0);
    }

    #[test]
    fn test_irradiance_times_area() {
        let irr = Irradiance::watts_per_square_meter(1000.0);
        let area = Area::square_meters(2.0);
        let power = irr * area;
        assert_eq!(power.to_watts(), 2000.0);
    }

    #[test]
    fn test_power_divided_by_irradiance() {
        let power = Power::watts(2000.0);
        let irr = Irradiance::watts_per_square_meter(1000.0);
        let area = power / irr;
        assert_eq!(area.to_square_meters(), 2.0);
    }
}
