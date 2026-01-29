//! Power quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use crate::time::{Time, TimeUnit};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of power measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PowerUnit {
    /// Watts (W) - SI unit
    Watts,
    /// Milliwatts (mW)
    Milliwatts,
    /// Kilowatts (kW)
    Kilowatts,
    /// Megawatts (MW)
    Megawatts,
    /// Gigawatts (GW)
    Gigawatts,
    /// BTU per hour
    BtusPerHour,
    /// Ergs per second
    ErgsPerSecond,
    /// Horsepower (mechanical)
    Horsepower,
    /// Solar luminosities
    SolarLuminosities,
}

impl PowerUnit {
    /// All available power units.
    pub const ALL: &'static [PowerUnit] = &[
        PowerUnit::Watts,
        PowerUnit::Milliwatts,
        PowerUnit::Kilowatts,
        PowerUnit::Megawatts,
        PowerUnit::Gigawatts,
        PowerUnit::BtusPerHour,
        PowerUnit::ErgsPerSecond,
        PowerUnit::Horsepower,
        PowerUnit::SolarLuminosities,
    ];
}

// Conversion factors relative to Watts
const BTU_TO_J: f64 = 1055.06;
const SECONDS_PER_HOUR: f64 = 3600.0;
const BTU_PER_HOUR_TO_W: f64 = BTU_TO_J / SECONDS_PER_HOUR;
const HORSEPOWER_TO_W: f64 = 745.7; // Mechanical horsepower
const SOLAR_LUMINOSITY_TO_W: f64 = 3.828e26;

impl fmt::Display for PowerUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for PowerUnit {
    fn symbol(&self) -> &'static str {
        match self {
            PowerUnit::Watts => "W",
            PowerUnit::Milliwatts => "mW",
            PowerUnit::Kilowatts => "kW",
            PowerUnit::Megawatts => "MW",
            PowerUnit::Gigawatts => "GW",
            PowerUnit::BtusPerHour => "BTU/h",
            PowerUnit::ErgsPerSecond => "erg/s",
            PowerUnit::Horsepower => "hp",
            PowerUnit::SolarLuminosities => "L☉",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            PowerUnit::Watts => 1.0,
            PowerUnit::Milliwatts => 1e-3,
            PowerUnit::Kilowatts => 1e3,
            PowerUnit::Megawatts => 1e6,
            PowerUnit::Gigawatts => 1e9,
            PowerUnit::BtusPerHour => BTU_PER_HOUR_TO_W,
            PowerUnit::ErgsPerSecond => 1e-7,
            PowerUnit::Horsepower => HORSEPOWER_TO_W,
            PowerUnit::SolarLuminosities => SOLAR_LUMINOSITY_TO_W,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            PowerUnit::Watts
                | PowerUnit::Milliwatts
                | PowerUnit::Kilowatts
                | PowerUnit::Megawatts
                | PowerUnit::Gigawatts
        )
    }
}

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
#[derive(Debug, Clone, Copy)]
pub struct Power {
    value: f64,
    unit: PowerUnit,
}

