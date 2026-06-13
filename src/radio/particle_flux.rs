//! Particle flux quantity and units.

crate::quantity! {
    /// A quantity of particle flux.
    ///
    /// Particle flux represents the number of particles crossing a unit area per unit time.
    /// SI unit: Bq/(m²·s) = particles/(m²·s)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let flux = ParticleFlux::becquerels_per_square_meter_second(1000.0);
    /// assert_eq!(flux.to_becquerels_per_square_meter_second(), 1000.0);
    /// ```
    pub quantity ParticleFlux {
        unit: ParticleFluxUnit;
        dimension: ParticleFluxDimension;
        conversions: ParticleFluxConversions;
        name: "ParticleFlux";
        primary: BecquerelsPerSquareMeterSecond;
        si: BecquerelsPerSquareMeterSecond;

        units {
            /// Becquerels per square meter per second (Bq/(m²·s)) - SI unit
            BecquerelsPerSquareMeterSecond {
                symbol: "Bq/(m²·s)",
                factor: 1.0,
                ctor: becquerels_per_square_meter_second,
                to: to_becquerels_per_square_meter_second,
                si: true
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_particle_flux_creation() {
        let pf = ParticleFlux::becquerels_per_square_meter_second(1000.0);
        assert_eq!(pf.value(), 1000.0);
        assert_eq!(pf.unit(), ParticleFluxUnit::BecquerelsPerSquareMeterSecond);
    }

    #[test]
    fn test_particle_flux_conversions() {
        let pf = ParticleFlux::becquerels_per_square_meter_second(1000.0);
        assert_eq!(pf.to_becquerels_per_square_meter_second(), 1000.0);
    }

    #[test]
    fn test_particle_flux_arithmetic() {
        let pf1 = ParticleFlux::becquerels_per_square_meter_second(1000.0);
        let pf2 = ParticleFlux::becquerels_per_square_meter_second(500.0);
        let sum = pf1 + pf2;
        assert_eq!(sum.to_becquerels_per_square_meter_second(), 1500.0);
    }
}
