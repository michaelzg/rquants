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
}

impl fmt::Display for TemperatureScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

// ===== Scale conversion functions =====
// These adjust for zero offsets (thermometer readings)

fn celsius_to_fahrenheit_scale(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius_scale(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_kelvin_scale(c: f64) -> f64 {
    c + 273.15
}

fn kelvin_to_celsius_scale(k: f64) -> f64 {
    k - 273.15
}

fn fahrenheit_to_kelvin_scale(f: f64) -> f64 {
    (f + 459.67) * 5.0 / 9.0
}

fn kelvin_to_fahrenheit_scale(k: f64) -> f64 {
    k * 9.0 / 5.0 - 459.67
}

fn kelvin_to_rankine_scale(k: f64) -> f64 {
    k * 9.0 / 5.0
}

fn rankine_to_kelvin_scale(r: f64) -> f64 {
    r * 5.0 / 9.0
}

fn celsius_to_rankine_scale(c: f64) -> f64 {
    (c + 273.15) * 9.0 / 5.0
}

fn rankine_to_celsius_scale(r: f64) -> f64 {
    (r - 491.67) * 5.0 / 9.0
}

fn fahrenheit_to_rankine_scale(f: f64) -> f64 {
    f + 459.67
}

fn rankine_to_fahrenheit_scale(r: f64) -> f64 {
    r - 459.67
}

// ===== Degree conversion functions =====
// These only convert magnitudes (no zero offset adjustment)

fn celsius_to_fahrenheit_degrees(c: f64) -> f64 {
    c * 9.0 / 5.0
}

fn fahrenheit_to_celsius_degrees(f: f64) -> f64 {
    f * 5.0 / 9.0
}

fn kelvin_to_fahrenheit_degrees(k: f64) -> f64 {
    k * 9.0 / 5.0
}

fn fahrenheit_to_kelvin_degrees(f: f64) -> f64 {
    f * 5.0 / 9.0
}

fn kelvin_to_rankine_degrees(k: f64) -> f64 {
    k * 9.0 / 5.0
}

fn rankine_to_kelvin_degrees(r: f64) -> f64 {
    r * 5.0 / 9.0
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
        use TemperatureScale::*;

        if self.scale == target {
            return *self;
        }

        let v = self.value;

        if with_offset {
            // Scale conversions (adjust for zero offset)
            let converted = match (self.scale, target) {
                (Celsius, Fahrenheit) => celsius_to_fahrenheit_scale(v),
                (Fahrenheit, Celsius) => fahrenheit_to_celsius_scale(v),
                (Celsius, Kelvin) => celsius_to_kelvin_scale(v),
                (Kelvin, Celsius) => kelvin_to_celsius_scale(v),
                (Fahrenheit, Kelvin) => fahrenheit_to_kelvin_scale(v),
                (Kelvin, Fahrenheit) => kelvin_to_fahrenheit_scale(v),
                (Kelvin, Rankine) => kelvin_to_rankine_scale(v),
                (Rankine, Kelvin) => rankine_to_kelvin_scale(v),
                (Celsius, Rankine) => celsius_to_rankine_scale(v),
                (Rankine, Celsius) => rankine_to_celsius_scale(v),
                (Fahrenheit, Rankine) => fahrenheit_to_rankine_scale(v),
                (Rankine, Fahrenheit) => rankine_to_fahrenheit_scale(v),
                _ => unreachable!(),
            };
            Temperature::new(converted, target)
        } else {
            // Degree conversions (magnitude only)
            let converted = match (self.scale, target) {
                (Celsius, Fahrenheit) => celsius_to_fahrenheit_degrees(v),
                (Fahrenheit, Celsius) => fahrenheit_to_celsius_degrees(v),
                (Celsius, Kelvin) | (Kelvin, Celsius) => v, // 1:1 ratio
                (Fahrenheit, Kelvin) => fahrenheit_to_kelvin_degrees(v),
                (Kelvin, Fahrenheit) => kelvin_to_fahrenheit_degrees(v),
                (Kelvin, Rankine) => kelvin_to_rankine_degrees(v),
                (Rankine, Kelvin) => rankine_to_kelvin_degrees(v),
                (Celsius, Rankine) => celsius_to_fahrenheit_degrees(v), // same ratio as C->F
                (Rankine, Celsius) => fahrenheit_to_celsius_degrees(v), // same ratio as F->C
                (Fahrenheit, Rankine) | (Rankine, Fahrenheit) => v, // 1:1 ratio
                _ => unreachable!(),
            };
            Temperature::new(converted, target)
        }
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
        let a = self.to_kelvin_scale();
        let b = other.to_kelvin_scale();
        // Use a relative tolerance for temperature scale conversions
        // which involve addition/subtraction of offsets
        let max_abs = a.abs().max(b.abs()).max(1.0);
        (a - b).abs() < max_abs * 1e-12
    }
}

impl PartialOrd for Temperature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_kelvin_scale()
            .partial_cmp(&other.to_kelvin_scale())
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
        assert_eq!(boiling_c, boiling_f);

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
