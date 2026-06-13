//! Solid angle quantity and units.

use std::f64::consts::PI;
crate::quantity! {
    /// A quantity of solid angle.
    ///
    /// Solid angle represents a two-dimensional angle subtended at a point.
    /// A full sphere subtends 4π steradians.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    /// use std::f64::consts::PI;
    ///
    /// let sa = SolidAngle::spheres(1.0);
    /// assert!((sa.to_steradians() - 4.0 * PI).abs() < 1e-10);
    /// ```
    pub quantity SolidAngle {
        unit: SolidAngleUnit;
        dimension: SolidAngleDimension;
        conversions: SolidAngleConversions;
        name: "SolidAngle";
        primary: Steradians;
        si: Steradians;

        units {
            /// Steradians (sr) - SI unit
            Steradians {
                symbol: "sr",
                factor: 1.0,
                ctor: steradians,
                to: to_steradians,
                si: true
            },
            /// Square degrees
            SquareDegrees {
                symbol: "deg²",
                factor: (PI / 180.0) * (PI / 180.0),
                ctor: square_degrees,
                to: to_square_degrees,
                si: false
            },
            /// Spheres (complete sphere = 4π steradians)
            Spheres {
                symbol: "sphere",
                factor: 4.0 * PI,
                ctor: spheres,
                to: to_spheres,
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
    fn test_solid_angle_creation() {
        let sa = SolidAngle::steradians(1.0);
        assert_eq!(sa.value(), 1.0);
        assert_eq!(sa.unit(), SolidAngleUnit::Steradians);
    }

    #[test]
    fn test_solid_angle_conversions() {
        let sa = SolidAngle::spheres(1.0);
        assert!((sa.to_steradians() - 4.0 * PI).abs() < 1e-10);
    }

    #[test]
    fn test_solid_angle_arithmetic() {
        let sa1 = SolidAngle::steradians(1.0);
        let sa2 = SolidAngle::steradians(1.0);
        let sum = sa1 + sa2;
        assert_eq!(sum.to_steradians(), 2.0);
    }
}
