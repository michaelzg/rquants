//! Irradiance quantity and units.
use crate::core::Quantity;
use std::ops::{Div, Mul};
crate::quantity! {
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
    pub quantity Irradiance {
        unit: IrradianceUnit;
        dimension: IrradianceDimension;
        conversions: IrradianceConversions;
        name: "Irradiance";
        primary: WattsPerSquareMeter;
        si: WattsPerSquareMeter;

        units {
            /// Watts per square meter (W/m²) - SI unit
            WattsPerSquareMeter {
                symbol: "W/m²",
                factor: 1.0,
                ctor: watts_per_square_meter,
                to: to_watts_per_square_meter,
                si: true
            }
        }
    }
}
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

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
