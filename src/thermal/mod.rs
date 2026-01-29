//! Thermal quantities and units.
//!
//! This module provides temperature and thermal capacity quantities.
//!
//! # Special Temperature Handling
//!
//! Temperature is unique because different scales have different zero points.
//! Two types of conversion are supported:
//!
//! - **Scale conversions**: Adjust for zero offset (5°C = 41°F on a thermometer)
//! - **Degree conversions**: No zero adjustment (5°C delta = 9°F delta)
//!
//! Addition and subtraction treat the right operand as a degree quantity
//! (not a scale temperature), allowing mixed-scale expressions.

pub mod temperature;
pub mod thermal_capacity;

pub use temperature::{Temperature, TemperatureConversions, TemperatureScale};
pub use thermal_capacity::{ThermalCapacity, ThermalCapacityConversions, ThermalCapacityUnit};
