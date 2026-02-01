//! Physics calculations — rockets, sports cars, and skydiving.
//!
//! Run with: `cargo run --example physics_calculations`

use rquants::prelude::*;

fn main() {
    println!("==========================================");
    println!("  RQuants Physics — Real-World Edition");
    println!("==========================================\n");

    kinematics();
    dynamics();
    energy_calculations();
    electromagnetism();
    pressure_scenarios();
}

/// Kinematics: how things move
fn kinematics() {
    println!("--- Kinematics ---\n");

    // Tesla Model S Plaid: 0-60 mph in ~2.0 seconds
    let v0 = Velocity::miles_per_hour(0.0);
    let v1 = Velocity::miles_per_hour(60.0);
    let t = Time::seconds(2.0);
    let accel = (v1 - v0) / t;
    println!(
        "Tesla Plaid 0-60 mph in 2s: {:.2} m/s² ({:.2} g's!)",
        accel.to_meters_per_second_squared(),
        accel.to_earth_gravities()
    );

    // SR-71 Blackbird cruising at Mach 3.2
    let sr71 = Velocity::meters_per_second(343.0 * 3.2);
    let flight_time = Time::hours(1.0);
    let distance = sr71 * flight_time;
    println!(
        "SR-71 in 1 hour: {:.0} km = {:.0} mi (Mach 3.2)",
        distance.to_kilometers(),
        distance.to_miles()
    );

    // ISS orbital speed: ~27,600 km/h
    let iss_speed = Velocity::kilometers_per_hour(27_600.0);
    let orbit_circumference = Length::kilometers(42_600.0);
    let orbit_time = Time::hours(orbit_circumference.to_kilometers() / iss_speed.to_kilometers_per_hour());
    println!(
        "ISS orbits Earth: {:.0} km at {:.0} km/h = {:.0} min per orbit",
        orbit_circumference.to_kilometers(),
        iss_speed.to_kilometers_per_hour(),
        orbit_time.to_minutes()
    );

    // Peregrine falcon dive: 390 km/h
    let falcon = Velocity::kilometers_per_hour(390.0);
    println!(
        "Peregrine falcon dive: {:.0} km/h = {:.0} mph (fastest animal alive)",
        falcon.to_kilometers_per_hour(),
        falcon.to_miles_per_hour()
    );

    println!();
}

/// Dynamics: forces, momentum, and Newton's laws
fn dynamics() {
    println!("--- Dynamics ---\n");

    // F = ma: Force to launch a SpaceX Falcon 9 (mass ~549,000 kg at liftoff)
    let rocket_mass = Mass::kilograms(549_000.0);
    let thrust_accel = Acceleration::meters_per_second_squared(12.0); // ~1.2g net
    let force = rocket_mass * thrust_accel;
    println!(
        "Falcon 9 thrust: {:.0} kg x {:.1} m/s² = {:.0} N = {:.0} kN",
        rocket_mass.to_kilograms(),
        thrust_accel.to_meters_per_second_squared(),
        force.to_newtons(),
        force.to_newtons() / 1000.0
    );

    // Momentum of a bowling ball vs a baseball
    let bowling_ball = Mass::kilograms(6.35);
    let bowling_speed = Velocity::meters_per_second(8.0);
    let bowling_p = bowling_ball * bowling_speed;

    let baseball = Mass::grams(145.0);
    let pitch_speed = Velocity::miles_per_hour(100.0); // fastball
    let baseball_p = baseball * pitch_speed;

    println!(
        "Bowling ball momentum: {:.1} kg·m/s",
        bowling_p.to_kilogram_meters_per_second()
    );
    println!(
        "100 mph fastball momentum: {:.1} kg·m/s",
        baseball_p.to_kilogram_meters_per_second()
    );

    // Weight of the heaviest deadlift ever (501 kg by Hafthor Bjornsson)
    let deadlift = Mass::kilograms(501.0);
    let g = Acceleration::meters_per_second_squared(9.80665);
    let weight = deadlift * g;
    println!(
        "World record deadlift: {} = {:.0} N = {:.0} lbf",
        deadlift,
        weight.to_newtons(),
        weight.to_pound_force()
    );

    println!();
}

