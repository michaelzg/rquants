//! Physics calculation examples demonstrating cross-quantity operations.

use rquants::prelude::*;

fn main() {
    println!("=== Kinematics ===");
    kinematics();

    println!("\n=== Dynamics ===");
    dynamics();

    println!("\n=== Energy ===");
    energy_calculations();

    println!("\n=== Electromagnetism ===");
    electromagnetism();

    println!("\n=== Dimensional Chain ===");
    dimensional_chain();
}

fn kinematics() {
    // v = d / t
    let distance = Length::kilometers(100.0);
    let time = Time::hours(1.5);
    let velocity = distance / time;
    println!(
        "Average speed: {:.1} km/h = {:.1} mph",
        velocity.to_kilometers_per_hour(),
        velocity.to_miles_per_hour()
    );

    // d = v * t (distance traveled)
    let cruising_speed = Velocity::kilometers_per_hour(900.0);
    let flight_time = Time::hours(5.0);
    let flight_distance = cruising_speed * flight_time;
    println!(
        "Flight distance: {:.0} km = {:.0} miles",
        flight_distance.to_kilometers(),
        flight_distance.to_miles()
    );

    // a = (v - v0) / t
    let v0 = Velocity::meters_per_second(0.0);
    let v1 = Velocity::kilometers_per_hour(100.0);
    let accel_time = Time::seconds(8.0);
    let acceleration = (v1 - v0) / accel_time;
    println!(
        "0-100 km/h in 8s: {:.2} m/s² ({:.2} g)",
        acceleration.to_meters_per_second_squared(),
        acceleration.to_earth_gravities()
    );
}

fn dynamics() {
    // F = ma
    let mass = Mass::kilograms(1000.0); // Car
    let acceleration = Acceleration::meters_per_second_squared(3.0);
    let force = mass * acceleration;
    println!(
        "Force to accelerate {}: {:.0} N = {:.1} kgf",
        mass,
        force.to_newtons(),
        force.to_kilogram_force()
    );

    // p = mv (momentum)
    let velocity = Velocity::meters_per_second(30.0);
    let momentum = mass * velocity;
    println!(
        "Momentum: {} × {} = {:.0} kg·m/s",
        mass,
        velocity,
        momentum.to_kilogram_meters_per_second()
    );

    // Pressure: P = F/A
    let force = Force::newtons(500.0);
    let area = Area::square_meters(0.01); // 100 cm²
    let pressure = force / area;
    println!(
        "Pressure: {} / {} = {:.0} Pa = {:.2} atm",
        force,
        area,
        pressure.to_pascals(),
        pressure.to_atmospheres()
    );
}

fn energy_calculations() {
    // Work = Force × Distance
    let force = Force::newtons(100.0);
    let distance = Length::meters(50.0);
    let work = force.to_newtons() * distance.to_meters();
    let energy = Energy::joules(work);
    println!(
        "Work done: {} × {} = {:.0} J = {:.4} kWh",
        force,
        distance,
        energy.to_joules(),
        energy.to_kilowatt_hours()
    );

    // Power = Energy / Time
    let energy = Energy::kilowatt_hours(10.0);
    let time = Time::hours(4.0);
    let power = energy / time;
    println!(
        "Average power: {:.0} kWh / {} = {:.1} kW",
        energy.to_kilowatt_hours(),
        time,
        power.to_kilowatts()
    );

    // Kinetic energy: KE = 0.5 * m * v²
    let mass = Mass::kilograms(1500.0);
    let speed = Velocity::kilometers_per_hour(120.0);
    let ke = Energy::kinetic(mass, speed);
    println!(
        "Kinetic energy at {}: {:.0} J = {:.3} kWh",
        speed,
        ke.to_joules(),
        ke.to_kilowatt_hours()
    );

    // Specific energy (energy per mass)
    let battery_energy = Energy::kilowatt_hours(75.0); // Tesla battery
    let battery_mass = Mass::kilograms(480.0);
    let specific = battery_energy / battery_mass;
    println!(
        "Battery specific energy: {:.0} J/kg ({:.0} Wh/kg)",
        specific.to_grays(),
        specific.to_grays() / 3.6 // Convert J/kg to Wh/kg
    );
}

fn electromagnetism() {
    // Ohm's law: V = IR
    let voltage = ElectricPotential::volts(230.0);
    let resistance = ElectricalResistance::ohms(100.0);
    let current = voltage / resistance;
    println!(
        "Ohm's law: {} / {} = {:.2} A",
        voltage,
        resistance,
        current.to_amperes()
    );

    // Power: P = VI
    let power = voltage * current;
    println!(
        "Power: {} × {:.2} A = {:.1} W",
        voltage,
        current.to_amperes(),
        power.to_watts()
    );

    // Charge: Q = It
    let current = ElectricCurrent::amperes(2.0);
    let time = Time::hours(1.0);
    let charge = current * time;
    println!(
        "Charge: {} × {} = {:.0} C = {:.1} Ah",
        current,
        time,
        charge.to_coulombs(),
        charge.to_amperehours()
    );

    // Capacitance: C = Q/V
    let charge = ElectricCharge::coulombs(0.001);
    let voltage = ElectricPotential::volts(5.0);
    let capacitance = charge / voltage;
    println!(
        "Capacitance: {} / {} = {:.0} µF",
        charge,
        voltage,
        capacitance.to_microfarads()
    );
}

fn dimensional_chain() {
    // Demonstrate a chain of dimensional operations
    println!("Position → Velocity → Acceleration chain:");

    let pos1 = Length::meters(0.0);
    let pos2 = Length::meters(100.0);
    let t1 = Time::seconds(0.0);
    let t2 = Time::seconds(10.0);

    let displacement = pos2 - pos1;
    let duration = t2 - t1;
    let avg_velocity = displacement / duration;
    println!("  Displacement: {}", displacement);
    println!("  Avg velocity: {:.1} m/s", avg_velocity.to_meters_per_second());

    let v1 = Velocity::meters_per_second(0.0);
    let v2 = avg_velocity;
    let accel = (v2 - v1) / duration;
    println!("  Acceleration: {:.1} m/s²", accel.to_meters_per_second_squared());

    let force = Mass::kilograms(50.0) * accel;
    println!("  Force on 50 kg: {:.1} N", force.to_newtons());

    let work_energy = force.to_newtons() * displacement.to_meters();
    let energy = Energy::joules(work_energy);
    let power = energy / duration;
    println!("  Energy: {:.0} J", energy.to_joules());
    println!("  Power: {:.1} W", power.to_watts());
}
