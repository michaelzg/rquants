//! Core module containing fundamental traits and types.
//!
//! This module provides the building blocks for all quantities:
//!
//! - [`Quantity`] - The core trait for all measurable quantities
//! - [`UnitOfMeasure`] - Trait for units of measurement
//! - [`Dimension`] - Trait for dimension metadata and parsing

pub mod dimension;
pub mod error;
pub mod quantity;
pub mod unit;

pub use dimension::Dimension;
pub use quantity::Quantity;
pub use unit::UnitOfMeasure;
