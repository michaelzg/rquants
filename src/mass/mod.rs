//! Mass module containing mass, density, and related quantities.
//!
//! This module provides types for representing and working with mass-related
//! physical quantities including:
//!
//! - [`Mass`] - Amount of matter (kg, lb, etc.)
//! - [`Density`] - Mass per unit volume (kg/m³, g/L, etc.)
//! - [`AreaDensity`] - Mass per unit area (kg/m², lb/ac, etc.)
//! - [`ChemicalAmount`] - Amount of substance (mol)
//! - [`MomentOfInertia`] - Rotational inertia (kg·m²)

pub mod area_density;
pub mod chemical_amount;
pub mod density;
pub mod mass;
pub mod moment_of_inertia;

pub use area_density::{AreaDensity, AreaDensityConversions, AreaDensityUnit};
pub use chemical_amount::{ChemicalAmount, ChemicalAmountConversions, ChemicalAmountUnit};
pub use density::{Density, DensityConversions, DensityUnit};
pub use mass::{Mass, MassConversions, MassUnit};
pub use moment_of_inertia::{MomentOfInertia, MomentOfInertiaConversions, MomentOfInertiaUnit};
