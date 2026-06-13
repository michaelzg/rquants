//! Resistivity quantity and units.
use crate::core::Quantity;
use std::ops::Div;
crate::quantity! {
    /// A quantity of resistivity.
    ///
    /// Resistivity is an intrinsic property of a material that quantifies how strongly
    /// it resists electric current. It is the inverse of conductivity.
    /// ρ = R × A / L (resistivity = resistance × area / length)
    ///
    /// # Relationships
    ///
    /// - Resistivity = Resistance × Length
    /// - Resistivity / Length = Resistance
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let resistivity = Resistivity::ohm_meters(1.68e-8); // Copper
    /// let length = Length::meters(10.0);
    ///
    /// // Resistance = Resistivity / Length
    /// let resistance = resistivity / length;
    /// assert!((resistance.to_ohms() - 1.68e-9).abs() < 1e-15);
    /// ```
    pub quantity Resistivity {
        unit: ResistivityUnit;
        dimension: ResistivityDimension;
        conversions: ResistivityConversions;
        name: "Resistivity";
        primary: OhmMeters;
        si: OhmMeters;

        units {
            /// Ohm-meters (Ω·m) - SI unit
            OhmMeters {
                symbol: "Ω·m",
                factor: 1.0,
                ctor: ohm_meters,
                to: to_ohm_meters,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use super::electrical_resistance::{ElectricalResistance, ElectricalResistanceUnit};
use crate::space::Length;

// Resistivity / Length = Resistance
impl Div<Length> for Resistivity {
    type Output = ElectricalResistance;

    fn div(self, rhs: Length) -> Self::Output {
        let ohms = self.to_ohm_meters() / rhs.to_meters();
        ElectricalResistance::new(ohms, ElectricalResistanceUnit::Ohms)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_resistivity_creation() {
        let rho = Resistivity::ohm_meters(1.68e-8);
        assert_eq!(rho.value(), 1.68e-8);
        assert_eq!(rho.unit(), ResistivityUnit::OhmMeters);
    }

    #[test]
    fn test_resistivity_divided_by_length() {
        let rho = Resistivity::ohm_meters(1.68e-8);
        let length = Length::meters(10.0);
        let r = rho / length;
        assert!((r.to_ohms() - 1.68e-9).abs() < 1e-15);
    }
}
