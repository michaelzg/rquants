//! Space and mass — from atoms to galaxies, and feathers to freight trains.
//!
//! Run with: `cargo run --example space_and_mass`

use rquants::prelude::*;

fn main() {
    println!("==========================================");
    println!("  RQuants Space & Mass — Scale of Things");
    println!("==========================================\n");

    lengths_at_every_scale();
    areas_and_volumes();
    angles();
    mass_and_density();
    frequency();
}

/// Length: from nanometers to light-years
fn lengths_at_every_scale() {
    println!("--- Length at Every Scale ---\n");

    // Width of a human hair: ~0.07 mm
    let hair = Length::millimeters(0.07);
    println!(
        "Human hair: {:.2} mm = {:.4} cm",
        hair.to_millimeters(),
        hair.to_centimeters()
    );

    // Wingspan of a Boeing 747: 64.4 m
    let wingspan = Length::meters(64.4);
    println!(
        "Boeing 747 wingspan: {:.1} m = {:.0} ft",
        wingspan.to_meters(),
        wingspan.to_feet()
    );

    // Diameter of Earth: 12,742 km
    let earth = Length::kilometers(12_742.0);
    println!(
        "Earth diameter: {:.0} km = {:.0} mi",
        earth.to_kilometers(),
        earth.to_miles()
    );

    // Distance to the Moon: ~384,400 km
    let moon = Length::kilometers(384_400.0);
    println!(
        "Earth to Moon: {:.0} km = {:.0} mi",
        moon.to_kilometers(),
        moon.to_miles()
    );

    // Distance to the Sun: 1 AU
    let sun = Length::astronomical_units(1.0);
    println!(
        "Earth to Sun: 1 AU = {:.0} km",
        sun.to_kilometers()
    );

    // Nearest star (Proxima Centauri): ~4.24 light-years = ~40.14 trillion km
    let proxima = Length::light_years(4.24);
    println!(
        "Proxima Centauri: 4.24 ly = {:.2e} km",
        proxima.to_kilometers()
    );

    println!();
}

/// Areas and volumes you can actually picture
fn areas_and_volumes() {
    println!("--- Areas & Volumes ---\n");

    // Soccer field: ~105m x 68m
    let soccer = Length::meters(105.0) * Length::meters(68.0);
    println!(
        "Soccer pitch: {:.0} m² = {:.2} acres",
        soccer.to_square_meters(),
        soccer.to_acres()
    );

    // Central Park NYC: ~3.41 km²
    let central_park = Area::hectares(341.0);
    println!(
        "Central Park: {:.0} hectares = {:.2} km² = {:.0} acres",
        central_park.to_hectares(),
        central_park.to_square_kilometers(),
        central_park.to_acres()
    );

    // A can of soda: 355 mL
    let soda = Volume::milliliters(355.0);
    println!(
        "Soda can: {:.0} mL = {:.2} liters = {:.2} cubic feet",
        soda.to_milliliters(),
        soda.to_liters(),
        soda.to_cubic_feet()
    );

    // An Olympic swimming pool: 2,500 m³
    let pool = Volume::cubic_meters(2500.0);
    println!(
        "Olympic pool: {:.0} m³ = {:.0} liters = {:.0} gallons",
        pool.to_cubic_meters(),
        pool.to_liters(),
        pool.to_us_gallons()
    );

    println!();
}

/// Angles: rotation, tilt, and trigonometry
fn angles() {
    println!("--- Angles ---\n");

    // Full circle
    let full = Angle::degrees(360.0);
    println!(
        "Full circle: {:.0}° = {:.4} rad",
        full.to_degrees(),
        full.to_radians()
    );

    // Earth's axial tilt: 23.44°
    let tilt = Angle::degrees(23.44);
    println!(
        "Earth's axial tilt: {:.2}° = {:.4} rad (gives us seasons!)",
        tilt.to_degrees(),
        tilt.to_radians()
    );

    // Pizza slice (8 slices): 45°
    let slice = Angle::degrees(45.0);
    println!(
        "Pizza slice (1/8): {:.0}° = {:.4} rad",
        slice.to_degrees(),
        slice.to_radians()
    );

    println!();
}

/// Mass and density: from hummingbirds to neutron stars
fn mass_and_density() {
    println!("--- Mass & Density ---\n");

    // A smartphone: ~200g
    let phone = Mass::grams(200.0);
    println!(
        "Smartphone: {:.0} g = {:.2} oz",
        phone.to_grams(),
        phone.to_ounces()
    );

    // An African elephant: ~6,000 kg
    let elephant = Mass::kilograms(6000.0);
    println!(
        "African elephant: {:.0} kg = {:.0} lbs = {:.0} tons (short)",
        elephant.to_kilograms(),
        elephant.to_pounds(),
        elephant.to_pounds() / 2000.0
    );

    // Density of water vs honey vs mercury
    let water = Density::kilograms_per_liter(1.0);
    let honey = Density::kilograms_per_liter(1.42);
    let mercury = Density::kilograms_per_liter(13.6);
    println!(
        "Water: {:.1} kg/L, Honey: {:.2} kg/L, Mercury: {:.1} kg/L",
        water.to_kilograms_per_liter(),
        honey.to_kilograms_per_liter(),
        mercury.to_kilograms_per_liter()
    );
    println!(
        "Mercury is {:.0}x denser than water — iron floats on it!",
        mercury.to_kilograms_per_liter() / water.to_kilograms_per_liter()
    );

    // Avogadro: 1 mole of water = 18g
    let one_mole = ChemicalAmount::moles(1.0);
    let water_mass = Mass::grams(18.015);
    println!(
        "1 mole of water ({:.3}g) = 6.022 x 10²³ molecules",
        water_mass.to_grams()
    );
    let _ = one_mole; // used for context

    println!();
}

/// Frequency: from heartbeats to WiFi
fn frequency() {
    println!("--- Frequency ---\n");

    // Resting heart rate: ~72 bpm = 1.2 Hz
    let heartbeat = Frequency::hertz(1.2);
    println!(
        "Resting heart: {:.1} Hz = {:.0} bpm",
        heartbeat.to_hertz(),
        heartbeat.to_hertz() * 60.0
    );

    // Middle C on a piano: 261.63 Hz
    let middle_c = Frequency::hertz(261.63);
    println!(
        "Middle C (piano): {:.2} Hz",
        middle_c.to_hertz()
    );

    // FM radio: ~100 MHz
    let fm = Frequency::megahertz(100.0);
    println!(
        "FM radio: {:.0} MHz = {:.0} kHz",
        fm.to_megahertz(),
        fm.to_kilohertz()
    );

    // WiFi 5 GHz band
    let wifi = Frequency::gigahertz(5.0);
    println!(
        "WiFi 5 GHz: {:.0} GHz = {:.0} MHz",
        wifi.to_gigahertz(),
        wifi.to_megahertz()
    );

    // CPU clock speed: ~5 GHz
    let cpu = Frequency::gigahertz(5.0);
    println!(
        "Modern CPU: {:.1} GHz = {:.0} MHz (billions of cycles per second!)",
        cpu.to_gigahertz(),
        cpu.to_megahertz()
    );

    println!("\nAll space & mass examples done!");
}
