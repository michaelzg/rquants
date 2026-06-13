//! Pressure quantity and units.

use super::force::{Force, ForceUnit};
use crate::core::Quantity;
use crate::space::area::{Area, AreaUnit};
use std::ops::{Div, Mul};

// Conversion factors to Pascals
const BAR_TO_PA: f64 = 100_000.0;
const ATM_TO_PA: f64 = 101_325.0;
const PSI_TO_PA: f64 = 6894.757293168;
const MMHG_TO_PA: f64 = 133.322387415;
const INHG_TO_PA: f64 = 3386.389;
const TORR_TO_PA: f64 = ATM_TO_PA / 760.0;
crate::quantity! {
    /// A quantity of pressure (force per unit area).
    ///
    /// Pressure represents force distributed over an area.
    /// P = F / A
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let pressure = Pressure::atmospheres(1.0);
    /// let area = Area::square_meters(1.0);
    ///
    /// // Force = Pressure * Area
    /// let force = pressure * area;
    /// assert!((force.to_newtons() - 101325.0).abs() < 1.0);
    /// ```
    pub quantity Pressure {
        unit: PressureUnit;
        dimension: PressureDimension;
        conversions: PressureConversions;
        name: "Pressure";
        primary: Pascals;
        si: Pascals;

        units {
            /// Pascals (Pa) - SI unit (N/m²)
            Pascals {
                symbol: "Pa",
                factor: 1.0,
                ctor: pascals,
                to: to_pascals,
                si: true
            },
            /// Kilopascals (kPa)
            Kilopascals {
                symbol: "kPa",
                factor: 1000.0,
                ctor: kilopascals,
                to: to_kilopascals,
                si: true
            },
            /// Megapascals (MPa)
            Megapascals {
                symbol: "MPa",
                factor: 1_000_000.0,
                ctor: megapascals,
                to: to_megapascals,
                si: true
            },
            /// Bars (bar)
            Bars {
                symbol: "bar",
                factor: BAR_TO_PA,
                ctor: bars,
                to: to_bars,
                si: false
            },
            /// Pounds per square inch (psi)
            PoundsPerSquareInch {
                symbol: "psi",
                factor: PSI_TO_PA,
                ctor: psi,
                to: to_psi,
                si: false
            },
            /// Standard atmospheres (atm)
            Atmospheres {
                symbol: "atm",
                factor: ATM_TO_PA,
                ctor: atmospheres,
                to: to_atmospheres,
                si: false
            },
            /// Millimeters of mercury (mmHg)
            MillimetersOfMercury {
                symbol: "mmHg",
                factor: MMHG_TO_PA,
                ctor: millimeters_of_mercury,
                to: to_millimeters_of_mercury,
                si: false
            },
            /// Inches of mercury (inHg)
            InchesOfMercury {
                symbol: "inHg",
                factor: INHG_TO_PA,
                ctor: inches_of_mercury,
                to: to_inches_of_mercury,
                si: false
            },
            /// Torr
            Torr {
                symbol: "Torr",
                factor: TORR_TO_PA,
                ctor: torr,
                to: to_torr,
                si: false
            }
        }
    }
}
impl Pressure {
    /// Creates a Pressure from force and area (P = F/A).
    pub fn from_force_and_area(force: Force, area: Area) -> Self {
        let pascals = force.to_newtons() / area.to_square_meters();
        Self::new(pascals, PressureUnit::Pascals)
    }
}

// Pressure * Area = Force
impl Mul<Area> for Pressure {
    type Output = Force;

    fn mul(self, rhs: Area) -> Self::Output {
        let newtons = self.to_pascals() * rhs.to_square_meters();
        Force::new(newtons, ForceUnit::Newtons)
    }
}

// Force / Area = Pressure
impl Div<Area> for Force {
    type Output = Pressure;

    fn div(self, rhs: Area) -> Self::Output {
        Pressure::from_force_and_area(self, rhs)
    }
}

// Force / Pressure = Area
impl Div<Pressure> for Force {
    type Output = Area;

    fn div(self, rhs: Pressure) -> Self::Output {
        let sqm = self.to_newtons() / rhs.to_pascals();
        Area::new(sqm, AreaUnit::SquareMeters)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_pressure_creation() {
        let p = Pressure::pascals(101325.0);
        assert_eq!(p.value(), 101325.0);
        assert_eq!(p.unit(), PressureUnit::Pascals);
    }

    #[test]
    fn test_atmosphere_conversion() {
        let p = Pressure::atmospheres(1.0);
        assert!((p.to_pascals() - 101325.0).abs() < 1.0);
    }

    #[test]
    fn test_bar_conversion() {
        let p = Pressure::bars(1.0);
        assert_eq!(p.to_pascals(), 100000.0);
    }

    #[test]
    fn test_psi_conversion() {
        let p = Pressure::psi(14.696);
        // 14.696 psi ≈ 1 atm ≈ 101325 Pa
        assert!((p.to_pascals() - 101325.0).abs() < 100.0);
    }

    #[test]
    fn test_torr_conversion() {
        let p = Pressure::torr(760.0);
        // 760 Torr = 1 atm
        assert!((p.to_atmospheres() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_pressure_times_area() {
        let p = Pressure::pascals(1000.0);
        let a = Area::square_meters(10.0);
        let f = p * a;
        assert_eq!(f.to_newtons(), 10000.0);
    }

    #[test]
    fn test_force_divided_by_area() {
        let f = Force::newtons(1000.0);
        let a = Area::square_meters(10.0);
        let p = f / a;
        assert_eq!(p.to_pascals(), 100.0);
    }

    #[test]
    fn test_force_divided_by_pressure() {
        let f = Force::newtons(1000.0);
        let p = Pressure::pascals(100.0);
        let a = f / p;
        assert_eq!(a.to_square_meters(), 10.0);
    }
}
