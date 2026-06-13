use proptest::prelude::*;
use proptest::test_runner::{FileFailurePersistence, TestCaseError};
use rquants::core::error::QuantityParseError;
use rquants::energy::Energy;
use rquants::prelude::*;
use rquants::{Dimension, Quantity};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

const RELATIVE_TOLERANCE: f64 = 1e-8;

fn close(left: f64, right: f64) -> bool {
    close_with_scale(left, right, 0.0)
}

fn close_with_scale(left: f64, right: f64, scale_hint: f64) -> bool {
    if left == right {
        return true;
    }

    let scale = left.abs().max(right.abs()).max(scale_hint.abs()).max(1.0);
    (left - right).abs() <= scale * RELATIVE_TOLERANCE
}

fn check_dimension<D>(
    value: f64,
    other: f64,
    scale: f64,
    source_index: usize,
    target_index: usize,
) -> Result<(), TestCaseError>
where
    D: Dimension,
    D::Quantity: Add<Output = D::Quantity>
        + Sub<Output = D::Quantity>
        + Mul<f64, Output = D::Quantity>
        + Div<f64, Output = D::Quantity>
        + FromStr<Err = QuantityParseError>,
    <D::Quantity as FromStr>::Err: Debug,
{
    let units = D::units();
    prop_assert!(!units.is_empty());

    let source = units[source_index % units.len()];
    let target = units[target_index % units.len()];
    let quantity = D::Quantity::new(value, source);
    let other_quantity = D::Quantity::new(other, target);
    let operation_scale = quantity
        .to_primary()
        .abs()
        .max(other_quantity.to_primary().abs());

    let round_trip = quantity.in_unit(target).in_unit(source);
    prop_assert!(
        close(round_trip.value(), quantity.value()),
        "round-trip failed for {} via {}: {} vs {}",
        source,
        target,
        round_trip.value(),
        quantity.value()
    );

    let relation_count = u8::from(quantity < other_quantity)
        + u8::from(quantity == other_quantity)
        + u8::from(quantity > other_quantity);
    prop_assert_eq!(relation_count, 1);

    let add_then_subtract = (quantity + other_quantity) - other_quantity;
    prop_assert!(
        close_with_scale(
            add_then_subtract.to_primary(),
            quantity.to_primary(),
            operation_scale
        ),
        "add/sub algebra failed for {}",
        D::name()
    );

    let scaled = (quantity * scale) / scale;
    prop_assert!(
        close(scaled.to_primary(), quantity.to_primary()),
        "scale/divide algebra failed for {}",
        D::name()
    );

    let left_sum = quantity + other_quantity;
    let right_sum = other_quantity + quantity;
    prop_assert!(
        close_with_scale(
            left_sum.to_primary(),
            right_sum.to_primary(),
            operation_scale
        ),
        "commutative addition failed for {}",
        D::name()
    );

    let rendered = quantity.to_string();
    let parsed = D::Quantity::from_str(&rendered)
        .map_err(|err| TestCaseError::fail(format!("parse {rendered:?}: {err:?}")))?;
    prop_assert!(
        close(parsed.to_primary(), quantity.to_primary()),
        "display/parse round-trip failed for {}",
        D::name()
    );

    Ok(())
}

macro_rules! quantity_properties {
    ($($name:ident => $dimension:path;)+) => {
        proptest! {
            #![proptest_config(ProptestConfig {
                cases: 64,
                failure_persistence: Some(Box::new(FileFailurePersistence::Off)),
                .. ProptestConfig::default()
            })]

            $(
                #[test]
                fn $name(
                    value in -1.0e6f64..1.0e6,
                    other in -1.0e6f64..1.0e6,
                    scale in 1.0e-6f64..1.0e6,
                    source_index in any::<usize>(),
                    target_index in any::<usize>(),
                ) {
                    check_dimension::<$dimension>(
                        value,
                        other,
                        scale,
                        source_index,
                        target_index,
                    )?;
                }
            )+
        }
    };
}

