//! Time quantities and units.
//!
//! This module provides:
//! - [`Time`] - A quantity representing a duration of time
//! - [`Frequency`] - A quantity representing cycles per time
//! - [`TimeIntegral`] / [`TimeDerivative`] - Traits for calculus relationships
//!
//! # Example
//!
//! ```rust
//! use rquants::prelude::*;
//!
//! // Create time quantities
//! let duration = Time::seconds(60.0);
//! assert_eq!(duration.to(TimeUnit::Minutes), 1.0);
//!
//! // Create frequency quantities
//! let freq = Frequency::hertz(1000.0);
//! assert_eq!(freq.to(FrequencyUnit::Kilohertz), 1.0);
//! ```

pub mod frequency;
pub mod time;
pub mod time_derivative;

pub use frequency::{Frequency, FrequencyConversions, FrequencyUnit};
pub use time::{Time, TimeConversions, TimeDimension, TimeUnit};
pub use time_derivative::{SecondTimeDerivative, SecondTimeIntegral, TimeDerivative, TimeIntegral};
