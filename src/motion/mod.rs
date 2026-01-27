//! Motion module containing kinematic and dynamic quantities.
//!
//! This module provides types for representing motion-related physical quantities:
//!
//! - [`Velocity`] - Rate of change of position (m/s, km/h, mph)
//! - [`Acceleration`] - Rate of change of velocity (m/s², g)
//! - [`Force`] - Push or pull on an object (N, lbf)
//! - [`Momentum`] - Mass in motion (kg·m/s)
//! - [`Pressure`] - Force per unit area (Pa, bar, psi)

pub mod acceleration;
pub mod force;
pub mod momentum;
pub mod pressure;
pub mod velocity;

pub use acceleration::{Acceleration, AccelerationConversions, AccelerationUnit};
pub use force::{Force, ForceConversions, ForceUnit};
pub use momentum::{Momentum, MomentumConversions, MomentumUnit};
pub use pressure::{Pressure, PressureConversions, PressureUnit};
pub use velocity::{Velocity, VelocityConversions, VelocityUnit};
