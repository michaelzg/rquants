//! Temperature quantity with scale and degree conversions.
//!
//! Temperature is unique among quantities because different scales
//! have different zero points. This module supports both:
//!
//! - **Scale conversions**: Account for zero offsets (e.g., 0°C = 273.15K)
//! - **Degree conversions**: Only ratio differences (e.g., 1°C = 1.8°F in magnitude)

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// Temperature scales.
///
/// Unlike other units, temperature scales have different zero points,
/// requiring special conversion logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemperatureScale {
    /// Kelvin (K) - SI absolute scale
    Kelvin,
    /// Celsius (°C) - empirical scale (0°C = 273.15K)
    Celsius,
    /// Fahrenheit (°F) - empirical scale (32°F = 0°C)
    Fahrenheit,
    /// Rankine (°R) - absolute scale with Fahrenheit-sized degrees
    Rankine,
}

impl TemperatureScale {
    /// All available temperature scales.
    pub const ALL: &'static [TemperatureScale] = &[
        TemperatureScale::Kelvin,
        TemperatureScale::Celsius,
        TemperatureScale::Fahrenheit,
        TemperatureScale::Rankine,
    ];

    /// Returns the symbol for this scale.
    pub fn symbol(&self) -> &'static str {
        match self {
            TemperatureScale::Kelvin => "K",
            TemperatureScale::Celsius => "°C",
            TemperatureScale::Fahrenheit => "°F",
            TemperatureScale::Rankine => "°R",
        }
    }

    /// Returns `(ratio, offset)` for converting this scale to Kelvin:
    /// `kelvin = value * ratio + offset`.
    const fn to_kelvin_params(self) -> (f64, f64) {
        match self {
            TemperatureScale::Kelvin => (1.0, 0.0),
            TemperatureScale::Celsius => (1.0, 273.15),
            TemperatureScale::Fahrenheit => (5.0 / 9.0, 459.67 * 5.0 / 9.0),
            TemperatureScale::Rankine => (5.0 / 9.0, 0.0),
        }
    }
}

impl fmt::Display for TemperatureScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

/// A quantity of temperature.
///
/// Temperature supports both scale and degree conversions.
///
/// # Scale vs Degree Conversions
///
/// - `to_celsius_scale()` / `to_fahrenheit_scale()` etc.: Convert a thermometer reading
/// - `to_celsius_degrees()` / `to_fahrenheit_degrees()` etc.: Convert a temperature difference
///
/// # Arithmetic
///
/// Addition and subtraction treat the right operand as degrees (no offset):
/// ```rust
/// use rquants::thermal::temperature::{Temperature, TemperatureScale};
///
/// // 100°F - 5°C (as degrees) = 100 - 9 = 91°F
/// let t = Temperature::fahrenheit(100.0) - Temperature::celsius(5.0);
/// assert!((t.value() - 91.0).abs() < 1e-10);
/// ```
///
/// # Example
///
/// ```rust
/// use rquants::thermal::temperature::{Temperature, TemperatureScale};
///
/// let boiling = Temperature::celsius(100.0);
/// assert!((boiling.to_fahrenheit_scale() - 212.0).abs() < 1e-10);
/// assert!((boiling.to_kelvin_scale() - 373.15).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Temperature {
    value: f64,
    scale: TemperatureScale,
}

impl Temperature {
    /// Creates a new Temperature.
    pub fn new(value: f64, scale: TemperatureScale) -> Self {
        Self { value, scale }
    }

    /// Creates a Temperature in Kelvin.
    pub fn kelvin(value: f64) -> Self {
        Self::new(value, TemperatureScale::Kelvin)
    }

    /// Creates a Temperature in Celsius.
    pub fn celsius(value: f64) -> Self {
        Self::new(value, TemperatureScale::Celsius)
    }

    /// Creates a Temperature in Fahrenheit.
    pub fn fahrenheit(value: f64) -> Self {
        Self::new(value, TemperatureScale::Fahrenheit)
    }

    /// Creates a Temperature in Rankine.
    pub fn rankine(value: f64) -> Self {
        Self::new(value, TemperatureScale::Rankine)
    }

    /// Returns the numeric value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns the temperature scale.
    pub fn scale(&self) -> TemperatureScale {
        self.scale
    }

    // ===== Scale conversions (thermometer readings) =====

    /// Converts to Kelvin (scale).
    pub fn to_kelvin_scale(&self) -> f64 {
        self.to_scale(TemperatureScale::Kelvin)
    }