quantity_properties! {
    dimensionless_properties => rquants::core::dimensionless::DimensionlessDimension;
    capacitance_properties => rquants::electro::capacitance::CapacitanceDimension;
    conductivity_properties => rquants::electro::conductivity::ConductivityDimension;
    electric_charge_properties => rquants::electro::electric_charge::ElectricChargeDimension;
    electric_current_properties => rquants::electro::electric_current::ElectricCurrentDimension;
    electric_potential_properties => rquants::electro::electric_potential::ElectricPotentialDimension;
    electrical_conductance_properties => rquants::electro::electrical_conductance::ElectricalConductanceDimension;
    electrical_resistance_properties => rquants::electro::electrical_resistance::ElectricalResistanceDimension;
    inductance_properties => rquants::electro::inductance::InductanceDimension;
    magnetic_flux_properties => rquants::electro::magnetic_flux::MagneticFluxDimension;
    magnetic_flux_density_properties => rquants::electro::magnetic_flux_density::MagneticFluxDensityDimension;
    resistivity_properties => rquants::electro::resistivity::ResistivityDimension;
    energy_properties => rquants::energy::energy::EnergyDimension;
    energy_density_properties => rquants::energy::energy_density::EnergyDensityDimension;
    molar_energy_properties => rquants::energy::molar_energy::MolarEnergyDimension;
    power_properties => rquants::energy::power::PowerDimension;
    power_density_properties => rquants::energy::power_density::PowerDensityDimension;
    power_ramp_properties => rquants::energy::power_ramp::PowerRampDimension;
    specific_energy_properties => rquants::energy::specific_energy::SpecificEnergyDimension;
    data_rate_properties => rquants::information::data_rate::DataRateDimension;
    information_properties => rquants::information::information::InformationDimension;
    area_density_properties => rquants::mass::area_density::AreaDensityDimension;
    chemical_amount_properties => rquants::mass::chemical_amount::ChemicalAmountDimension;
    density_properties => rquants::mass::density::DensityDimension;
    mass_properties => rquants::mass::mass::MassDimension;
    moment_of_inertia_properties => rquants::mass::moment_of_inertia::MomentOfInertiaDimension;
    acceleration_properties => rquants::motion::acceleration::AccelerationDimension;
    force_properties => rquants::motion::force::ForceDimension;
    momentum_properties => rquants::motion::momentum::MomentumDimension;
    pressure_properties => rquants::motion::pressure::PressureDimension;
    velocity_properties => rquants::motion::velocity::VelocityDimension;
    illuminance_properties => rquants::photo::illuminance::IlluminanceDimension;
    luminance_properties => rquants::photo::luminance::LuminanceDimension;
    luminous_energy_properties => rquants::photo::luminous_energy::LuminousEnergyDimension;
    luminous_exposure_properties => rquants::photo::luminous_exposure::LuminousExposureDimension;
    luminous_flux_properties => rquants::photo::luminous_flux::LuminousFluxDimension;
    luminous_intensity_properties => rquants::photo::luminous_intensity::LuminousIntensityDimension;
    activity_properties => rquants::radio::activity::ActivityDimension;
    dose_properties => rquants::radio::dose::DoseDimension;
    irradiance_properties => rquants::radio::irradiance::IrradianceDimension;
    particle_flux_properties => rquants::radio::particle_flux::ParticleFluxDimension;
    radiance_properties => rquants::radio::radiance::RadianceDimension;
    radiant_intensity_properties => rquants::radio::radiant_intensity::RadiantIntensityDimension;
    spectral_irradiance_properties => rquants::radio::spectral_irradiance::SpectralIrradianceDimension;
    spectral_power_properties => rquants::radio::spectral_power::SpectralPowerDimension;
    angle_properties => rquants::space::angle::AngleDimension;
    area_properties => rquants::space::area::AreaDimension;
    length_properties => rquants::space::length::LengthDimension;
    solid_angle_properties => rquants::space::solid_angle::SolidAngleDimension;
    volume_properties => rquants::space::volume::VolumeDimension;
    thermal_capacity_properties => rquants::thermal::thermal_capacity::ThermalCapacityDimension;
    frequency_properties => rquants::time::frequency::FrequencyDimension;
    time_properties => rquants::time::time::TimeDimension;
}

#[test]
fn standard_traits_work_for_generated_quantities() {
    let total: Length = [Length::meters(1.0), Length::centimeters(50.0)]
        .into_iter()
        .sum();
    assert_eq!(total.unit(), LengthUnit::Meters);
    assert!(close(total.to_meters(), 1.5));

    let total_from_refs: Length = [Length::meters(1.0), Length::centimeters(50.0)]
        .iter()
        .sum();
    assert_eq!(total_from_refs.unit(), LengthUnit::Meters);
    assert!(close(total_from_refs.to_meters(), 1.5));

    let mut length = Length::meters(1.0);
    length += Length::centimeters(50.0);
    length -= Length::millimeters(250.0);
    length *= 4.0;
    length /= 2.0;
    assert!(close(length.to_meters(), 2.5));

    let parsed: Length = "1.5 km".parse().unwrap();
    assert_eq!(parsed.unit(), LengthUnit::Kilometers);
    assert!(close(parsed.to_meters(), 1500.0));
}

#[test]
fn standard_traits_work_for_dimensionless() {
    let total: Dimensionless = [Dimensionless::each(1.0), Dimensionless::percent(50.0)]
        .into_iter()
        .sum();
    assert_eq!(total.unit(), DimensionlessUnit::Each);
    assert!(close(total.to_each(), 1.5));

    let mut value = Dimensionless::each(1.0);
    value += Dimensionless::percent(50.0);
    value -= Dimensionless::percent(25.0);
    value *= 4.0;
    value /= 2.0;
    assert!(close(value.to_each(), 2.5));

    let parsed: Dimensionless = "50 %".parse().unwrap();
    assert_eq!(parsed.unit(), DimensionlessUnit::Percent);
    assert!(close(parsed.to_each(), 0.5));
}

#[test]
fn electron_volt_uses_exact_si_charge() {
    let joules = Energy::electron_volts(1.0).to_joules();
    assert!((joules - 1.602_176_634e-19).abs() < 1e-30);
}

#[test]
fn electron_volt_equality_does_not_overlap_ordering() {
    let small = Energy::electron_volts(1.0);
    let large = Energy::electron_volts(1000.0);

    assert_ne!(small, large);
    assert!(small < large);
}

#[cfg(feature = "serde")]
#[test]
fn serde_round_trips_quantities_as_display_strings() {
    let json = serde_json::to_string(&Length::meters(3.0)).unwrap();
    assert_eq!(json, "\"3 m\"");

    let length: Length = serde_json::from_str(&json).unwrap();
    assert_eq!(length.unit(), LengthUnit::Meters);
    assert!(close(length.to_meters(), 3.0));

    let dimensionless_json = serde_json::to_string(&Dimensionless::percent(50.0)).unwrap();
    assert_eq!(dimensionless_json, "\"50 %\"");

    let dimensionless: Dimensionless = serde_json::from_str(&dimensionless_json).unwrap();
    assert_eq!(dimensionless.unit(), DimensionlessUnit::Percent);
    assert!(close(dimensionless.to_each(), 0.5));
}
