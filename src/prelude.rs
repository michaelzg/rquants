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

// Energy quantities
pub use crate::energy::energy::EnergyConversions;
pub use crate::energy::energy_density::EnergyDensityConversions;
pub use crate::energy::molar_energy::MolarEnergyConversions;
pub use crate::energy::power::PowerConversions;
pub use crate::energy::power_density::PowerDensityConversions;
pub use crate::energy::power_ramp::PowerRampConversions;
pub use crate::energy::specific_energy::SpecificEnergyConversions;
pub use crate::energy::{
    Energy, EnergyDensity, EnergyDensityUnit, EnergyUnit, MolarEnergy, MolarEnergyUnit, Power,
    PowerDensity, PowerDensityUnit, PowerRamp, PowerRampUnit, PowerUnit, SpecificEnergy,
    SpecificEnergyUnit,
};

// Information quantities
pub use crate::information::data_rate::DataRateConversions;
pub use crate::information::information::InformationConversions;
pub use crate::information::{DataRate, DataRateUnit, Information, InformationUnit};

// Thermal quantities
pub use crate::thermal::temperature::TemperatureConversions;
pub use crate::thermal::thermal_capacity::ThermalCapacityConversions;
pub use crate::thermal::{Temperature, TemperatureScale, ThermalCapacity, ThermalCapacityUnit};

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

// Electro quantities
pub use crate::electro::capacitance::CapacitanceConversions;
pub use crate::electro::conductivity::ConductivityConversions;
pub use crate::electro::electric_charge::ElectricChargeConversions;
pub use crate::electro::electric_current::ElectricCurrentConversions;
pub use crate::electro::electric_potential::ElectricPotentialConversions;
pub use crate::electro::electrical_conductance::ElectricalConductanceConversions;
pub use crate::electro::electrical_resistance::ElectricalResistanceConversions;
pub use crate::electro::inductance::InductanceConversions;
pub use crate::electro::magnetic_flux::MagneticFluxConversions;
pub use crate::electro::magnetic_flux_density::MagneticFluxDensityConversions;
pub use crate::electro::resistivity::ResistivityConversions;
pub use crate::electro::{
    Capacitance, CapacitanceUnit, Conductivity, ConductivityUnit, ElectricCharge,
    ElectricChargeUnit, ElectricCurrent, ElectricCurrentUnit, ElectricPotential,
    ElectricPotentialUnit, ElectricalConductance, ElectricalConductanceUnit,
    ElectricalResistance, ElectricalResistanceUnit, Inductance, InductanceUnit, MagneticFlux,
    MagneticFluxDensity, MagneticFluxDensityUnit, MagneticFluxUnit, Resistivity,
    ResistivityUnit,
};

// Radio quantities
pub use crate::radio::activity::ActivityConversions;
pub use crate::radio::dose::DoseConversions;
pub use crate::radio::irradiance::IrradianceConversions;
pub use crate::radio::particle_flux::ParticleFluxConversions;
pub use crate::radio::radiance::RadianceConversions;
pub use crate::radio::radiant_intensity::RadiantIntensityConversions;
pub use crate::radio::spectral_irradiance::SpectralIrradianceConversions;
pub use crate::radio::spectral_power::SpectralPowerConversions;
pub use crate::radio::{
    Activity, ActivityUnit, Dose, DoseUnit, Irradiance, IrradianceUnit, ParticleFlux,
    ParticleFluxUnit, Radiance, RadianceUnit, RadiantIntensity, RadiantIntensityUnit,
    SpectralIrradiance, SpectralIrradianceUnit, SpectralPower, SpectralPowerUnit,
};

// Market types
pub use crate::market::money::MoneyConversions;
pub use crate::market::{Currency, CurrencyExchangeRate, Money, Price};

// Photo quantities
pub use crate::photo::illuminance::IlluminanceConversions;
pub use crate::photo::luminance::LuminanceConversions;
pub use crate::photo::luminous_energy::LuminousEnergyConversions;
pub use crate::photo::luminous_exposure::LuminousExposureConversions;
pub use crate::photo::luminous_flux::LuminousFluxConversions;
pub use crate::photo::luminous_intensity::LuminousIntensityConversions;
pub use crate::photo::{
    Illuminance, IlluminanceUnit, Luminance, LuminanceUnit, LuminousEnergy, LuminousEnergyUnit,
    LuminousExposure, LuminousExposureUnit, LuminousFlux, LuminousFluxUnit, LuminousIntensity,
    LuminousIntensityUnit,
};
