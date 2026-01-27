//! Core module containing fundamental traits and types.
//!
//! This module provides the building blocks for all quantities:
//!
//! - [`Quantity`] - The core trait for all measurable quantities
//! - [`UnitOfMeasure`] - Trait for units of measurement
//! - [`Dimension`] - Trait for dimension metadata and parsing
//! - [`Dimensionless`] - Quantities without physical dimension
//! - [`quantity_range::QuantityRange`] - Ranges of quantities
//! - [`ratio::Ratio`] - Ratios between quantities

pub mod dimension;
pub mod dimensionless;
pub mod error;
pub mod quantity;
pub mod quantity_range;
pub mod ratio;
pub mod unit;

pub use dimension::Dimension;
pub use dimensionless::{Dimensionless, DimensionlessConversions, DimensionlessUnit};
pub use quantity::Quantity;
pub use unit::UnitOfMeasure;