/// Energy and power in the real world
fn energy_calculations() {
    println!("--- Energy & Power ---\n");

    // Tesla Powerwall: 13.5 kWh battery, 5 kW continuous output
    let battery = Energy::kilowatt_hours(13.5);
    let output = Power::kilowatts(5.0);
    let runtime = battery / output;
    println!(
        "Tesla Powerwall: {:.1} kWh / {:.0} kW = {:.1} hours of backup power",
        battery.to_kilowatt_hours(),
        output.to_kilowatts(),
        runtime.to_hours()
    );

    // Kinetic energy: Bugatti Chiron at top speed (490 km/h, 1995 kg)
    let chiron_mass = Mass::kilograms(1995.0);
    let chiron_speed = Velocity::kilometers_per_hour(490.0);
    let ke = Energy::kinetic(chiron_mass, chiron_speed);
    println!(
        "Bugatti Chiron at {:.0} km/h: KE = {:.0} kJ = {:.2} kWh",
        chiron_speed.to_kilometers_per_hour(),
        ke.to_joules() / 1000.0,
        ke.to_kilowatt_hours()
    );

    // Power: gaming PC under full load (~500W for 6 hours)
    let gaming_power = Power::watts(500.0);
    let session = Time::hours(6.0);
    let energy_used = gaming_power * session;
    println!(
        "Gaming marathon (6h): {:.0} W x {} = {:.1} kWh",
        gaming_power.to_watts(),
        session,
        energy_used.to_kilowatt_hours()
    );

    // Calories: a Big Mac is about 550 kcal
    let big_mac = Energy::kilocalories(550.0);
    println!(
        "Big Mac: {:.0} kcal = {:.0} kJ = {:.2} kWh (enough to power a lightbulb for {:.1}h)",
        big_mac.to_kilocalories(),
        big_mac.to_joules() / 1000.0,
        big_mac.to_kilowatt_hours(),
        (big_mac / Power::watts(60.0)).to_hours()
    );

    // 1.21 gigawatts! (Back to the Future)
    let flux_capacitor = Power::gigawatts(1.21);
    println!(
        "Flux capacitor: {:.2} GW = {:.0} MW = {:.0} hp (Great Scott!)",
        flux_capacitor.to_gigawatts(),
        flux_capacitor.to_megawatts(),
        flux_capacitor.to_horsepower()
    );

    println!();
}

/// Electricity: Ohm's law and everyday circuits
fn electromagnetism() {
    println!("--- Electromagnetism ---\n");

    // Phone charger: 5V, 2A (USB)
    let voltage = ElectricPotential::volts(5.0);
    let current = ElectricCurrent::amperes(2.0);
    let power = voltage * current;
    println!(
        "USB charger: {} x {} = {:.0} W",
        voltage, current,
        power.to_watts()
    );

    // Ohm's law: LED with 220 ohm resistor on 5V
    let v = ElectricPotential::volts(5.0);
    let r = ElectricalResistance::ohms(220.0);
    let i = v / r;
    println!(
        "LED circuit: {} / {} = {:.1} mA",
        v, r,
        i.to_amperes() * 1000.0
    );

    // iPhone battery: ~3349 mAh at 3.83V
    let charge = ElectricCharge::milliamperehours(3349.0);
    let batt_voltage = ElectricPotential::volts(3.83);
    let energy_wh = charge.to_amperehours() * batt_voltage.to_volts();
    println!(
        "iPhone battery: {:.0} mAh at {:.2} V = {:.1} Wh",
        charge.to_milliamperehours(),
        batt_voltage.to_volts(),
        energy_wh
    );

    // Capacitor: camera flash capacitor ~200 µF at 300V
    let cap = Capacitance::microfarads(200.0);
    let cap_v = ElectricPotential::volts(300.0);
    let stored = 0.5 * cap.to_farads() * cap_v.to_volts() * cap_v.to_volts();
    println!(
        "Camera flash capacitor: {:.0} µF at {:.0} V = {:.1} J (zap!)",
        cap.to_microfarads(),
        cap_v.to_volts(),
        stored
    );

    println!();
}

/// Pressure: from tires to the deep ocean
fn pressure_scenarios() {
    println!("--- Pressure ---\n");

    // Car tire pressure (~32 psi)
    let tire = Pressure::psi(32.0);
    println!(
        "Car tire: {:.0} psi = {:.0} kPa = {:.1} bar",
        tire.to_psi(),
        tire.to_pascals() / 1000.0,
        tire.to_bars()
    );

    // Mariana Trench: ~1086 bar at the bottom
    let mariana = Pressure::bars(1086.0);
    println!(
        "Mariana Trench: {:.0} bar = {:.0} atm = {:.0} psi (crushes submarines)",
        mariana.to_bars(),
        mariana.to_atmospheres(),
        mariana.to_psi()
    );

    // Blood pressure: 120/80 mmHg
    let systolic = Pressure::millimeters_of_mercury(120.0);
    let diastolic = Pressure::millimeters_of_mercury(80.0);
    println!(
        "Blood pressure: {:.0}/{:.0} mmHg = {:.1}/{:.1} kPa",
        systolic.to_millimeters_of_mercury(),
        diastolic.to_millimeters_of_mercury(),
        systolic.to_pascals() / 1000.0,
        diastolic.to_pascals() / 1000.0
    );

    println!("\nAll physics examples done!");
}
