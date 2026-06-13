//! Moment of inertia (rotational inertia) quantity and units.

use super::mass::{Mass, MassUnit};
use crate::core::Quantity;
use crate::space::area::Area;
use crate::space::length::Length;
use std::ops::{Div, Mul};

// Conversion factors to kg·m² (primary unit)
// 1 lb = 0.45359237 kg, 1 ft = 0.3048 m
// 1 lb·ft² = 0.45359237 * 0.3048² kg·m² ≈ 0.0421401 kg·m²
const LB_FT2_FACTOR: f64 = 0.45359237 * 0.3048 * 0.3048;
crate::quantity! {
    /// A quantity of moment of inertia (rotational inertia).
    ///
    /// Moment of inertia represents the resistance of an object to rotational
    /// acceleration about an axis. It depends on both the mass and its
    /// distribution relative to the axis.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// // Moment of inertia of a point mass at a given radius
    /// let mass = Mass::kilograms(2.0);
    /// let radius = Length::meters(3.0);
    /// let inertia = mass.on_radius(radius);
    ///
    /// // I = m * r² = 2 * 9 = 18 kg·m²
    /// assert!((inertia.to_kilogram_meters_squared() - 18.0).abs() < 1e-10);
    /// ```
    pub quantity MomentOfInertia {
        unit: MomentOfInertiaUnit;
        dimension: MomentOfInertiaDimension;
        conversions: MomentOfInertiaConversions;
        name: "MomentOfInertia";
        primary: KilogramMetersSquared;
        si: KilogramMetersSquared;

        units {
            /// Kilogram-meters squared (kg·m²) - SI unit
            KilogramMetersSquared {
                symbol: "kg·m²",
                factor: 1.0,
                ctor: kilogram_meters_squared,
                to: to_kilogram_meters_squared,
                si: true
            },
            /// Pound-feet squared (lb·ft²)
            PoundFeetSquared {
                symbol: "lb·ft²",
                factor: LB_FT2_FACTOR,
                ctor: pound_feet_squared,
                to: to_pound_feet_squared,
                si: false
            }
        }
    }
}
impl MomentOfInertia {
    /// Returns the mass of a point mass at the given radius that would
    /// have this moment of inertia.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let inertia = MomentOfInertia::kilogram_meters_squared(18.0);
    /// let radius = Length::meters(3.0);
    /// let mass = inertia.at_center(radius);
    ///
    /// // m = I / r² = 18 / 9 = 2 kg
    /// assert!((mass.to_kilograms() - 2.0).abs() < 1e-10);
    /// ```
    pub fn at_center(&self, radius: Length) -> Mass {
        let radius_sq = radius.squared();
        let mass_kg = self.to_kilogram_meters_squared() / radius_sq.to_square_meters();
        Mass::new(mass_kg, MassUnit::Kilograms)
    }
}

// MomentOfInertia / Area = Mass
impl Div<Area> for MomentOfInertia {
    type Output = Mass;

    fn div(self, rhs: Area) -> Self::Output {
        let mass_kg = self.to_kilogram_meters_squared() / rhs.to_square_meters();
        Mass::new(mass_kg, MassUnit::Kilograms)
    }
}

// Mass * Area = MomentOfInertia (approximation for I = m*r²)
impl Mul<Area> for Mass {
    type Output = MomentOfInertia;

    fn mul(self, rhs: Area) -> Self::Output {
        let inertia = self.to_kilograms() * rhs.to_square_meters();
        MomentOfInertia::new(inertia, MomentOfInertiaUnit::KilogramMetersSquared)
    }
}

/// Extension method for creating MomentOfInertia from Mass and Length.
impl Mass {
    /// Creates a MomentOfInertia for a point mass at the given radius.
    ///
    /// I = m * r²
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let mass = Mass::kilograms(5.0);
    /// let radius = Length::meters(2.0);
    /// let inertia = mass.on_radius(radius);
    ///
    /// assert!((inertia.to_kilogram_meters_squared() - 20.0).abs() < 1e-10);
    /// ```
    pub fn on_radius(&self, radius: Length) -> MomentOfInertia {
        let radius_sq = radius.squared();
        let inertia = self.to_kilograms() * radius_sq.to_square_meters();
        MomentOfInertia::new(inertia, MomentOfInertiaUnit::KilogramMetersSquared)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_moment_of_inertia_creation() {
        let i = MomentOfInertia::kilogram_meters_squared(1.0);
        assert_eq!(i.value(), 1.0);
        assert_eq!(i.unit(), MomentOfInertiaUnit::KilogramMetersSquared);
    }

    #[test]
    fn test_moment_of_inertia_conversions() {
        let i = MomentOfInertia::pound_feet_squared(1.0);
        // 1 lb·ft² ≈ 0.0421401 kg·m²
        assert!((i.to_kilogram_meters_squared() - 0.0421401).abs() < 0.0001);
    }

    #[test]
    fn test_mass_on_radius() {
        let mass = Mass::kilograms(2.0);
        let radius = Length::meters(3.0);
        let inertia = mass.on_radius(radius);
        // I = m * r² = 2 * 9 = 18 kg·m²
        assert!((inertia.to_kilogram_meters_squared() - 18.0).abs() < 1e-10);
    }

    #[test]
    fn test_at_center() {
        let inertia = MomentOfInertia::kilogram_meters_squared(18.0);
        let radius = Length::meters(3.0);
        let mass = inertia.at_center(radius);
        // m = I / r² = 18 / 9 = 2 kg
        assert!((mass.to_kilograms() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_mass_times_area() {
        let mass = Mass::kilograms(2.0);
        let area = Area::square_meters(9.0);
        let inertia = mass * area;
        assert!((inertia.to_kilogram_meters_squared() - 18.0).abs() < 1e-10);
    }

    #[test]
    fn test_inertia_divided_by_area() {
        let inertia = MomentOfInertia::kilogram_meters_squared(18.0);
        let area = Area::square_meters(9.0);
        let mass = inertia / area;
        assert!((mass.to_kilograms() - 2.0).abs() < 1e-10);
    }
}
