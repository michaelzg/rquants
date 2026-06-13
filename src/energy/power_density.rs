//! Power density quantity and units.
use crate::core::Quantity;
use std::ops::Mul;
crate::quantity! {
    /// A quantity of power density (power per unit volume).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let pd = PowerDensity::watts_per_cubic_meter(500.0);
    /// let volume = Volume::cubic_meters(2.0);
    ///
    /// // Power = PowerDensity * Volume
    /// let power = pd * volume;
    /// assert!((power.to_watts() - 1000.0).abs() < 1e-10);
    /// ```
    pub quantity PowerDensity {
        unit: PowerDensityUnit;
        dimension: PowerDensityDimension;
        conversions: PowerDensityConversions;
        name: "PowerDensity";
        primary: WattsPerCubicMeter;
        si: WattsPerCubicMeter;

        units {
            /// Watts per cubic meter (W/m³) - SI unit
            WattsPerCubicMeter {
                symbol: "W/m³",
                factor: 1.0,
                ctor: watts_per_cubic_meter,
                to: to_watts_per_cubic_meter,
                si: false
            }
        }
    }
}
// Cross-quantity operations
use super::power::{Power, PowerUnit};
use crate::space::Volume;

// PowerDensity * Volume = Power
impl Mul<Volume> for PowerDensity {
    type Output = Power;

    fn mul(self, rhs: Volume) -> Self::Output {
        let watts = self.to_watts_per_cubic_meter() * rhs.to_cubic_meters();
        Power::new(watts, PowerUnit::Watts)
    }
}

// Volume * PowerDensity = Power
impl Mul<PowerDensity> for Volume {
    type Output = Power;

    fn mul(self, rhs: PowerDensity) -> Self::Output {
        let watts = rhs.to_watts_per_cubic_meter() * self.to_cubic_meters();
        Power::new(watts, PowerUnit::Watts)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_power_density_creation() {
        let pd = PowerDensity::watts_per_cubic_meter(100.0);
        assert_eq!(pd.value(), 100.0);
        assert_eq!(pd.unit(), PowerDensityUnit::WattsPerCubicMeter);
    }

    #[test]
    fn test_power_density_times_volume() {
        let pd = PowerDensity::watts_per_cubic_meter(500.0);
        let v = Volume::cubic_meters(2.0);
        let p = pd * v;
        assert!((p.to_watts() - 1000.0).abs() < 1e-10);
    }
}
