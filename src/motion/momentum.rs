//! Momentum quantity and units.

use super::velocity::{Velocity, VelocityUnit};
use crate::core::Quantity;
use crate::mass::{Mass, MassUnit};
use std::ops::{Div, Mul};

// Conversion factors to kg·m/s
const LB_TO_KG: f64 = 0.45359237;
const FT_TO_M: f64 = 0.3048;
crate::quantity! {
    /// A quantity of momentum (mass in motion).
    ///
    /// Momentum is the product of mass and velocity.
    /// p = m * v
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let mass = Mass::kilograms(10.0);
    /// let velocity = Velocity::meters_per_second(5.0);
    ///
    /// // Momentum = Mass * Velocity
    /// let momentum = mass * velocity;
    /// assert!((momentum.to_kilogram_meters_per_second() - 50.0).abs() < 1e-10);
    /// ```
    pub quantity Momentum {
        unit: MomentumUnit;
        dimension: MomentumDimension;
        conversions: MomentumConversions;
        name: "Momentum";
        primary: KilogramMetersPerSecond;
        si: KilogramMetersPerSecond;

        units {
            /// Kilogram-meters per second (kg·m/s) - SI unit
            KilogramMetersPerSecond {
                symbol: "kg·m/s",
                factor: 1.0,
                ctor: kilogram_meters_per_second,
                to: to_kilogram_meters_per_second,
                si: true
            },
            /// Newton-seconds (N·s) - equivalent to kg·m/s
            NewtonSeconds {
                symbol: "N·s",
                factor: 1.0,
                ctor: newton_seconds,
                to: to_newton_seconds,
                si: true
            },
            /// Pound-feet per second (lb·ft/s)
            PoundFeetPerSecond {
                symbol: "lb·ft/s",
                factor: LB_TO_KG * FT_TO_M,
                ctor: pound_feet_per_second,
                to: to_pound_feet_per_second,
                si: false
            }
        }
    }
}
impl Momentum {
    /// Creates a Momentum from mass and velocity (p = mv).
    pub fn from_mass_and_velocity(mass: Mass, velocity: Velocity) -> Self {
        let kgmps = mass.to_kilograms() * velocity.to_meters_per_second();
        Self::new(kgmps, MomentumUnit::KilogramMetersPerSecond)
    }
}

// Momentum / Mass = Velocity
impl Div<Mass> for Momentum {
    type Output = Velocity;

    fn div(self, rhs: Mass) -> Self::Output {
        let mps = self.to_kilogram_meters_per_second() / rhs.to_kilograms();
        Velocity::new(mps, VelocityUnit::MetersPerSecond)
    }
}

// Momentum / Velocity = Mass
impl Div<Velocity> for Momentum {
    type Output = Mass;

    fn div(self, rhs: Velocity) -> Self::Output {
        let kg = self.to_kilogram_meters_per_second() / rhs.to_meters_per_second();
        Mass::new(kg, MassUnit::Kilograms)
    }
}

// Mass * Velocity = Momentum
impl Mul<Velocity> for Mass {
    type Output = Momentum;

    fn mul(self, rhs: Velocity) -> Self::Output {
        Momentum::from_mass_and_velocity(self, rhs)
    }
}

// Velocity * Mass = Momentum
impl Mul<Mass> for Velocity {
    type Output = Momentum;

    fn mul(self, rhs: Mass) -> Self::Output {
        Momentum::from_mass_and_velocity(rhs, self)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_momentum_creation() {
        let p = Momentum::kilogram_meters_per_second(10.0);
        assert_eq!(p.value(), 10.0);
        assert_eq!(p.unit(), MomentumUnit::KilogramMetersPerSecond);
    }

    #[test]
    fn test_newton_seconds_equivalent() {
        let p1 = Momentum::kilogram_meters_per_second(10.0);
        let p2 = Momentum::newton_seconds(10.0);
        assert_eq!(
            p1.to_kilogram_meters_per_second(),
            p2.to_kilogram_meters_per_second()
        );
    }

    #[test]
    fn test_mass_times_velocity() {
        let m = Mass::kilograms(5.0);
        let v = Velocity::meters_per_second(10.0);
        let p = m * v;
        assert_eq!(p.to_kilogram_meters_per_second(), 50.0);
    }

    #[test]
    fn test_momentum_divided_by_mass() {
        let p = Momentum::kilogram_meters_per_second(100.0);
        let m = Mass::kilograms(10.0);
        let v = p / m;
        assert_eq!(v.to_meters_per_second(), 10.0);
    }

    #[test]
    fn test_momentum_divided_by_velocity() {
        let p = Momentum::kilogram_meters_per_second(100.0);
        let v = Velocity::meters_per_second(10.0);
        let m = p / v;
        assert_eq!(m.to_kilograms(), 10.0);
    }
}
