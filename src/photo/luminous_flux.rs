//! Luminous flux quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};
use std::ops::{Div, Mul};

/// Units of luminous flux measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LuminousFluxUnit {
    /// Lumens (lm) - SI unit
    Lumens,
}

impl LuminousFluxUnit {
    /// All available luminous flux units.
    pub const ALL: &'static [LuminousFluxUnit] = &[LuminousFluxUnit::Lumens];
}

impl_unit_display!(LuminousFluxUnit);

impl UnitOfMeasure for LuminousFluxUnit {
    fn symbol(&self) -> &'static str {
        match self {
            LuminousFluxUnit::Lumens => "lm",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            LuminousFluxUnit::Lumens => 1.0,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, LuminousFluxUnit::Lumens)
    }
}

/// A quantity of luminous flux.
///
/// Luminous flux represents the total amount of visible light emitted by a source,
/// weighted by the sensitivity of the human eye.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let flux = LuminousFlux::lumens(1000.0);
/// let area = Area::square_meters(10.0);
///
/// // LuminousFlux / Area = Illuminance
/// let illuminance = flux / area;
/// assert!((illuminance.to_lux() - 100.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct LuminousFlux {
    value: f64,
    unit: LuminousFluxUnit,
}

impl LuminousFlux {
    /// Creates a new LuminousFlux quantity.
    pub const fn new_const(value: f64, unit: LuminousFluxUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a LuminousFlux in lumens.
    pub fn lumens(value: f64) -> Self {
        Self::new(value, LuminousFluxUnit::Lumens)
    }

    // Conversion methods
    /// Converts to lumens.
    pub fn to_lumens(&self) -> f64 {
        self.to(LuminousFluxUnit::Lumens)
    }
}

impl_quantity!(LuminousFlux, LuminousFluxUnit);

// Cross-quantity operations
use super::illuminance::{Illuminance, IlluminanceUnit};
use super::luminous_energy::{LuminousEnergy, LuminousEnergyUnit};
use super::luminous_intensity::{LuminousIntensity, LuminousIntensityUnit};
use crate::space::{Area, AreaUnit, SolidAngle, SolidAngleUnit};
use crate::time::Time;

// LuminousFlux / Area = Illuminance
impl Div<Area> for LuminousFlux {
    type Output = Illuminance;

    fn div(self, rhs: Area) -> Self::Output {
        let lux = self.to_lumens() / rhs.to_square_meters();
        Illuminance::new(lux, IlluminanceUnit::Lux)
    }
}

// LuminousFlux / Illuminance = Area
impl Div<Illuminance> for LuminousFlux {
    type Output = Area;

    fn div(self, rhs: Illuminance) -> Self::Output {
        let m2 = self.to_lumens() / rhs.to_lux();
        Area::new(m2, AreaUnit::SquareMeters)
    }
}

// LuminousFlux / SolidAngle = LuminousIntensity
impl Div<SolidAngle> for LuminousFlux {
    type Output = LuminousIntensity;

    fn div(self, rhs: SolidAngle) -> Self::Output {
        let cd = self.to_lumens() / rhs.to_steradians();
        LuminousIntensity::new(cd, LuminousIntensityUnit::Candelas)
    }
}

// LuminousFlux / LuminousIntensity = SolidAngle
impl Div<LuminousIntensity> for LuminousFlux {
    type Output = SolidAngle;

    fn div(self, rhs: LuminousIntensity) -> Self::Output {
        let sr = self.to_lumens() / rhs.to_candelas();
        SolidAngle::new(sr, SolidAngleUnit::Steradians)
    }
}

// LuminousFlux * Time = LuminousEnergy
impl Mul<Time> for LuminousFlux {
    type Output = LuminousEnergy;

    fn mul(self, rhs: Time) -> Self::Output {
        let lm_s = self.to_lumens() * rhs.to_seconds();
        LuminousEnergy::new(lm_s, LuminousEnergyUnit::LumenSeconds)
    }
}

// Time * LuminousFlux = LuminousEnergy
impl Mul<LuminousFlux> for Time {
    type Output = LuminousEnergy;

    fn mul(self, rhs: LuminousFlux) -> Self::Output {
        let lm_s = rhs.to_lumens() * self.to_seconds();
        LuminousEnergy::new(lm_s, LuminousEnergyUnit::LumenSeconds)
    }
}

impl_dimension!(
    LuminousFluxDimension,
    LuminousFlux,
    LuminousFluxUnit,
    "LuminousFlux",
    LuminousFluxUnit::Lumens,
    LuminousFluxUnit::Lumens
);

/// Extension trait for creating LuminousFlux quantities from numeric types.
pub trait LuminousFluxConversions {
    /// Creates a LuminousFlux in lumens.
    fn lumens(self) -> LuminousFlux;
}

impl LuminousFluxConversions for f64 {
    fn lumens(self) -> LuminousFlux {
        LuminousFlux::lumens(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luminous_flux_creation() {
        let f = LuminousFlux::lumens(1000.0);
        assert_eq!(f.value(), 1000.0);
        assert_eq!(f.unit(), LuminousFluxUnit::Lumens);
    }

    #[test]
    fn test_luminous_flux_divided_by_area() {
        let f = LuminousFlux::lumens(500.0);
        let a = Area::square_meters(5.0);
        let i = f / a;
        // 500 lm / 5 m² = 100 lx
        assert!((i.to_lux() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_luminous_flux_divided_by_solid_angle() {
        let f = LuminousFlux::lumens(200.0);
        let sa = SolidAngle::steradians(4.0);
        let i = f / sa;
        // 200 lm / 4 sr = 50 cd
        assert!((i.to_candelas() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_luminous_flux_times_time() {
        let f = LuminousFlux::lumens(100.0);
        let t = Time::seconds(5.0);
        let e = f * t;
        // 100 lm * 5 s = 500 lm·s
        assert!((e.to_lumen_seconds() - 500.0).abs() < 1e-10);
    }
}
