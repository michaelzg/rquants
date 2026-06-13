//! Activity quantity and units.

// Conversion factor
const CURIE_TO_BECQUEREL: f64 = 3.7e10;
crate::quantity! {
    /// A quantity of radioactivity.
    ///
    /// Activity represents the rate of radioactive decay.
    /// SI unit: Becquerel (Bq) = 1 decay per second
    /// 1 Ci = 3.7×10¹⁰ Bq
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let activity = Activity::becquerels(3.7e10);
    /// assert!((activity.to_curies() - 1.0).abs() < 1e-10);
    /// ```
    pub quantity Activity {
        unit: ActivityUnit;
        dimension: ActivityDimension;
        conversions: ActivityConversions;
        name: "Activity";
        primary: Becquerels;
        si: Becquerels;

        units {
            /// Becquerels (Bq) - SI unit
            Becquerels {
                symbol: "Bq",
                factor: 1.0,
                ctor: becquerels,
                to: to_becquerels,
                si: true
            },
            /// Curies (Ci)
            Curies {
                symbol: "Ci",
                factor: CURIE_TO_BECQUEREL,
                ctor: curies,
                to: to_curies,
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
    fn test_activity_creation() {
        let a = Activity::becquerels(1000.0);
        assert_eq!(a.value(), 1000.0);
        assert_eq!(a.unit(), ActivityUnit::Becquerels);
    }

    #[test]
    fn test_activity_conversions() {
        let a = Activity::curies(1.0);
        assert_eq!(a.to_becquerels(), 3.7e10);

        let b = Activity::becquerels(3.7e10);
        assert!((b.to_curies() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_activity_arithmetic() {
        let a1 = Activity::becquerels(1000.0);
        let a2 = Activity::becquerels(500.0);
        let sum = a1 + a2;
        assert_eq!(sum.to_becquerels(), 1500.0);
    }
}
