//! Power quantity and units.

use crate::core::Quantity;
use crate::time::{Time, TimeUnit};
use std::ops::{Div, Mul};

// Conversion factors relative to Watts
const BTU_TO_J: f64 = 1055.06;
const SECONDS_PER_HOUR: f64 = 3600.0;
const BTU_PER_HOUR_TO_W: f64 = BTU_TO_J / SECONDS_PER_HOUR;
const HORSEPOWER_TO_W: f64 = 745.7; // Mechanical horsepower
const SOLAR_LUMINOSITY_TO_W: f64 = 3.828e26;
crate::quantity! {
    /// A quantity of power.
    ///
    /// Power represents the rate of energy transfer.
    /// P = E / t = dE/dt
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let power = Power::kilowatts(1.0);
    /// let time = Time::hours(2.0);
    ///
    /// // Energy = Power * Time
    /// let energy = power * time;
    /// assert!((energy.to_kilowatt_hours() - 2.0).abs() < 1e-10);
    /// ```
    pub quantity Power {
        unit: PowerUnit;
        dimension: PowerDimension;
        conversions: PowerConversions;
        name: "Power";
        primary: Watts;
        si: Watts;

        units {
            /// Watts (W) - SI unit
            Watts {
                symbol: "W",
                factor: 1.0,
                ctor: watts,
                to: to_watts,
                si: true
            },
            /// Milliwatts (mW)
            Milliwatts {
                symbol: "mW",
                factor: 1e-3,
                ctor: milliwatts,
                to: to_milliwatts,
                si: true
            },
            /// Kilowatts (kW)
            Kilowatts {
                symbol: "kW",
                factor: 1e3,
                ctor: kilowatts,
                to: to_kilowatts,
                si: true
            },
            /// Megawatts (MW)
            Megawatts {
                symbol: "MW",
                factor: 1e6,
                ctor: megawatts,
                to: to_megawatts,
                si: true
            },
            /// Gigawatts (GW)
            Gigawatts {
                symbol: "GW",
                factor: 1e9,
                ctor: gigawatts,
                to: to_gigawatts,
                si: true
            },
            /// BTU per hour
            BtusPerHour {
                symbol: "BTU/h",
                factor: BTU_PER_HOUR_TO_W,
                ctor: btus_per_hour,
                to: to_btus_per_hour,
                si: false
            },
            /// Ergs per second
            ErgsPerSecond {
                symbol: "erg/s",
                factor: 1e-7,
                ctor: ergs_per_second,
                to: to_ergs_per_second,
                si: false
            },
            /// Horsepower (mechanical)
            Horsepower {
                symbol: "hp",
                factor: HORSEPOWER_TO_W,
                ctor: horsepower,
                to: to_horsepower,
                si: false
            },
            /// Solar luminosities
            SolarLuminosities {
                symbol: "L☉",
                factor: SOLAR_LUMINOSITY_TO_W,
                ctor: solar_luminosities,
                to: to_solar_luminosities,
                si: false
            }
        }
    }
}
// Cross-quantity operations
use super::energy::{Energy, EnergyUnit};
use super::power_density::{PowerDensity, PowerDensityUnit};
use super::power_ramp::{PowerRamp, PowerRampUnit};
use crate::space::{Volume, VolumeUnit};

// Power * Time = Energy
impl Mul<Time> for Power {
    type Output = Energy;

    fn mul(self, rhs: Time) -> Self::Output {
        let joules = self.to_watts() * rhs.to_seconds();
        Energy::new(joules, EnergyUnit::Joules)
    }
}

// Time * Power = Energy
impl Mul<Power> for Time {
    type Output = Energy;

    fn mul(self, rhs: Power) -> Self::Output {
        let joules = rhs.to_watts() * self.to_seconds();
        Energy::new(joules, EnergyUnit::Joules)
    }
}

// Power / Time = PowerRamp
impl Div<Time> for Power {
    type Output = PowerRamp;

    fn div(self, rhs: Time) -> Self::Output {
        let wph = self.to_watts() / rhs.to_hours();
        PowerRamp::new(wph, PowerRampUnit::WattsPerHour)
    }
}

// Power / PowerRamp = Time
impl Div<PowerRamp> for Power {
    type Output = Time;

    fn div(self, rhs: PowerRamp) -> Self::Output {
        let hours = self.to_watts() / rhs.to_watts_per_hour();
        Time::new(hours, TimeUnit::Hours)
    }
}

// Power / Volume = PowerDensity
impl Div<Volume> for Power {
    type Output = PowerDensity;

    fn div(self, rhs: Volume) -> Self::Output {
        let wpcm = self.to_watts() / rhs.to_cubic_meters();
        PowerDensity::new(wpcm, PowerDensityUnit::WattsPerCubicMeter)
    }
}

// Power / PowerDensity = Volume
impl Div<PowerDensity> for Power {
    type Output = Volume;

    fn div(self, rhs: PowerDensity) -> Self::Output {
        let m3 = self.to_watts() / rhs.to_watts_per_cubic_meter();
        Volume::new(m3, VolumeUnit::CubicMeters)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

    #[test]
    fn test_power_creation() {
        let p = Power::watts(1000.0);
        assert_eq!(p.value(), 1000.0);
        assert_eq!(p.unit(), PowerUnit::Watts);
    }

    #[test]
    fn test_power_conversions() {
        let p = Power::kilowatts(1.0);
        assert_eq!(p.to_watts(), 1000.0);

        let p2 = Power::megawatts(1.0);
        assert_eq!(p2.to_kilowatts(), 1000.0);
    }

    #[test]
    fn test_horsepower_conversion() {
        let p = Power::horsepower(1.0);
        // 1 hp ≈ 745.7 W
        assert!((p.to_watts() - 745.7).abs() < 1.0);
    }

    #[test]
    fn test_power_times_time() {
        let p = Power::kilowatts(1.0);
        let t = Time::hours(2.0);
        let e = p * t;
        // 1 kW * 2 h = 2 kWh = 7,200,000 J
        assert!((e.to_joules() - 7_200_000.0).abs() < 1.0);
    }

    #[test]
    fn test_power_divided_by_time() {
        let p = Power::kilowatts(100.0);
        let t = Time::hours(2.0);
        let pr = p / t;
        // 100 kW / 2 h = 50 kW/h = 50000 W/h
        assert!((pr.to_watts_per_hour() - 50000.0).abs() < 1e-10);
    }
}
