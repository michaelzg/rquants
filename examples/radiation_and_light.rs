//! Radiation and light — X-rays, sunshine, and screen brightness.
//!
//! Run with: `cargo run --example radiation_and_light`

use rquants::prelude::*;

fn main() {
    println!("=============================================");
    println!("  RQuants Radiation & Light — Glow Edition");
    println!("=============================================\n");

    radioactivity();
    radiation_dose();
    solar_irradiance();
    everyday_light();
}

/// Radioactivity: how quickly atoms are falling apart
fn radioactivity() {
    println!("--- Radioactivity ---\n");

    // A banana contains ~15 Bq of potassium-40
    let banana = Activity::becquerels(15.0);
    println!(
        "One banana: {:.0} Bq = {:.10} Ci (the 'banana equivalent dose')",
        banana.to_becquerels(),
        banana.to_curies()
    );

    // Smoke detector: ~33,000 Bq of americium-241
    let smoke_detector = Activity::becquerels(33_000.0);
    println!(
        "Smoke detector: {:.0} Bq = {:.7} Ci (perfectly safe)",
        smoke_detector.to_becquerels(),
        smoke_detector.to_curies()
    );

    // Medical tracer injection: ~370 MBq (10 mCi)
    let pet_scan = Activity::becquerels(370_000_000.0);
    println!(
        "PET scan tracer: {:.0} MBq = {:.1} mCi (decays in hours)",
        pet_scan.to_becquerels() / 1_000_000.0,
        pet_scan.to_curies() * 1000.0
    );

    // Chernobyl reactor 4 at explosion: ~12 EBq (insane)
    let chernobyl = Activity::becquerels(1.2e19);
    println!(
        "Chernobyl (1986): {:.1} EBq = {:.0} MCi",
        chernobyl.to_becquerels() / 1e18,
        chernobyl.to_curies() / 1e6
    );

    println!();
}

/// Radiation dose: how much your body absorbs
fn radiation_dose() {
    println!("--- Radiation Dose ---\n");

    // Chest X-ray: ~0.02 mSv
    let xray = Dose::sieverts(0.00002);
    println!(
        "Chest X-ray: {:.3} mSv = {:.1} mrem",
        xray.to_sieverts() * 1000.0,
        xray.to_rems() * 1000.0
    );

    // Coast-to-coast flight: ~0.04 mSv (cosmic radiation)
    let flight = Dose::sieverts(0.00004);
    println!(
        "NYC→LA flight: {:.3} mSv = {:.1} mrem (cosmic rays at altitude)",
        flight.to_sieverts() * 1000.0,
        flight.to_rems() * 1000.0
    );

    // CT scan: ~7 mSv
    let ct = Dose::sieverts(0.007);
    println!(
        "CT scan: {:.0} mSv = {:.0} mrem",
        ct.to_sieverts() * 1000.0,
        ct.to_rems() * 1000.0
    );

    // Average annual background radiation: ~2.4 mSv
    let annual = Dose::sieverts(0.0024);
    println!(
        "Annual background: {:.1} mSv = {:.0} mrem",
        annual.to_sieverts() * 1000.0,
        annual.to_rems() * 1000.0
    );

    // How many chest X-rays = one CT scan?
    let ratio = ct.to_sieverts() / xray.to_sieverts();
    println!("1 CT scan = {:.0} chest X-rays worth of radiation", ratio);

    println!();
}

/// Solar irradiance: the power of sunshine
fn solar_irradiance() {
    println!("--- Solar Irradiance ---\n");

    // Solar constant: ~1361 W/m² (at the edge of Earth's atmosphere)
    let solar_constant = Irradiance::watts_per_square_meter(1361.0);
    println!(
        "Solar constant: {:.0} W/m² (power hitting the top of our atmosphere)",
        solar_constant.to_watts_per_square_meter()
    );

    // On a clear day at Earth's surface: ~1000 W/m²
    let clear_day = Irradiance::watts_per_square_meter(1000.0);
    println!(
        "Sunny day (ground level): {:.0} W/m²",
        clear_day.to_watts_per_square_meter()
    );

    // Solar panel area needed for 5 kW system (at 20% efficiency)
    let panel_efficiency = 0.20;
    let desired_power = 5000.0; // watts
    let area_needed = desired_power / (clear_day.to_watts_per_square_meter() * panel_efficiency);
    println!(
        "5 kW solar system: needs {:.1} m² of panels at 20% efficiency",
        area_needed
    );

    println!();
}

/// Light levels we see every day
fn everyday_light() {
    println!("--- Everyday Light ---\n");

    // Sunlight outdoors: ~100,000 lux
    let sunlight = Illuminance::lux(100_000.0);
    println!(
        "Direct sunlight: {:.0} lux",
        sunlight.to_lux()
    );

    // Overcast day: ~1,000 lux
    let overcast = Illuminance::lux(1_000.0);
    println!(
        "Overcast day: {:.0} lux",
        overcast.to_lux()
    );

    // Office lighting: ~500 lux
    let office = Illuminance::lux(500.0);
    println!(
        "Office: {:.0} lux (enough for reading)",
        office.to_lux()
    );

    // Candlelight: ~10 lux
    let candle = Illuminance::lux(10.0);
    println!(
        "Candlelight dinner: {:.0} lux (romantic, not practical)",
        candle.to_lux()
    );

    // Full moon: ~0.25 lux
    let moonlight = Illuminance::lux(0.25);
    println!(
        "Full moon: {:.2} lux",
        moonlight.to_lux()
    );

    // Ratio: sunlight is how many times brighter than moonlight?
    let ratio = sunlight.to_lux() / moonlight.to_lux();
    println!(
        "Sunlight is {:.0}x brighter than moonlight", ratio
    );

    // Screen brightness: a phone at max ~700 cd/m²
    let phone_screen = Luminance::candelas_per_square_meter(700.0);
    let laptop = Luminance::candelas_per_square_meter(300.0);
    println!(
        "\nPhone screen: {:.0} cd/m² (nits), Laptop: {:.0} cd/m²",
        phone_screen.to_candelas_per_square_meter(),
        laptop.to_candelas_per_square_meter()
    );

    // A 60W-equivalent LED bulb: ~800 lumens
    let led_bulb = LuminousFlux::lumens(800.0);
    println!(
        "60W-equivalent LED: {:.0} lumens",
        led_bulb.to_lumens()
    );

    // Candela: a regular candle is ~1 cd
    let candle_intensity = LuminousIntensity::candelas(1.0);
    println!(
        "One candle: {:.0} cd (that's literally where the unit name comes from)",
        candle_intensity.to_candelas()
    );

    println!("\nAll radiation & light examples done!");
}
