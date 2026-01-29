//! Electrical and magnetic quantities.
//!
//! This module provides types for working with electrical and magnetic quantities
//! including current, voltage, resistance, capacitance, inductance, and magnetic fields.

pub mod capacitance;
pub mod conductivity;
pub mod electric_charge;
pub mod electric_current;
pub mod electric_potential;
pub mod electrical_conductance;
pub mod electrical_resistance;
pub mod inductance;
pub mod magnetic_flux;
pub mod magnetic_flux_density;
pub mod resistivity;

pub use capacitance::{Capacitance, CapacitanceUnit};
pub use conductivity::{Conductivity, ConductivityUnit};
pub use electric_charge::{ElectricCharge, ElectricChargeUnit};
pub use electric_current::{ElectricCurrent, ElectricCurrentUnit};
pub use electric_potential::{ElectricPotential, ElectricPotentialUnit};
pub use electrical_conductance::{ElectricalConductance, ElectricalConductanceUnit};
pub use electrical_resistance::{ElectricalResistance, ElectricalResistanceUnit};
pub use inductance::{Inductance, InductanceUnit};
pub use magnetic_flux::{MagneticFlux, MagneticFluxUnit};
pub use magnetic_flux_density::{MagneticFluxDensity, MagneticFluxDensityUnit};
pub use resistivity::{Resistivity, ResistivityUnit};
