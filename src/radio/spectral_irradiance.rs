//! Spectral irradiance quantity and units.
crate::quantity! {
    /// A quantity of spectral irradiance.
    ///
    /// Spectral irradiance represents power per unit area per unit wavelength.
    /// SI unit: W/m³
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let si = SpectralIrradiance::watts_per_cubic_meter(100.0);
    /// assert_eq!(si.to_watts_per_cubic_meter(), 100.0);
    /// ```
    pub quantity SpectralIrradiance {
        unit: SpectralIrradianceUnit;
        dimension: SpectralIrradianceDimension;
        conversions: SpectralIrradianceConversions;
        name: "SpectralIrradiance";
        primary: WattsPerCubicMeter;
        si: WattsPerCubicMeter;

        units {
            /// Watts per cubic meter (W/m³) - SI unit
            WattsPerCubicMeter {
                symbol: "W/m³",
                factor: 1.0,
                ctor: watts_per_cubic_meter,
                to: to_watts_per_cubic_meter,
                si: true
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_spectral_irradiance_creation() {
        let si = SpectralIrradiance::watts_per_cubic_meter(100.0);
        assert_eq!(si.value(), 100.0);
        assert_eq!(si.unit(), SpectralIrradianceUnit::WattsPerCubicMeter);
    }

    #[test]
    fn test_spectral_irradiance_conversions() {
        let si = SpectralIrradiance::watts_per_cubic_meter(100.0);
        assert_eq!(si.to_watts_per_cubic_meter(), 100.0);
    }

    #[test]
    fn test_spectral_irradiance_arithmetic() {
        let si1 = SpectralIrradiance::watts_per_cubic_meter(100.0);
        let si2 = SpectralIrradiance::watts_per_cubic_meter(50.0);
        let sum = si1 + si2;
        assert_eq!(sum.to_watts_per_cubic_meter(), 150.0);
    }
}