impl Power {
    /// Creates a new Power quantity.
    pub const fn new_const(value: f64, unit: PowerUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a Power in watts.
    pub fn watts(value: f64) -> Self {
        Self::new(value, PowerUnit::Watts)
    }

    /// Creates a Power in milliwatts.
    pub fn milliwatts(value: f64) -> Self {
        Self::new(value, PowerUnit::Milliwatts)
    }

    /// Creates a Power in kilowatts.
    pub fn kilowatts(value: f64) -> Self {
        Self::new(value, PowerUnit::Kilowatts)
    }

    /// Creates a Power in megawatts.
    pub fn megawatts(value: f64) -> Self {
        Self::new(value, PowerUnit::Megawatts)
    }

    /// Creates a Power in gigawatts.
    pub fn gigawatts(value: f64) -> Self {
        Self::new(value, PowerUnit::Gigawatts)
    }

    /// Creates a Power in horsepower.
    pub fn horsepower(value: f64) -> Self {
        Self::new(value, PowerUnit::Horsepower)
    }

    /// Creates a Power in BTU/hour.
    pub fn btus_per_hour(value: f64) -> Self {
        Self::new(value, PowerUnit::BtusPerHour)
    }

    // Conversion methods
    /// Converts to watts.
    pub fn to_watts(&self) -> f64 {
        self.to(PowerUnit::Watts)
    }

    /// Converts to milliwatts.
    pub fn to_milliwatts(&self) -> f64 {
        self.to(PowerUnit::Milliwatts)
    }

    /// Converts to kilowatts.
    pub fn to_kilowatts(&self) -> f64 {
        self.to(PowerUnit::Kilowatts)
    }

    /// Converts to megawatts.
    pub fn to_megawatts(&self) -> f64 {
        self.to(PowerUnit::Megawatts)
    }

    /// Converts to gigawatts.
    pub fn to_gigawatts(&self) -> f64 {
        self.to(PowerUnit::Gigawatts)
    }

    /// Converts to horsepower.
    pub fn to_horsepower(&self) -> f64 {
        self.to(PowerUnit::Horsepower)
    }

    /// Converts to BTU/hour.
    pub fn to_btus_per_hour(&self) -> f64 {
        self.to(PowerUnit::BtusPerHour)
    }
}

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Power {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Power {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Power {
    type Unit = PowerUnit;

    fn new(value: f64, unit: Self::Unit) -> Self {
        Self { value, unit }
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> Self::Unit {
        self.unit
    }
}

// Arithmetic operations

impl Add for Power {
    type Output = Power;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Power::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Power {
    type Output = Power;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Power::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Power {
    type Output = Power;

    fn mul(self, rhs: f64) -> Self::Output {
        Power::new(self.value * rhs, self.unit)
    }
}

impl Mul<Power> for f64 {
    type Output = Power;

    fn mul(self, rhs: Power) -> Self::Output {
        Power::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for Power {
    type Output = Power;

    fn div(self, rhs: f64) -> Self::Output {
        Power::new(self.value / rhs, self.unit)
    }
}

impl Div<Power> for Power {
    type Output = f64;

    fn div(self, rhs: Power) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Power {
    type Output = Power;

    fn neg(self) -> Self::Output {
        Power::new(-self.value, self.unit)
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

/// Dimension for Power.
pub struct PowerDimension;

impl Dimension for PowerDimension {
    type Quantity = Power;
    type Unit = PowerUnit;

    fn name() -> &'static str {
        "Power"
    }

    fn primary_unit() -> Self::Unit {
        PowerUnit::Watts
    }

    fn si_unit() -> Self::Unit {
        PowerUnit::Watts
    }

    fn units() -> &'static [Self::Unit] {
        PowerUnit::ALL
    }
}

/// Extension trait for creating Power quantities from numeric types.
pub trait PowerConversions {
    /// Creates a Power in watts.
    fn watts(self) -> Power;
    /// Creates a Power in milliwatts.
    fn milliwatts(self) -> Power;
    /// Creates a Power in kilowatts.
    fn kilowatts(self) -> Power;
    /// Creates a Power in megawatts.
    fn megawatts(self) -> Power;
    /// Creates a Power in gigawatts.
    fn gigawatts(self) -> Power;
    /// Creates a Power in horsepower.
    fn horsepower(self) -> Power;
}

impl PowerConversions for f64 {
    fn watts(self) -> Power {
        Power::watts(self)
    }
    fn milliwatts(self) -> Power {
        Power::milliwatts(self)
    }
    fn kilowatts(self) -> Power {
        Power::kilowatts(self)
    }
    fn megawatts(self) -> Power {
        Power::megawatts(self)
    }
    fn gigawatts(self) -> Power {
        Power::gigawatts(self)
    }
    fn horsepower(self) -> Power {
        Power::horsepower(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
