//! Radiant intensity quantity and units.
use crate::core::Quantity;
use std::ops::{Div, Mul};
crate::quantity! {
    /// A quantity of radiant intensity.
    ///
    /// Radiant intensity represents power per unit solid angle.
    /// SI unit: W/sr
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let ri = RadiantIntensity::watts_per_steradian(100.0);
    /// let angle = SolidAngle::steradians(2.0);
    ///
    /// // RadiantIntensity * SolidAngle = Power
    /// let power = ri * angle;
    /// assert_eq!(power.to_watts(), 200.0);
    /// ```
    pub quantity RadiantIntensity {
        unit: RadiantIntensityUnit;
        dimension: RadiantIntensityDimension;
        conversions: RadiantIntensityConversions;
        name: "RadiantIntensity";
        primary: WattsPerSteradian;
        si: WattsPerSteradian;

        units {
            /// Watts per steradian (W/sr) - SI unit
            WattsPerSteradian {
                symbol: "W/sr",
                factor: 1.0,
                ctor: watts_per_steradian,
                to: to_watts_per_steradian,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use crate::energy::{Power, PowerUnit};
use crate::space::{SolidAngle, SolidAngleUnit};

// RadiantIntensity * SolidAngle = Power
impl Mul<SolidAngle> for RadiantIntensity {
    type Output = Power;

    fn mul(self, rhs: SolidAngle) -> Self::Output {
        let watts = self.to_watts_per_steradian() * rhs.to_steradians();
        Power::new(watts, PowerUnit::Watts)
    }
}

// SolidAngle * RadiantIntensity = Power
impl Mul<RadiantIntensity> for SolidAngle {
    type Output = Power;

    fn mul(self, rhs: RadiantIntensity) -> Self::Output {
        let watts = rhs.to_watts_per_steradian() * self.to_steradians();
        Power::new(watts, PowerUnit::Watts)
    }
}

// Power / RadiantIntensity = SolidAngle
impl Div<RadiantIntensity> for Power {
    type Output = SolidAngle;

    fn div(self, rhs: RadiantIntensity) -> Self::Output {
        let sr = self.to_watts() / rhs.to_watts_per_steradian();
        SolidAngle::new(sr, SolidAngleUnit::Steradians)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_radiant_intensity_creation() {
        let ri = RadiantIntensity::watts_per_steradian(100.0);
        assert_eq!(ri.value(), 100.0);
        assert_eq!(ri.unit(), RadiantIntensityUnit::WattsPerSteradian);
    }

    #[test]
    fn test_radiant_intensity_conversions() {
        let ri = RadiantIntensity::watts_per_steradian(100.0);
        assert_eq!(ri.to_watts_per_steradian(), 100.0);
    }

    #[test]
    fn test_radiant_intensity_times_solid_angle() {
        let ri = RadiantIntensity::watts_per_steradian(100.0);
        let angle = SolidAngle::steradians(2.0);
        let power = ri * angle;
        assert_eq!(power.to_watts(), 200.0);
    }

    #[test]
    fn test_power_divided_by_radiant_intensity() {
        let power = Power::watts(200.0);
        let ri = RadiantIntensity::watts_per_steradian(100.0);
        let angle = power / ri;
        assert_eq!(angle.to_steradians(), 2.0);
    }
}
