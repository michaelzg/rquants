//! Information quantities and units.
//!
//! This module provides quantities for information and data transfer rates.
//!
//! # Quantities
//!
//! - [`Information`] - Digital information (B, KB, MB, GB, bits, Kbits, etc.)
//! - [`DataRate`] - Rate of information transfer (B/s, MB/s, Mbps, Gbps)
//!
//! # Example
//!
//! ```rust
//! use rquants::prelude::*;
//!
//! // DataRate = Information / Time
//! let info = Information::megabytes(100.0);
//! let time = Time::seconds(10.0);
//! let rate = info / time;
//! assert!((rate.to_megabytes_per_second() - 10.0).abs() < 1e-10);
//!
//! // Information = DataRate * Time
//! let rate = DataRate::megabits_per_second(100.0);
//! let time = Time::seconds(10.0);
//! let info = rate * time;
//! assert!((info.to_megabits() - 1000.0).abs() < 1e-10);
//! ```

pub mod data_rate;
pub mod information;

pub use data_rate::{DataRate, DataRateConversions, DataRateUnit};
pub use information::{Information, InformationConversions, InformationUnit};
