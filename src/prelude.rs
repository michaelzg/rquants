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

// Dimensionless
pub use crate::core::dimensionless::DimensionlessConversions;
pub use crate::core::{Dimensionless, DimensionlessUnit};

// Error types
pub use crate::core::error::{QuantityError, QuantityParseError};

// Metric and binary prefixes
pub use crate::systems::binary::*;
pub use crate::systems::metric::*;

// Time quantities
pub use crate::time::time::TimeConversions;
pub use crate::time::{Frequency, FrequencyUnit, Time, TimeUnit};

// Space quantities
pub use crate::space::angle::AngleConversions;
pub use crate::space::area::AreaConversions;
pub use crate::space::length::LengthConversions;
pub use crate::space::volume::VolumeConversions;
pub use crate::space::{
    Angle, AngleUnit, Area, AreaUnit, Length, LengthUnit, SolidAngle, SolidAngleUnit, Volume,
    VolumeUnit,
};

// Mass quantities
pub use crate::mass::area_density::AreaDensityConversions;
pub use crate::mass::chemical_amount::ChemicalAmountConversions;
pub use crate::mass::density::DensityConversions;
pub use crate::mass::mass::MassConversions;
pub use crate::mass::moment_of_inertia::MomentOfInertiaConversions;
pub use crate::mass::{
    AreaDensity, AreaDensityUnit, ChemicalAmount, ChemicalAmountUnit, Density, DensityUnit, Mass,
    MassUnit, MomentOfInertia, MomentOfInertiaUnit,
};

// Motion quantities
pub use crate::motion::acceleration::AccelerationConversions;
pub use crate::motion::force::ForceConversions;
pub use crate::motion::momentum::MomentumConversions;
pub use crate::motion::pressure::PressureConversions;
pub use crate::motion::velocity::VelocityConversions;
pub use crate::motion::{
    Acceleration, AccelerationUnit, Force, ForceUnit, Momentum, MomentumUnit, Pressure,
    PressureUnit, Velocity, VelocityUnit,
};
