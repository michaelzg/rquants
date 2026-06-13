//! Chemical amount (substance) quantity and units.

// Conversion factors to moles (primary unit)
// 1 lb-mol = 453.59237 mol (same as pounds to grams ratio)
const POUND_MOLE_FACTOR: f64 = 453.59237;
crate::quantity! {
    /// A quantity of chemical amount (amount of substance).
    ///
    /// Chemical amount, measured in moles, represents the amount of a substance
    /// containing as many elementary entities as there are atoms in 12 grams
    /// of carbon-12 (Avogadro's number).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let amount = ChemicalAmount::moles(2.0);
    /// let amount_lb = ChemicalAmount::pound_moles(1.0);
    ///
    /// // 1 pound-mole ≈ 453.59 moles
    /// assert!((amount_lb.to_moles() - 453.59237).abs() < 0.001);
    /// ```
    pub quantity ChemicalAmount {
        unit: ChemicalAmountUnit;
        dimension: ChemicalAmountDimension;
        conversions: ChemicalAmountConversions;
        name: "ChemicalAmount";
        primary: Moles;
        si: Moles;

        units {
            /// Moles (mol) - SI base unit
            Moles {
                symbol: "mol",
                factor: 1.0,
                ctor: moles,
                to: to_moles,
                si: true
            },
            /// Pound-moles (lb-mol)
            PoundMoles {
                symbol: "lb-mol",
                factor: POUND_MOLE_FACTOR,
                ctor: pound_moles,
                to: to_pound_moles,
                si: false
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_chemical_amount_creation() {
        let n = ChemicalAmount::moles(1.0);
        assert_eq!(n.value(), 1.0);
        assert_eq!(n.unit(), ChemicalAmountUnit::Moles);
    }

    #[test]
    fn test_chemical_amount_conversions() {
        let n = ChemicalAmount::pound_moles(1.0);
        assert!((n.to_moles() - 453.59237).abs() < 0.001);

        let n2 = ChemicalAmount::moles(453.59237);
        assert!((n2.to_pound_moles() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_chemical_amount_arithmetic() {
        let n1 = ChemicalAmount::moles(2.0);
        let n2 = ChemicalAmount::moles(3.0);
        let sum = n1 + n2;
        assert_eq!(sum.to_moles(), 5.0);
    }
}
