//! Temperature and thermal — cooking, weather, and extreme environments.
//!
//! Run with: `cargo run --example thermal`

use rquants::prelude::*;

fn main() {
    println!("==========================================");
    println!("  RQuants Thermal — Hot and Cold Edition");
    println!("==========================================\n");

    temperature_scales();
    extreme_temperatures();
    cooking_temps();
    thermal_capacity();
}

/// The same temperature looks different depending on the scale
fn temperature_scales() {
    println!("--- Temperature Scales ---\n");

    // Body temperature
    let body = Temperature::fahrenheit(98.6);
    println!(
        "Body temp: {:.1}°F = {:.1}°C = {:.2} K",
        body.to_fahrenheit_scale(),
        body.to_celsius_scale(),
        body.to_kelvin_scale()
    );

    // Room temperature
    let room = Temperature::celsius(22.0);
    println!(
        "Comfy room: {:.0}°C = {:.1}°F = {:.2} K",
        room.to_celsius_scale(),
        room.to_fahrenheit_scale(),
        room.to_kelvin_scale()
    );

    // The crossover: -40 is the same in both Celsius and Fahrenheit!
    let crossover = Temperature::celsius(-40.0);
    println!(
        "Fun fact: -40°C = {:.0}°F (they're the same number!)",
        crossover.to_fahrenheit_scale()
    );

    // Absolute zero: the coldest possible temperature
    let absolute_zero = Temperature::kelvin(0.0);
    println!(
        "Absolute zero: 0 K = {:.2}°C = {:.2}°F (atoms stop moving)",
        absolute_zero.to_celsius_scale(),
        absolute_zero.to_fahrenheit_scale()
    );

    println!();
}

/// Extreme temperatures in the universe
fn extreme_temperatures() {
    println!("--- Extreme Temperatures ---\n");

    // Surface of the Sun: ~5,500°C
    let sun_surface = Temperature::celsius(5500.0);
    println!(
        "Sun's surface: {:.0}°C = {:.0}°F = {:.0} K",
        sun_surface.to_celsius_scale(),
        sun_surface.to_fahrenheit_scale(),
        sun_surface.to_kelvin_scale()
    );

    // Core of the Sun: ~15,000,000°C
    let sun_core = Temperature::celsius(15_000_000.0);
    println!(
        "Sun's core: {:.0}°C = {:.0} K (nuclear fusion happens here)",
        sun_core.to_celsius_scale(),
        sun_core.to_kelvin_scale()
    );

    // Coldest recorded on Earth: -89.2°C (Antarctica, 1983)
    let coldest = Temperature::celsius(-89.2);
    println!(
        "Coldest on Earth: {:.1}°C = {:.1}°F (Vostok Station, Antarctica)",
        coldest.to_celsius_scale(),
        coldest.to_fahrenheit_scale()
    );

    // Hottest recorded on Earth: 56.7°C (Death Valley, 1913)
    let hottest = Temperature::celsius(56.7);
    println!(
        "Hottest on Earth: {:.1}°C = {:.1}°F (Death Valley)",
        hottest.to_celsius_scale(),
        hottest.to_fahrenheit_scale()
    );

    // Liquid nitrogen: -196°C
    let liquid_n2 = Temperature::celsius(-196.0);
    println!(
        "Liquid nitrogen: {:.0}°C = {:.0}°F = {:.2} K",
        liquid_n2.to_celsius_scale(),
        liquid_n2.to_fahrenheit_scale(),
        liquid_n2.to_kelvin_scale()
    );

    println!();
}

/// Cooking temperatures every home chef should know
fn cooking_temps() {
    println!("--- Cooking Temperatures ---\n");

    let water_boil = Temperature::celsius(100.0);
    let water_freeze = Temperature::celsius(0.0);
    println!(
        "Water freezes: {:.0}°C = {:.0}°F",
        water_freeze.to_celsius_scale(),
        water_freeze.to_fahrenheit_scale()
    );
    println!(
        "Water boils: {:.0}°C = {:.0}°F",
        water_boil.to_celsius_scale(),
        water_boil.to_fahrenheit_scale()
    );

    // Medium-rare steak: 57°C internal
    let steak = Temperature::celsius(57.0);
    println!(
        "Medium-rare steak: {:.0}°C = {:.0}°F (chef's kiss)",
        steak.to_celsius_scale(),
        steak.to_fahrenheit_scale()
    );

    // Pizza oven: 450°C
    let pizza = Temperature::celsius(450.0);
    println!(
        "Neapolitan pizza oven: {:.0}°C = {:.0}°F (90 seconds to perfection)",
        pizza.to_celsius_scale(),
        pizza.to_fahrenheit_scale()
    );

    // Candy making: hard crack stage 300°F
    let hard_crack = Temperature::fahrenheit(300.0);
    println!(
        "Hard crack (candy): {:.0}°F = {:.0}°C",
        hard_crack.to_fahrenheit_scale(),
        hard_crack.to_celsius_scale()
    );

    println!();
}

/// Thermal capacity: how much energy to heat stuff up
fn thermal_capacity() {
    println!("--- Thermal Capacity ---\n");

    // Water has a famously high specific heat: 4.184 J/(g·K)
    // Thermal capacity of 1 kg of water: 4184 J/K
    let water_cap = ThermalCapacity::joules_per_kelvin(4184.0);
    let temp_rise = 80.0; // from 20°C to 100°C
    let energy_needed = water_cap.to_joules_per_kelvin() * temp_rise;
    println!(
        "Boil 1 kg water (20°C → 100°C): {:.0} J/K x {:.0} K = {:.0} kJ",
        water_cap.to_joules_per_kelvin(),
        temp_rise,
        energy_needed / 1000.0
    );

    // Iron: specific heat ~0.45 J/(g·K) → 450 J/K per kg
    let iron_cap = ThermalCapacity::joules_per_kelvin(450.0);
    let same_energy = water_cap.to_joules_per_kelvin() * temp_rise;
    let iron_rise = same_energy / iron_cap.to_joules_per_kelvin();
    println!(
        "Same energy heats 1 kg iron by {:.0} K vs water by {:.0} K",
        iron_rise, temp_rise
    );
    println!("(That's why metal pans heat up way faster than water!)");

    println!("\nAll thermal examples done!");
}