    /// Converts to Celsius (scale).
    pub fn to_celsius_scale(&self) -> f64 {
        self.to_scale(TemperatureScale::Celsius)
    }

    /// Converts to Fahrenheit (scale).
    pub fn to_fahrenheit_scale(&self) -> f64 {
        self.to_scale(TemperatureScale::Fahrenheit)
    }

    /// Converts to Rankine (scale).
    pub fn to_rankine_scale(&self) -> f64 {
        self.to_scale(TemperatureScale::Rankine)
    }

    /// Converts this temperature to another scale (adjusting for zero offset).
    pub fn to_scale(&self, target: TemperatureScale) -> f64 {
        self.convert(target, true).value
    }

    /// Returns this temperature expressed in the given scale.
    pub fn in_scale(&self, target: TemperatureScale) -> Temperature {
        self.convert(target, true)
    }

    /// Returns this temperature in Kelvin.
    pub fn in_kelvin(&self) -> Temperature {
        self.in_scale(TemperatureScale::Kelvin)
    }

    /// Returns this temperature in Celsius.
    pub fn in_celsius(&self) -> Temperature {
        self.in_scale(TemperatureScale::Celsius)
    }

    /// Returns this temperature in Fahrenheit.
    pub fn in_fahrenheit(&self) -> Temperature {
        self.in_scale(TemperatureScale::Fahrenheit)
    }

    // ===== Degree conversions (magnitudes only, no offset) =====

    /// Converts to Kelvin degrees (magnitude only).
    pub fn to_kelvin_degrees(&self) -> f64 {
        self.convert(TemperatureScale::Kelvin, false).value
    }

    /// Converts to Celsius degrees (magnitude only).
    pub fn to_celsius_degrees(&self) -> f64 {
        self.convert(TemperatureScale::Celsius, false).value
    }

    /// Converts to Fahrenheit degrees (magnitude only).
    pub fn to_fahrenheit_degrees(&self) -> f64 {
        self.convert(TemperatureScale::Fahrenheit, false).value
    }

    /// Converts to Rankine degrees (magnitude only).
    pub fn to_rankine_degrees(&self) -> f64 {
        self.convert(TemperatureScale::Rankine, false).value
    }

    fn convert(&self, target: TemperatureScale, with_offset: bool) -> Temperature {
        if self.scale == target {
            return *self;
        }

        let (source_ratio, source_offset) = self.scale.to_kelvin_params();
        let (target_ratio, target_offset) = target.to_kelvin_params();
        let converted = if with_offset {
            ((self.value * source_ratio + source_offset) - target_offset) / target_ratio
        } else {
            self.value * source_ratio / target_ratio
        };
        Temperature::new(converted, target)
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.scale {
            TemperatureScale::Kelvin => write!(f, "{} {}", self.value, self.scale.symbol()),
            _ => write!(f, "{}{}", self.value, self.scale.symbol()),
        }
    }
}

impl PartialEq for Temperature {
    fn eq(&self, other: &Self) -> bool {
        self.to_kelvin_scale() == other.to_kelvin_scale()
    }
}

impl PartialOrd for Temperature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_kelvin_scale().partial_cmp(&other.to_kelvin_scale())
    }
}

// Addition treats right operand as degrees (not scale)
impl Add for Temperature {
    type Output = Temperature;

    fn add(self, rhs: Self) -> Self::Output {
        let rhs_degrees = rhs.convert(self.scale, false).value;
        Temperature::new(self.value + rhs_degrees, self.scale)
    }
}

// Subtraction treats right operand as degrees (not scale)
impl Sub for Temperature {
    type Output = Temperature;

    fn sub(self, rhs: Self) -> Self::Output {
        let rhs_degrees = rhs.convert(self.scale, false).value;
        Temperature::new(self.value - rhs_degrees, self.scale)
    }
}

impl Mul<f64> for Temperature {
    type Output = Temperature;

    fn mul(self, rhs: f64) -> Self::Output {
        Temperature::new(self.value * rhs, self.scale)
    }
}

impl Mul<Temperature> for f64 {
    type Output = Temperature;

    fn mul(self, rhs: Temperature) -> Self::Output {
        Temperature::new(self * rhs.value, rhs.scale)
    }
}

impl Neg for Temperature {
    type Output = Temperature;

    fn neg(self) -> Self::Output {
        Temperature::new(-self.value, self.scale)
    }
}

// Cross-quantity: Temperature * ThermalCapacity = Energy
use super::thermal_capacity::ThermalCapacity;
use crate::core::Quantity;
use crate::energy::{Energy, EnergyUnit};

impl Mul<ThermalCapacity> for Temperature {
    type Output = Energy;

