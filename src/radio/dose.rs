//! Dose quantity and units.

// Conversion factor
const SIEVERT_TO_REM: f64 = 100.0;
crate::quantity! {
    /// A quantity of radiation dose.
    ///
    /// Dose represents the equivalent or effective radiation dose to biological tissue.
    /// SI unit: Sievert (Sv)
    /// 1 Sv = 100 rem
    ///
    /// Note: This is different from absorbed dose (SpecificEnergy). Dose measures
    /// the biological effect of radiation, while absorbed dose measures the energy
    /// deposited per unit mass.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let dose = Dose::sieverts(1.0);
    /// assert_eq!(dose.to_rems(), 100.0);
    /// ```
    pub quantity Dose {
        unit: DoseUnit;
        dimension: DoseDimension;
        conversions: DoseConversions;
        name: "Dose";
        primary: Sieverts;
        si: Sieverts;

        units {
            /// Sieverts (Sv) - SI unit
            Sieverts {
                symbol: "Sv",
                factor: 1.0,
                ctor: sieverts,
                to: to_sieverts,
                si: true
            },
            /// Rems (rem)
            Rems {
                symbol: "rem",
                factor: 1.0 / SIEVERT_TO_REM,
                ctor: rems,
                to: to_rems,
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
    fn test_dose_creation() {
        let d = Dose::sieverts(1.0);
        assert_eq!(d.value(), 1.0);
        assert_eq!(d.unit(), DoseUnit::Sieverts);
    }

    #[test]
    fn test_dose_conversions() {
        let d = Dose::sieverts(1.0);
        assert_eq!(d.to_rems(), 100.0);

        let d2 = Dose::rems(100.0);
        assert!((d2.to_sieverts() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_dose_arithmetic() {
        let d1 = Dose::sieverts(1.0);
        let d2 = Dose::sieverts(0.5);
        let sum = d1 + d2;
        assert_eq!(sum.to_sieverts(), 1.5);
    }
}
