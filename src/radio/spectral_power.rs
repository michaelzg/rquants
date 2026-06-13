//! Spectral power quantity and units.
use crate::core::Quantity;
use std::ops::{Div, Mul};
crate::quantity! {
    /// A quantity of spectral power.
    ///
    /// Spectral power represents power per unit wavelength.
    /// SI unit: W/m
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let sp = SpectralPower::watts_per_meter(50.0);
    /// let length = Length::meters(2.0);
    ///
    /// // SpectralPower * Length = Power
    /// let power = sp * length;
    /// assert_eq!(power.to_watts(), 100.0);
    /// ```
    pub quantity SpectralPower {
        unit: SpectralPowerUnit;
        dimension: SpectralPowerDimension;
        conversions: SpectralPowerConversions;
        name: "SpectralPower";
        primary: WattsPerMeter;
        si: WattsPerMeter;

        units {
            /// Watts per meter (W/m) - SI unit
            WattsPerMeter {
                symbol: "W/m",
                factor: 1.0,
                ctor: watts_per_meter,
                to: to_watts_per_meter,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use crate::energy::{Power, PowerUnit};
use crate::space::{Length, LengthUnit};

// SpectralPower * Length = Power
impl Mul<Length> for SpectralPower {
    type Output = Power;

    fn mul(self, rhs: Length) -> Self::Output {
        let watts = self.to_watts_per_meter() * rhs.to_meters();
        Power::new(watts, PowerUnit::Watts)
    }
}

// Length * SpectralPower = Power
impl Mul<SpectralPower> for Length {
    type Output = Power;

    fn mul(self, rhs: SpectralPower) -> Self::Output {
        let watts = rhs.to_watts_per_meter() * self.to_meters();
        Power::new(watts, PowerUnit::Watts)
    }
}

// SpectralPower / Power = 1/Length
// Power / SpectralPower = Length
impl Div<SpectralPower> for Power {
    type Output = Length;

    fn div(self, rhs: SpectralPower) -> Self::Output {
        let meters = self.to_watts() / rhs.to_watts_per_meter();
        Length::new(meters, LengthUnit::Meters)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_spectral_power_creation() {
        let sp = SpectralPower::watts_per_meter(50.0);
        assert_eq!(sp.value(), 50.0);
        assert_eq!(sp.unit(), SpectralPowerUnit::WattsPerMeter);
    }

    #[test]
    fn test_spectral_power_conversions() {
        let sp = SpectralPower::watts_per_meter(50.0);
        assert_eq!(sp.to_watts_per_meter(), 50.0);
    }

    #[test]
    fn test_spectral_power_times_length() {
        let sp = SpectralPower::watts_per_meter(50.0);
        let length = Length::meters(2.0);
        let power = sp * length;
        assert_eq!(power.to_watts(), 100.0);
    }

    #[test]
    fn test_power_divided_by_spectral_power() {
        let power = Power::watts(100.0);
        let sp = SpectralPower::watts_per_meter(50.0);
        let length = power / sp;
        assert_eq!(length.to_meters(), 2.0);
    }
}
