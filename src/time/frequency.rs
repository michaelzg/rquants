//! Frequency quantity and units.

use crate::systems::metric::{GIGA, KILO, MEGA, TERA};
crate::quantity! {
    /// A quantity of frequency.
    ///
    /// Frequency represents the number of cycles or occurrences per unit time.
    /// The SI unit is Hertz (Hz), which is cycles per second.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let f1 = Frequency::hertz(1000.0);
    /// let f2 = Frequency::kilohertz(1.0);
    ///
    /// // These represent the same frequency
    /// assert!((f1.to(FrequencyUnit::Hertz) - f2.to(FrequencyUnit::Hertz)).abs() < 1e-10);
    /// ```
    pub quantity Frequency {
        unit: FrequencyUnit;
        dimension: FrequencyDimension;
        conversions: FrequencyConversions;
        name: "Frequency";
        primary: Hertz;
        si: Hertz;

        units {
            /// Hertz (Hz) - cycles per second, SI unit
            Hertz {
                symbol: "Hz",
                factor: 1.0,
                ctor: hertz,
                to: to_hertz,
                si: true
            },
            /// Kilohertz (kHz) - 10^3 Hz
            Kilohertz {
                symbol: "kHz",
                factor: KILO,
                ctor: kilohertz,
                to: to_kilohertz,
                si: true
            },
            /// Megahertz (MHz) - 10^6 Hz
            Megahertz {
                symbol: "MHz",
                factor: MEGA,
                ctor: megahertz,
                to: to_megahertz,
                si: true
            },
            /// Gigahertz (GHz) - 10^9 Hz
            Gigahertz {
                symbol: "GHz",
                factor: GIGA,
                ctor: gigahertz,
                to: to_gigahertz,
                si: true
            },
            /// Terahertz (THz) - 10^12 Hz
            Terahertz {
                symbol: "THz",
                factor: TERA,
                ctor: terahertz,
                to: to_terahertz,
                si: true
            },
            /// Revolutions per minute (rpm)
            RevolutionsPerMinute {
                symbol: "rpm",
                factor: 1.0 / 60.0,
                ctor: rpm,
                to: to_rpm,
                si: false
            }
        }
    }
}
impl Frequency {
    /// Returns the period (1 / frequency) in seconds.
    pub fn period_seconds(&self) -> f64 {
        1.0 / self.to_hertz()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_frequency_creation() {
        let f = Frequency::hertz(1000.0);
        assert_eq!(f.value(), 1000.0);
        assert_eq!(f.unit(), FrequencyUnit::Hertz);
    }

    #[test]
    fn test_frequency_conversions() {
        let f = Frequency::hertz(1_000_000.0);
        assert_eq!(f.to_kilohertz(), 1000.0);
        assert_eq!(f.to_megahertz(), 1.0);
    }

    #[test]
    fn test_frequency_arithmetic() {
        let f1 = Frequency::hertz(500.0);
        let f2 = Frequency::hertz(500.0);
        let sum = f1 + f2;
        assert_eq!(sum.to_hertz(), 1000.0);
    }

    #[test]
    fn test_frequency_comparison() {
        let f1 = Frequency::kilohertz(1.0);
        let f2 = Frequency::hertz(1000.0);
        assert_eq!(f1, f2);
    }

    #[test]
    fn test_frequency_display() {
        let f = Frequency::megahertz(2.4);
        assert_eq!(format!("{}", f), "2.4 MHz");
    }

    #[test]
    fn test_frequency_dsl() {
        let f = 100.0.hertz();
        assert_eq!(f.to_hertz(), 100.0);
    }

    #[test]
    fn test_rpm_conversion() {
        let f = Frequency::rpm(60.0);
        // 60 rpm = 1 Hz
        assert!((f.to_hertz() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_period() {
        let f = Frequency::hertz(2.0);
        assert_eq!(f.period_seconds(), 0.5);
    }
}
