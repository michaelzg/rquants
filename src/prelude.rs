//! Prelude module for convenient imports.
//!
//! This module re-exports the most commonly used types and traits.
//!
//! ```rust
//! use rquants::prelude::*;
//! ```

// Core traits
pub use crate::core::Dimension;
pub use crate::core::Quantity;
pub use crate::core::UnitOfMeasure;

// Error types
pub use crate::core::error::{QuantityError, QuantityParseError};

// Metric and binary prefixes
pub use crate::systems::binary::*;
pub use crate::systems::metric::*;
