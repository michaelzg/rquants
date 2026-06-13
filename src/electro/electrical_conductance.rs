//! Electrical conductance quantity and units.
use crate::core::Quantity;
use std::ops::Div;
crate::quantity! {
    /// A quantity of electrical conductance.
    ///
    /// Electrical conductance is the reciprocal of electrical resistance,
    /// representing the ease with which electric current flows through a conductor.
    /// G = 1 / R (conductance = 1 / resistance)
    ///
    /// # Relationships
    ///
    /// - Conductance = 1 / Resistance
    /// - Resistance = 1 / Conductance
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let conductance = ElectricalConductance::siemens(0.01);
    /// let resistance = ElectricalResistance::ohms(100.0);
    ///
    /// // Conductance = 1 / Resistance
    /// assert!((conductance.to_siemens() - 1.0 / resistance.to_ohms()).abs() < 1e-10);
    /// ```
    pub quantity ElectricalConductance {
        unit: ElectricalConductanceUnit;
        dimension: ElectricalConductanceDimension;
        conversions: ElectricalConductanceConversions;
        name: "ElectricalConductance";
        primary: Siemens;
        si: Siemens;

        units {
            /// Siemens (S) - SI unit
            Siemens {
                symbol: "S",
                factor: 1.0,
                ctor: siemens,
                to: to_siemens,
                si: true
            },
            /// Millisiemens (mS)
            Millisiemens {
                symbol: "mS",
                factor: 1e-3,
                ctor: millisiemens,
                to: to_millisiemens,
                si: true
            },
            /// Microsiemens (µS)
            Microsiemens {
                symbol: "µS",
                factor: 1e-6,
                ctor: microsiemens,
                to: to_microsiemens,
                si: true
            }
        }
    }
}
impl ElectricalConductance {
    /// Creates an ElectricalConductance from resistance (G = 1/R).
    pub fn from_resistance(resistance: super::electrical_resistance::ElectricalResistance) -> Self {
        let siemens = 1.0 / resistance.to_ohms();
        Self::new(siemens, ElectricalConductanceUnit::Siemens)
    }

    /// Converts to resistance (R = 1/G).
    pub fn to_resistance(&self) -> super::electrical_resistance::ElectricalResistance {
        use super::electrical_resistance::{ElectricalResistance, ElectricalResistanceUnit};
        let ohms = 1.0 / self.to_siemens();
        ElectricalResistance::new(ohms, ElectricalResistanceUnit::Ohms)
    }
}
// Cross-quantity operations
use crate::space::Length;

// Conductance / Length = Conductivity
use super::conductivity::{Conductivity, ConductivityUnit};

impl Div<Length> for ElectricalConductance {
    type Output = Conductivity;

    fn div(self, rhs: Length) -> Self::Output {
        let s_per_m = self.to_siemens() / rhs.to_meters();
        Conductivity::new(s_per_m, ConductivityUnit::SiemensPerMeter)
    }
}
#[cfg(test)]
mod tests {
    use super::super::electrical_resistance::ElectricalResistance;
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_conductance_creation() {
        let g = ElectricalConductance::siemens(0.01);
        assert_eq!(g.value(), 0.01);
        assert_eq!(g.unit(), ElectricalConductanceUnit::Siemens);
    }

    #[test]
    fn test_conductance_conversions() {
        let g = ElectricalConductance::siemens(1.0);
        assert_eq!(g.to_millisiemens(), 1000.0);

        let g2 = ElectricalConductance::millisiemens(500.0);
        assert_eq!(g2.to_siemens(), 0.5);
    }

    #[test]
    fn test_conductance_from_resistance() {
        let r = ElectricalResistance::ohms(100.0);
        let g = ElectricalConductance::from_resistance(r);
        assert_eq!(g.to_siemens(), 0.01);
    }

    #[test]
    fn test_conductance_to_resistance() {
        let g = ElectricalConductance::siemens(0.01);
        let r = g.to_resistance();
        assert_eq!(r.to_ohms(), 100.0);
    }
}
