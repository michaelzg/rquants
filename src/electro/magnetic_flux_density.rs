//! Magnetic flux density quantity and units.
use crate::core::Quantity;
use std::ops::Mul;
crate::quantity! {
    /// A quantity of magnetic flux density.
    ///
    /// Magnetic flux density (also known as magnetic induction or magnetic field strength)
    /// is the measure of the magnetic field strength at a point in space.
    /// B = Φ / A (magnetic flux density = flux / area)
    ///
    /// # Relationships
    ///
    /// - MagneticFluxDensity × Area = MagneticFlux (Φ = B·A)
    /// - MagneticFluxDensity = MagneticFlux / Area (B = Φ/A)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let density = MagneticFluxDensity::teslas(0.5);
    /// let area = Area::square_meters(2.0);
    ///
    /// // Magnetic flux = Density × Area
    /// let flux = density * area;
    /// assert!((flux.to_webers() - 1.0).abs() < 1e-10);
    /// ```
    pub quantity MagneticFluxDensity {
        unit: MagneticFluxDensityUnit;
        dimension: MagneticFluxDensityDimension;
        conversions: MagneticFluxDensityConversions;
        name: "MagneticFluxDensity";
        primary: Teslas;
        si: Teslas;

        units {
            /// Teslas (T) - SI unit
            Teslas {
                symbol: "T",
                factor: 1.0,
                ctor: teslas,
                to: to_teslas,
                si: true
            },
            /// Gauss (G)
            Gauss {
                symbol: "G",
                factor: 1e-4,
                ctor: gauss,
                to: to_gauss,
                si: false
            }
        }
    }
}
// Cross-quantity operations
use super::magnetic_flux::{MagneticFlux, MagneticFluxUnit};
use crate::space::Area;

// MagneticFluxDensity * Area = MagneticFlux (Φ = B·A)
impl Mul<Area> for MagneticFluxDensity {
    type Output = MagneticFlux;

    fn mul(self, rhs: Area) -> Self::Output {
        let webers = self.to_teslas() * rhs.to_square_meters();
        MagneticFlux::new(webers, MagneticFluxUnit::Webers)
    }
}

// Area * MagneticFluxDensity = MagneticFlux
impl Mul<MagneticFluxDensity> for Area {
    type Output = MagneticFlux;

    fn mul(self, rhs: MagneticFluxDensity) -> Self::Output {
        let webers = self.to_square_meters() * rhs.to_teslas();
        MagneticFlux::new(webers, MagneticFluxUnit::Webers)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_density_creation() {
        let b = MagneticFluxDensity::teslas(0.5);
        assert_eq!(b.value(), 0.5);
        assert_eq!(b.unit(), MagneticFluxDensityUnit::Teslas);
    }

    #[test]
    fn test_density_conversions() {
        let b = MagneticFluxDensity::teslas(1.0);
        assert_eq!(b.to_gauss(), 10000.0);

        let b2 = MagneticFluxDensity::gauss(5000.0);
        assert_eq!(b2.to_teslas(), 0.5);
    }

    #[test]
    fn test_density_times_area() {
        let b = MagneticFluxDensity::teslas(0.5);
        let area = Area::square_meters(2.0);
        let flux = b * area;
        assert!((flux.to_webers() - 1.0).abs() < 1e-10);
    }
}
