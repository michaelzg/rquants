//! Conductivity quantity and units.
use crate::core::Quantity;
use std::ops::Mul;
crate::quantity! {
    /// A quantity of conductivity.
    ///
    /// Conductivity is an intrinsic property of a material that quantifies how easily
    /// it conducts electric current. It is the inverse of resistivity.
    /// σ = 1/ρ (conductivity = 1/resistivity)
    ///
    /// # Relationships
    ///
    /// - Conductivity = Conductance / Length
    /// - Conductivity × Length = Conductance
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let conductivity = Conductivity::siemens_per_meter(5.96e7); // Copper
    /// let length = Length::meters(10.0);
    ///
    /// // Conductance = Conductivity × Length
    /// let conductance = conductivity * length;
    /// assert!((conductance.to_siemens() - 5.96e8).abs() < 1e2);
    /// ```
    pub quantity Conductivity {
        unit: ConductivityUnit;
        dimension: ConductivityDimension;
        conversions: ConductivityConversions;
        name: "Conductivity";
        primary: SiemensPerMeter;
        si: SiemensPerMeter;

        units {
            /// Siemens per meter (S/m) - SI unit
            SiemensPerMeter {
                symbol: "S/m",
                factor: 1.0,
                ctor: siemens_per_meter,
                to: to_siemens_per_meter,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use super::electrical_conductance::{ElectricalConductance, ElectricalConductanceUnit};
use crate::space::Length;

// Conductivity * Length = Conductance
impl Mul<Length> for Conductivity {
    type Output = ElectricalConductance;

    fn mul(self, rhs: Length) -> Self::Output {
        let siemens = self.to_siemens_per_meter() * rhs.to_meters();
        ElectricalConductance::new(siemens, ElectricalConductanceUnit::Siemens)
    }
}

// Length * Conductivity = Conductance
impl Mul<Conductivity> for Length {
    type Output = ElectricalConductance;

    fn mul(self, rhs: Conductivity) -> Self::Output {
        let siemens = self.to_meters() * rhs.to_siemens_per_meter();
        ElectricalConductance::new(siemens, ElectricalConductanceUnit::Siemens)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_conductivity_creation() {
        let sigma = Conductivity::siemens_per_meter(5.96e7);
        assert_eq!(sigma.value(), 5.96e7);
        assert_eq!(sigma.unit(), ConductivityUnit::SiemensPerMeter);
    }

    #[test]
    fn test_conductivity_times_length() {
        let sigma = Conductivity::siemens_per_meter(5.96e7);
        let length = Length::meters(10.0);
        let g = sigma * length;
        assert!((g.to_siemens() - 5.96e8).abs() < 1e2);
    }
}