    fn mul(self, rhs: ThermalCapacity) -> Self::Output {
        let joules = self.to_kelvin_scale() * rhs.to_joules_per_kelvin();
        Energy::new(joules, EnergyUnit::Joules)
    }
}

/// Extension trait for creating Temperature quantities from numeric types.
pub trait TemperatureConversions {
    /// Creates a Temperature in Kelvin.
    fn kelvin(self) -> Temperature;
    /// Creates a Temperature in Celsius.
    fn celsius(self) -> Temperature;
    /// Creates a Temperature in Fahrenheit.
    fn fahrenheit(self) -> Temperature;
    /// Creates a Temperature in Rankine.
    fn rankine(self) -> Temperature;
}

impl TemperatureConversions for f64 {
    fn kelvin(self) -> Temperature {
        Temperature::kelvin(self)
    }
    fn celsius(self) -> Temperature {
        Temperature::celsius(self)
    }
    fn fahrenheit(self) -> Temperature {
        Temperature::fahrenheit(self)
    }
    fn rankine(self) -> Temperature {
        Temperature::rankine(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit_scale() {
        let t = Temperature::celsius(100.0);
        assert!((t.to_fahrenheit_scale() - 212.0).abs() < 1e-10);
    }

    #[test]
    fn test_fahrenheit_to_celsius_scale() {
        let t = Temperature::fahrenheit(32.0);
        assert!((t.to_celsius_scale() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_celsius_to_kelvin_scale() {
        let t = Temperature::celsius(0.0);
        assert!((t.to_kelvin_scale() - 273.15).abs() < 1e-10);
    }

    #[test]
    fn test_kelvin_to_celsius_scale() {
        let t = Temperature::kelvin(373.15);
        assert!((t.to_celsius_scale() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_fahrenheit_to_kelvin_scale() {
        let t = Temperature::fahrenheit(32.0);
        assert!((t.to_kelvin_scale() - 273.15).abs() < 0.01);
    }

    #[test]
    fn test_kelvin_to_rankine_scale() {
        let t = Temperature::kelvin(300.0);
        assert!((t.to_rankine_scale() - 540.0).abs() < 1e-10);
    }

    #[test]
    fn test_absolute_zero() {
        let t = Temperature::kelvin(0.0);
        assert!((t.to_celsius_scale() - (-273.15)).abs() < 1e-10);
        assert!((t.to_fahrenheit_scale() - (-459.67)).abs() < 0.01);
        assert!((t.to_rankine_scale() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_degree_conversions() {
        // 5 degrees C = 9 degrees F
        let t = Temperature::celsius(5.0);
        assert!((t.to_fahrenheit_degrees() - 9.0).abs() < 1e-10);

        // 1 degree K = 1 degree C
        let t = Temperature::kelvin(1.0);
        assert!((t.to_celsius_degrees() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_mixed_scale_subtraction() {
        // 100°F - 5°C (as degrees) = 100 - 9 = 91°F
        let t = Temperature::fahrenheit(100.0) - Temperature::celsius(5.0);
        assert!((t.value() - 91.0).abs() < 1e-10);
        assert_eq!(t.scale(), TemperatureScale::Fahrenheit);
    }

    #[test]
    fn test_mixed_scale_addition() {
        // 50°F + 10°C (as degrees) = 50 + 18 = 68°F
        let t = Temperature::fahrenheit(50.0) + Temperature::celsius(10.0);
        assert!((t.value() - 68.0).abs() < 1e-10);
    }

    #[test]
    fn test_temperature_comparison() {
        let boiling_c = Temperature::celsius(100.0);
        let boiling_f = Temperature::fahrenheit(212.0);
        assert!((boiling_c.to_kelvin_scale() - boiling_f.to_kelvin_scale()).abs() < 1e-10);

        let freezing = Temperature::celsius(0.0);
        assert!(freezing < boiling_c);
    }

    #[test]
    fn test_display() {
        let k = Temperature::kelvin(300.0);
        assert_eq!(format!("{}", k), "300 K");

        let c = Temperature::celsius(100.0);
        assert_eq!(format!("{}", c), "100°C");

        let f = Temperature::fahrenheit(72.0);
        assert_eq!(format!("{}", f), "72°F");
    }

    #[test]
    fn test_in_scale() {
        let t = Temperature::celsius(100.0);
        let t_f = t.in_fahrenheit();
        assert_eq!(t_f.scale(), TemperatureScale::Fahrenheit);
        assert!((t_f.value() - 212.0).abs() < 1e-10);
    }
}
