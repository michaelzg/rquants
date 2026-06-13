//! Magnetic flux quantity and units.
use crate::core::Quantity;
use std::ops::Div;
crate::quantity! {
    /// A quantity of magnetic flux.
    ///
    /// Magnetic flux is a measure of the total magnetic field passing through a given area.
    /// It is defined as the surface integral of the magnetic field.
    /// Φ = ∫B·dA
    ///
    /// # Relationships
    ///
    /// - MagneticFlux / Area = MagneticFluxDensity (B = Φ/A)
    /// - MagneticFlux / Current = Inductance (L = Φ/I)
    /// - MagneticFlux / Time = Potential (V = dΦ/dt, Faraday's law)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let flux = MagneticFlux::webers(0.5);
    /// let area = Area::square_meters(2.0);
    ///
    /// // Magnetic flux density = Flux / Area
    /// let density = flux / area;
    /// assert!((density.to_teslas() - 0.25).abs() < 1e-10);
    /// ```
    pub quantity MagneticFlux {
        unit: MagneticFluxUnit;
        dimension: MagneticFluxDimension;
        conversions: MagneticFluxConversions;
        name: "MagneticFlux";
        primary: Webers;
        si: Webers;

        units {
            /// Webers (Wb) - SI unit
            Webers {
                symbol: "Wb",
                factor: 1.0,
                ctor: webers,
                to: to_webers,
                si: true
            }
        }
    }
}
// Cross-quantity operations
use super::electric_current::{ElectricCurrent, ElectricCurrentUnit};
use super::electric_potential::{ElectricPotential, ElectricPotentialUnit};
use super::inductance::{Inductance, InductanceUnit};
use super::magnetic_flux_density::{MagneticFluxDensity, MagneticFluxDensityUnit};
use crate::space::{Area, AreaUnit};
use crate::time::{Time, TimeUnit};

// MagneticFlux / Area = MagneticFluxDensity (B = Φ/A)
impl Div<Area> for MagneticFlux {
    type Output = MagneticFluxDensity;

    fn div(self, rhs: Area) -> Self::Output {
        let teslas = self.to_webers() / rhs.to_square_meters();
        MagneticFluxDensity::new(teslas, MagneticFluxDensityUnit::Teslas)
    }
}

// MagneticFlux / MagneticFluxDensity = Area (A = Φ/B)
impl Div<MagneticFluxDensity> for MagneticFlux {
    type Output = Area;

    fn div(self, rhs: MagneticFluxDensity) -> Self::Output {
        let m2 = self.to_webers() / rhs.to_teslas();
        Area::new(m2, AreaUnit::SquareMeters)
    }
}

// MagneticFlux / Current = Inductance (L = Φ/I)
impl Div<ElectricCurrent> for MagneticFlux {
    type Output = Inductance;

    fn div(self, rhs: ElectricCurrent) -> Self::Output {
        let henrys = self.to_webers() / rhs.to_amperes();
        Inductance::new(henrys, InductanceUnit::Henrys)
    }
}

// MagneticFlux / Inductance = Current (I = Φ/L)
impl Div<Inductance> for MagneticFlux {
    type Output = ElectricCurrent;

    fn div(self, rhs: Inductance) -> Self::Output {
        let amperes = self.to_webers() / rhs.to_henrys();
        ElectricCurrent::new(amperes, ElectricCurrentUnit::Amperes)
    }
}

// MagneticFlux / Time = Potential (V = dΦ/dt, Faraday's law)
impl Div<Time> for MagneticFlux {
    type Output = ElectricPotential;

    fn div(self, rhs: Time) -> Self::Output {
        let volts = self.to_webers() / rhs.to_seconds();
        ElectricPotential::new(volts, ElectricPotentialUnit::Volts)
    }
}

// MagneticFlux / Potential = Time
impl Div<ElectricPotential> for MagneticFlux {
    type Output = Time;

    fn div(self, rhs: ElectricPotential) -> Self::Output {
        let seconds = self.to_webers() / rhs.to_volts();
        Time::new(seconds, TimeUnit::Seconds)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_flux_creation() {
        let flux = MagneticFlux::webers(0.5);
        assert_eq!(flux.value(), 0.5);
        assert_eq!(flux.unit(), MagneticFluxUnit::Webers);
    }

    #[test]
    fn test_flux_divided_by_area() {
        let flux = MagneticFlux::webers(0.5);
        let area = Area::square_meters(2.0);
        let density = flux / area;
        assert!((density.to_teslas() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn test_flux_divided_by_current() {
        let flux = MagneticFlux::webers(0.1);
        let current = ElectricCurrent::amperes(2.0);
        let inductance = flux / current;
        assert!((inductance.to_henrys() - 0.05).abs() < 1e-10);
    }

    #[test]
    fn test_flux_divided_by_time() {
        let flux = MagneticFlux::webers(10.0);
        let time = Time::seconds(5.0);
        let voltage = flux / time;
        assert!((voltage.to_volts() - 2.0).abs() < 1e-10);
    }
}
