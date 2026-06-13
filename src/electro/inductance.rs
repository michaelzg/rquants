//! Inductance quantity and units.
use crate::core::Quantity;
use std::ops::Mul;
crate::quantity! {
    /// A quantity of inductance.
    ///
    /// Inductance is the property of an electrical conductor by which a change in current
    /// through it induces an electromotive force in both the conductor itself and in any
    /// nearby conductors by mutual inductance.
    ///
    /// # Relationships
    ///
    /// - Inductance × Current = MagneticFlux (Φ = LI)
    /// - Inductance = MagneticFlux / Current (L = Φ/I)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let inductance = Inductance::millihenrys(50.0);
    /// let current = ElectricCurrent::amperes(2.0);
    ///
    /// // MagneticFlux = Inductance × Current
    /// let flux = inductance * current;
    /// assert!((flux.to_webers() - 0.1).abs() < 1e-10);
    /// ```
    pub quantity Inductance {
        unit: InductanceUnit;
        dimension: InductanceDimension;
        conversions: InductanceConversions;
        name: "Inductance";
        primary: Henrys;
        si: Henrys;

        units {
            /// Henrys (H) - SI unit
            Henrys {
                symbol: "H",
                factor: 1.0,
                ctor: henrys,
                to: to_henrys,
                si: true
            },
            /// Microhenrys (µH)
            Microhenrys {
                symbol: "µH",
                factor: 1e-6,
                ctor: microhenrys,
                to: to_microhenrys,
                si: true
            },
            /// Millihenrys (mH)
            Millihenrys {
                symbol: "mH",
                factor: 1e-3,
                ctor: millihenrys,
                to: to_millihenrys,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use super::electric_current::ElectricCurrent;
use super::magnetic_flux::{MagneticFlux, MagneticFluxUnit};

// Inductance * Current = MagneticFlux (Φ = LI)
impl Mul<ElectricCurrent> for Inductance {
    type Output = MagneticFlux;

    fn mul(self, rhs: ElectricCurrent) -> Self::Output {
        let webers = self.to_henrys() * rhs.to_amperes();
        MagneticFlux::new(webers, MagneticFluxUnit::Webers)
    }
}

// Current * Inductance = MagneticFlux
impl Mul<Inductance> for ElectricCurrent {
    type Output = MagneticFlux;

    fn mul(self, rhs: Inductance) -> Self::Output {
        let webers = self.to_amperes() * rhs.to_henrys();
        MagneticFlux::new(webers, MagneticFluxUnit::Webers)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_inductance_creation() {
        let l = Inductance::henrys(1.0);
        assert_eq!(l.value(), 1.0);
        assert_eq!(l.unit(), InductanceUnit::Henrys);
    }

    #[test]
    fn test_inductance_conversions() {
        let l = Inductance::millihenrys(1.0);
        assert_eq!(l.to_henrys(), 0.001);

        let l2 = Inductance::microhenrys(1000.0);
        assert!((l2.to_millihenrys() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_inductance_times_current() {
        let l = Inductance::millihenrys(50.0);
        let i = ElectricCurrent::amperes(2.0);
        let flux = l * i;
        // 50 mH * 2 A = 0.1 Wb
        assert!((flux.to_webers() - 0.1).abs() < 1e-10);
    }
}
