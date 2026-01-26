//! Measurement system prefixes and constants.
//!
//! This module provides prefix constants for the metric (SI) and binary systems.
//!
//! # Metric Prefixes
//!
//! The metric system uses powers of 10:
//! - `KILO` = 10^3 = 1,000
//! - `MEGA` = 10^6 = 1,000,000
//! - etc.
//!
//! # Binary Prefixes
//!
//! The binary system uses powers of 1024:
//! - `KIBI` = 2^10 = 1,024
//! - `MEBI` = 2^20 = 1,048,576
//! - etc.

pub mod binary;
pub mod metric;

pub use binary::*;
pub use metric::*;
