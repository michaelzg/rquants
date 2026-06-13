//! Luminous energy quantity and units.
use crate::core::Quantity;
use std::ops::Div;
crate::quantity! {
    /// A quantity of luminous energy.
    ///
    /// Luminous energy represents the time integral of luminous flux.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let energy = LuminousEnergy::lumen_seconds(1000.0);
    /// let time = Time::seconds(10.0);
    ///
    /// // LuminousEnergy / Time = LuminousFlux
    /// let flux = energy / time;
    /// assert!((flux.to_lumens() - 100.0).abs() < 1e-10);
    /// ```
    pub quantity LuminousEnergy {
        unit: LuminousEnergyUnit;
        dimension: LuminousEnergyDimension;
        conversions: LuminousEnergyConversions;
        name: "LuminousEnergy";
        primary: LumenSeconds;
        si: LumenSeconds;

        units {
            /// Lumen-seconds (lm·s) - SI unit
            LumenSeconds {
                symbol: "lm·s",
                factor: 1.0,
                ctor: lumen_seconds,
                to: to_lumen_seconds,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use super::luminous_flux::{LuminousFlux, LuminousFluxUnit};
use crate::time::{Time, TimeUnit};

// LuminousEnergy / Time = LuminousFlux
impl Div<Time> for LuminousEnergy {
    type Output = LuminousFlux;

    fn div(self, rhs: Time) -> Self::Output {
        let lumens = self.to_lumen_seconds() / rhs.to_seconds();
        LuminousFlux::new(lumens, LuminousFluxUnit::Lumens)
    }
}

// LuminousEnergy / LuminousFlux = Time
impl Div<LuminousFlux> for LuminousEnergy {
    type Output = Time;

    fn div(self, rhs: LuminousFlux) -> Self::Output {
        let seconds = self.to_lumen_seconds() / rhs.to_lumens();
        Time::new(seconds, TimeUnit::Seconds)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_luminous_energy_creation() {
        let e = LuminousEnergy::lumen_seconds(500.0);
        assert_eq!(e.value(), 500.0);
        assert_eq!(e.unit(), LuminousEnergyUnit::LumenSeconds);
    }

    #[test]
    fn test_luminous_energy_divided_by_time() {
        let e = LuminousEnergy::lumen_seconds(1000.0);
        let t = Time::seconds(10.0);
        let f = e / t;
        // 1000 lm·s / 10 s = 100 lm
        assert!((f.to_lumens() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_luminous_flux_times_time() {
        let f = LuminousFlux::lumens(50.0);
        let t = Time::seconds(20.0);
        let e = f * t;
        // 50 lm * 20 s = 1000 lm·s
        assert!((e.to_lumen_seconds() - 1000.0).abs() < 1e-10);
    }
}
