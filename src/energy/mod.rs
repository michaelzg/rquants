//! Energy quantities and units.
//!
//! This module provides quantities for energy, power, and related measurements.
//!
//! # Quantities
//!
//! - [`Energy`] - Energy (J, kWh, BTU, eV)
//! - [`Power`] - Power/rate of energy transfer (W, kW, MW)
//! - [`PowerRamp`] - Rate of power change (W/h, kW/h)
//! - [`SpecificEnergy`] - Energy per mass (Gy, rad)
//! - [`EnergyDensity`] - Energy per volume (J/m³)
//! - [`PowerDensity`] - Power per volume (W/m³)
//! - [`MolarEnergy`] - Energy per chemical amount (J/mol)
//!
//! # Example
//!
//! ```rust
//! use rquants::prelude::*;
//!
//! // Power = Energy / Time
//! let energy = Energy::joules(3600.0);
//! let time = Time::hours(1.0);
//! let power = energy / time;
//! assert!((power.to_watts() - 1.0).abs() < 1e-10);
//!
//! // Energy = Power * Time
//! let power = Power::kilowatts(1.0);
//! let time = Time::hours(2.0);
//! let energy = power * time;
//! assert!((energy.to_kilowatt_hours() - 2.0).abs() < 1e-10);
//! ```

pub mod energy;
pub mod energy_density;
pub mod molar_energy;
pub mod power;
pub mod power_density;
pub mod power_ramp;
pub mod specific_energy;

pub use energy::{Energy, EnergyConversions, EnergyUnit};
pub use energy_density::{EnergyDensity, EnergyDensityConversions, EnergyDensityUnit};
pub use molar_energy::{MolarEnergy, MolarEnergyConversions, MolarEnergyUnit};
pub use power::{Power, PowerConversions, PowerUnit};
pub use power_density::{PowerDensity, PowerDensityConversions, PowerDensityUnit};
pub use power_ramp::{PowerRamp, PowerRampConversions, PowerRampUnit};
pub use specific_energy::{SpecificEnergy, SpecificEnergyConversions, SpecificEnergyUnit};
