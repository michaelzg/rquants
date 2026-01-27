//! # RQuants
//!
//! A Rust library for quantities, units of measure, and dimensional analysis.
//!
//! RQuants provides type-safe dimensional analysis with compile-time verification,
//! extensive unit coverage across 12 domains, and an ergonomic API for creating
//! and manipulating physical quantities.
//!
//! ## Quick Start
//!
//! ```rust
//! use rquants::prelude::*;
//!
//! // Create quantities with units (coming in Phase 2)
//! // let distance = Length::meters(100.0);
//! // let time = Time::seconds(9.58);
//! // let speed = distance / time;  // Returns Velocity
//! ```
//!
//! ## Features
//!
//! - Type-safe dimensional analysis
//! - Extensive unit coverage across 12 domains
//! - Ergonomic DSL for creating quantities
//! - String parsing and formatting
//! - Approximate equality comparisons
//!
//! ## Modules
//!
//! - [`core`] - Core traits and types (Quantity, UnitOfMeasure, Dimension)
//! - [`systems`] - Metric and binary prefix systems
//! - [`time`] - Time and frequency quantities
//! - [`space`] - Length, area, volume, and angle quantities

pub mod core;
pub mod mass;
pub mod motion;
pub mod prelude;
pub mod space;
pub mod systems;
pub mod time;

// Re-export commonly used items at crate root
pub use crate::core::error::{QuantityError, QuantityParseError};
pub use crate::core::{Dimension, Quantity, UnitOfMeasure};
