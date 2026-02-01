//! Basic usage examples for RQuants — the fun way.
//!
//! Run with: `cargo run --example basic_usage`

use rquants::prelude::*;

fn main() {
    println!("========================================");
    println!("  RQuants Basic Usage — Greatest Hits");
    println!("========================================\n");

    creating_quantities();
    dsl_syntax();
    unit_conversions();
    quantity_comparison();
}

/// Creating quantities from real-world facts
fn creating_quantities() {
    println!("--- Creating Quantities ---\n");

    // Usain Bolt's 100m world record (9.58s, set 2009)
    let distance = Length::meters(100.0);
    let time = Time::seconds(9.58);
    let speed = distance / time;
    println!(
        "Usain Bolt: {} in {} = {:.2} m/s ({:.2} km/h)",
        distance, time,
        speed.to_meters_per_second(),
        speed.to_kilometers_per_hour()
    );

    // Height of the Burj Khalifa
    let burj = Length::meters(828.0);
    println!(
        "Burj Khalifa: {:.0} m = {:.0} ft (tallest building on Earth)",
        burj.to_meters(),
        burj.to_feet()
    );

    // Mass of a blue whale
    let whale = Mass::kilograms(150_000.0);
    println!(
        "Blue whale: {:.0} kg = {:.0} lbs = {:.0} tonnes",
        whale.to_kilograms(),
        whale.to_pounds(),
        whale.to_kilograms() / 1000.0
    );

    println!();
}

/// DSL syntax — just append the unit to any number
fn dsl_syntax() {
    println!("--- DSL Syntax (the slick way) ---\n");

    // Area of a basketball court (28m x 15m)
    let length = 28.0.meters();
    let width = 15.0.meters();
    let court = length * width;
    println!(
        "NBA court: {} x {} = {:.0} m² = {:.0} ft²",
        length, width,
        court.to_square_meters(),
        court.to_square_feet()
    );

    // Volume of an Olympic swimming pool (50m x 25m x 2m)
    let pool_area = 50.0.meters() * 25.0.meters();
    let depth = 2.0.meters();
    let pool_vol = pool_area.to_square_meters() * depth.to_meters();
    let pool = Volume::cubic_meters(pool_vol);
    println!(
        "Olympic pool: {:.0} m³ = {:.0} liters",
        pool.to_cubic_meters(),
        pool.to_liters()
    );

    println!();
}

/// Converting between units
fn unit_conversions() {
    println!("--- Unit Conversions ---\n");

    // Marathon distance
    let marathon = Length::kilometers(42.195);
    println!(
        "Marathon: {:.3} km = {:.2} mi = {:.0} ft",
        marathon.to_kilometers(),
        marathon.to_miles(),
        marathon.to_feet()
    );

    // Speed of sound at sea level
    let mach1 = Velocity::meters_per_second(343.0);
    println!(
        "Speed of sound: {:.0} m/s = {:.0} km/h = {:.0} mph",
        mach1.to_meters_per_second(),
        mach1.to_kilometers_per_hour(),
        mach1.to_miles_per_hour()
    );

    // Boiling point of water
    let boiling = Temperature::celsius(100.0);
    println!(
        "Water boils: {:.0}°C = {:.0}°F = {:.2} K",
        boiling.to_celsius_scale(),
        boiling.to_fahrenheit_scale(),
        boiling.to_kelvin_scale()
    );

    // Atmospheric pressure
    let atm = Pressure::atmospheres(1.0);
    println!(
        "Sea level: 1 atm = {:.0} Pa = {:.2} psi = {:.0} mmHg",
        atm.to_pascals(),
        atm.to_psi(),
        atm.to_millimeters_of_mercury()
    );

    println!();
}

/// Quantities in different units are equal if they represent the same amount
fn quantity_comparison() {
    println!("--- Quantity Comparison ---\n");

    let a = Length::meters(1000.0);
    let b = Length::kilometers(1.0);
    println!("1000 m == 1 km? {}", a == b);

    let c = Mass::kilograms(1.0);
    let d = Mass::grams(1000.0);
    println!("1 kg == 1000 g? {}", c == d);

    let e = Time::minutes(1.0);
    let f = Time::seconds(60.0);
    println!("1 min == 60 s? {}", e == f);

    println!("\nAll basic examples done!");
}
