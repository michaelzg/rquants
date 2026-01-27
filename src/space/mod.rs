//! Space quantities and units.
//!
//! This module provides:
//! - [`Length`] - A quantity representing a length/distance
//! - [`Area`] - A quantity representing an area (length squared)
//! - [`Volume`] - A quantity representing a volume (length cubed)
//! - [`Angle`] - A quantity representing a plane angle
//! - [`SolidAngle`] - A quantity representing a solid angle
//!
//! # Example
//!
//! ```rust
//! use rquants::prelude::*;
//!
//! // Create length quantities
//! let width = Length::meters(10.0);
//! let height = Length::meters(5.0);
//!
//! // Multiply lengths to get area
//! let area = width * height;
//! assert_eq!(area.to_square_meters(), 50.0);
//! ```

pub mod angle;
pub mod area;
pub mod length;
pub mod solid_angle;
pub mod volume;

pub use angle::{Angle, AngleUnit};
pub use area::{Area, AreaUnit};
pub use length::{Length, LengthUnit};
pub use solid_angle::{SolidAngle, SolidAngleUnit};
pub use volume::{Volume, VolumeUnit};
