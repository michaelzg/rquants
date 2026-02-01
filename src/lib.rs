//! # RQuants
//!
//! A Rust port of the Scala [squants](https://github.com/typelevel/squants) library
//! for quantities, units of measure, and dimensional analysis.
//!
//! RQuants provides type-safe dimensional analysis with extensive unit coverage
//! across 12 domains and an ergonomic API for creating and manipulating
//! physical quantities.
//!
//! ## Quick Start
//!
//! ```rust
//! use rquants::prelude::*;
//!
//! // Create quantities
//! let distance = Length::meters(100.0);
//! let time = Time::seconds(9.58);
//! let speed = distance / time;  // Returns Velocity
//! assert!((speed.to_meters_per_second() - 10.438).abs() < 0.001);
//!
//! // Newton's second law: F = ma
//! let mass = Mass::kilograms(10.0);
//! let accel = Acceleration::meters_per_second_squared(9.8);
//! let force = mass * accel;
//! assert!((force.to_newtons() - 98.0).abs() < 1e-10);
//!
//! // Energy and power
//! let power = Power::kilowatts(1.0);
//! let duration = Time::hours(2.0);
//! let energy = power * duration;
//! assert!((energy.to_kilowatt_hours() - 2.0).abs() < 1e-10);
//!
//! // DSL syntax via extension traits
//! let v = 100.0.meters_per_second();
//! let t = 5.0.seconds();
//! let d = v * t;
//! assert!((d.to_meters() - 500.0).abs() < 1e-10);
//! ```
//!
//! ## Features
//!
//! - **Type-safe dimensional analysis**: Cross-quantity operations return correct types
//! - **12 domain modules**: time, space, mass, motion, energy, thermal, electro, information, radio, photo, market
//! - **Ergonomic DSL**: `100.0.meters()`, `5.0.seconds()`, etc.
//! - **Extensive unit coverage**: 200+ units across all domains
//! - **Approximate equality**: `approx_eq` for floating-point comparisons
//! - **Temperature**: Special scale vs. degree conversions (Kelvin, Celsius, Fahrenheit, Rankine)
//! - **Financial**: Money, Currency, Exchange Rates, and generic `Price<Q>`
//!
//! ## Modules
//!
//! - [`core`] - Core traits (Quantity, UnitOfMeasure, Dimension), dimensionless, ranges, ratios
//! - [`systems`] - Metric and binary prefix systems
//! - [`time`] - Time and frequency
//! - [`space`] - Length, area, volume, angle, solid angle
//! - [`mass`] - Mass, density, area density, chemical amount, moment of inertia
//! - [`motion`] - Velocity, acceleration, force, momentum, pressure
//! - [`energy`] - Energy, power, power ramp, specific energy, energy density, molar energy
//! - [`thermal`] - Temperature (with scale/degree conversions), thermal capacity
//! - [`electro`] - Current, charge, potential, resistance, capacitance, inductance, magnetic flux
//! - [`information`] - Information (bytes/bits with metric/binary prefixes), data rate
//! - [`radio`] - Activity, dose, irradiance, radiance, spectral power, particle flux
//! - [`photo`] - Luminous intensity, flux, illuminance, luminance, luminous energy
//! - [`market`] - Money, currency, exchange rates, generic Price&lt;Q&gt;

// Module names intentionally mirror their parent (e.g., energy::energy, mass::mass)
// to match the Scala squants source structure.
#![allow(clippy::module_inception)]

pub mod core;
pub mod electro;
pub mod energy;
pub mod information;
pub mod market;
pub mod mass;
pub mod motion;
pub mod photo;
pub mod prelude;
pub mod radio;
pub mod space;
pub mod systems;
pub mod thermal;
pub mod time;

// Re-export commonly used items at crate root
pub use crate::core::error::{QuantityError, QuantityParseError};
pub use crate::core::{Dimension, Quantity, UnitOfMeasure};
