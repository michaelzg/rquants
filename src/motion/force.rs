//! Force quantity and units.

use super::acceleration::{Acceleration, AccelerationUnit};
use crate::core::Quantity;
use crate::mass::{Mass, MassUnit};
use std::ops::{Div, Mul};

// Conversion factors to Newtons
const STANDARD_GRAVITY: f64 = 9.80665;
const POUND_TO_KG: f64 = 0.45359237;
crate::quantity! {
    /// A quantity of force.
    ///
    /// Force represents a push or pull on an object.
    /// F = m * a (Newton's second law)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let mass = Mass::kilograms(10.0);
    /// let acceleration = Acceleration::meters_per_second_squared(9.8);
    ///
    /// // Force = Mass * Acceleration
    /// let force = mass * acceleration;
    /// assert!((force.to_newtons() - 98.0).abs() < 1e-10);
    /// ```
    pub quantity Force {
        unit: ForceUnit;
        dimension: ForceDimension;
        conversions: ForceConversions;
        name: "Force";
        primary: Newtons;
        si: Newtons;

        units {
            /// Newtons (N) - SI unit
            Newtons {
                symbol: "N",
                factor: 1.0,
                ctor: newtons,
                to: to_newtons,
                si: true
            },
            /// Kilonewtons (kN)
            Kilonewtons {
                symbol: "kN",
                factor: 1000.0,
                ctor: kilonewtons,
                to: to_kilonewtons,
                si: true
            },
            /// Kilogram-force (kgf)
            KilogramForce {
                symbol: "kgf",
                factor: STANDARD_GRAVITY,
                ctor: kilogram_force,
                to: to_kilogram_force,
                si: false
            },
            /// Pound-force (lbf)
            PoundForce {
                symbol: "lbf",
                factor: POUND_TO_KG * STANDARD_GRAVITY,
                ctor: pound_force,
                to: to_pound_force,
                si: false
            },
            /// Dynes (dyn) - CGS unit
            Dynes {
                symbol: "dyn",
                factor: 1e-5,
                ctor: dynes,
                to: to_dynes,
                si: true
            }
        }
    }
}
impl Force {
    /// Creates a Force from mass and acceleration (F = ma).
    pub fn from_mass_and_acceleration(mass: Mass, acceleration: Acceleration) -> Self {
        let newtons = mass.to_kilograms() * acceleration.to_meters_per_second_squared();
        Self::new(newtons, ForceUnit::Newtons)
    }
}

// Force / Mass = Acceleration
impl Div<Mass> for Force {
    type Output = Acceleration;

    fn div(self, rhs: Mass) -> Self::Output {
        let mpss = self.to_newtons() / rhs.to_kilograms();
        Acceleration::new(mpss, AccelerationUnit::MetersPerSecondSquared)
    }
}

// Force / Acceleration = Mass
impl Div<Acceleration> for Force {
    type Output = Mass;

    fn div(self, rhs: Acceleration) -> Self::Output {
        let kg = self.to_newtons() / rhs.to_meters_per_second_squared();
        Mass::new(kg, MassUnit::Kilograms)
    }
}

// Mass * Acceleration = Force
impl Mul<Acceleration> for Mass {
    type Output = Force;

    fn mul(self, rhs: Acceleration) -> Self::Output {
        Force::from_mass_and_acceleration(self, rhs)
    }
}

// Acceleration * Mass = Force
impl Mul<Mass> for Acceleration {
    type Output = Force;

    fn mul(self, rhs: Mass) -> Self::Output {
        Force::from_mass_and_acceleration(rhs, self)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_force_creation() {
        let f = Force::newtons(10.0);
        assert_eq!(f.value(), 10.0);
        assert_eq!(f.unit(), ForceUnit::Newtons);
    }

    #[test]
    fn test_force_conversions() {
        let f = Force::kilonewtons(1.0);
        assert_eq!(f.to_newtons(), 1000.0);

        let f2 = Force::kilogram_force(1.0);
        assert!((f2.to_newtons() - 9.80665).abs() < 0.0001);
    }

    #[test]
    fn test_fma() {
        // F = m * a
        let m = Mass::kilograms(10.0);
        let a = Acceleration::meters_per_second_squared(5.0);
        let f = m * a;
        assert_eq!(f.to_newtons(), 50.0);
    }

    #[test]
    fn test_force_divided_by_mass() {
        // a = F / m
        let f = Force::newtons(100.0);
        let m = Mass::kilograms(10.0);
        let a = f / m;
        assert_eq!(a.to_meters_per_second_squared(), 10.0);
    }

    #[test]
    fn test_force_divided_by_acceleration() {
        // m = F / a
        let f = Force::newtons(100.0);
        let a = Acceleration::meters_per_second_squared(10.0);
        let m = f / a;
        assert_eq!(m.to_kilograms(), 10.0);
    }

    #[test]
    fn test_pound_force() {
        let f = Force::pound_force(1.0);
        // 1 lbf = 0.45359237 kg * 9.80665 m/s² ≈ 4.448 N
        assert!((f.to_newtons() - 4.4482216).abs() < 0.001);
    }
}
