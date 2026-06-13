//! Angle quantity and units.

use std::f64::consts::PI;
crate::quantity! {
    /// A quantity of angle.
    ///
    /// Angle represents a plane angle measurement.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    /// use std::f64::consts::PI;
    ///
    /// let a1 = Angle::radians(PI);
    /// let a2 = Angle::degrees(180.0);
    ///
    /// // These represent the same angle
    /// assert!((a1.to_radians() - a2.to_radians()).abs() < 1e-10);
    /// ```
    pub quantity Angle {
        unit: AngleUnit;
        dimension: AngleDimension;
        conversions: AngleConversions;
        name: "Angle";
        primary: Radians;
        si: Radians;

        units {
            /// Radians (rad) - SI unit
            Radians {
                symbol: "rad",
                factor: 1.0,
                ctor: radians,
                to: to_radians,
                si: true
            },
            /// Degrees (°)
            Degrees {
                symbol: "°",
                factor: PI / 180.0,
                ctor: degrees,
                to: to_degrees,
                si: false
            },
            /// Gradians/Gons (gon)
            Gradians {
                symbol: "gon",
                factor: PI / 200.0,
                ctor: gradians,
                to: to_gradians,
                si: false
            },
            /// Turns (complete rotations)
            Turns {
                symbol: "tr",
                factor: 2.0 * PI,
                ctor: turns,
                to: to_turns,
                si: false
            },
            /// Arc minutes (')
            ArcMinutes {
                symbol: "'",
                factor: PI / (180.0 * 60.0),
                ctor: arc_minutes,
                to: to_arc_minutes,
                si: false
            },
            /// Arc seconds ('')
            ArcSeconds {
                symbol: "''",
                factor: PI / (180.0 * 3600.0),
                ctor: arc_seconds,
                to: to_arc_seconds,
                si: false
            }
        }
    }
}
impl Angle {
    // Trigonometric functions
    /// Returns the sine of this angle.
    pub fn sin(&self) -> f64 {
        self.to_radians().sin()
    }

    /// Returns the cosine of this angle.
    pub fn cos(&self) -> f64 {
        self.to_radians().cos()
    }

    /// Returns the tangent of this angle.
    pub fn tan(&self) -> f64 {
        self.to_radians().tan()
    }

    /// Creates an angle from its sine value.
    pub fn asin(value: f64) -> Self {
        Self::radians(value.asin())
    }

    /// Creates an angle from its cosine value.
    pub fn acos(value: f64) -> Self {
        Self::radians(value.acos())
    }

    /// Creates an angle from its tangent value.
    pub fn atan(value: f64) -> Self {
        Self::radians(value.atan())
    }

    /// Creates an angle from atan2(y, x).
    pub fn atan2(y: f64, x: f64) -> Self {
        Self::radians(y.atan2(x))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_angle_creation() {
        let a = Angle::degrees(90.0);
        assert_eq!(a.value(), 90.0);
        assert_eq!(a.unit(), AngleUnit::Degrees);
    }

    #[test]
    fn test_angle_conversions() {
        let a = Angle::degrees(180.0);
        assert!((a.to_radians() - PI).abs() < 1e-10);

        let a2 = Angle::turns(1.0);
        assert!((a2.to_degrees() - 360.0).abs() < 1e-10);
    }

    #[test]
    fn test_trig_functions() {
        let a = Angle::degrees(90.0);
        assert!((a.sin() - 1.0).abs() < 1e-10);
        assert!(a.cos().abs() < 1e-10);

        let a2 = Angle::degrees(45.0);
        assert!((a2.tan() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_trig() {
        let a = Angle::asin(1.0);
        assert!((a.to_degrees() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn test_angle_arithmetic() {
        let a1 = Angle::degrees(45.0);
        let a2 = Angle::degrees(45.0);
        let sum = a1 + a2;
        assert!((sum.to_degrees() - 90.0).abs() < 1e-10);
    }
}
