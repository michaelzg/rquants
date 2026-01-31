//! Basic usage examples for RQuants.

use rquants::prelude::*;

fn main() {
    // === Creating quantities ===
    let distance = Length::meters(100.0);
    let time = Time::seconds(9.58);
    let speed = distance / time;
    println!(
        "Usain Bolt: {} / {} = {:.2} m/s ({:.2} km/h)",
        distance,
        time,
        speed.to_meters_per_second(),
        speed.to_kilometers_per_hour()
    );

    // === DSL syntax ===
    let height = 10.0.meters();
    let width = 5.0.meters();
    let area = height * width;
    println!("Room area: {} × {} = {}", height, width, area);

    // === Unit conversions ===
    let marathon = Length::kilometers(42.195);
    println!(
        "Marathon: {:.3} km = {:.2} miles = {:.0} ft",
        marathon.to_kilometers(),
        marathon.to_miles(),
        marathon.to_feet()
    );

    // === Newton's second law: F = ma ===
    let mass = Mass::kilograms(75.0);
    let gravity = Acceleration::meters_per_second_squared(9.80665);
    let weight = mass * gravity;
    println!(
        "Weight of {} person: {:.1} N ({:.1} lbf)",
        mass,
        weight.to_newtons(),
        weight.to_pound_force()
    );

    // === Energy and Power ===
    let power = Power::kilowatts(1.5);
    let duration = Time::hours(3.0);
    let energy = power * duration;
    println!(
        "Energy used: {} × {} = {:.1} kWh = {:.0} J",
        power,
        duration,
        energy.to_kilowatt_hours(),
        energy.to_joules()
    );

    // === Pressure ===
    let atm = Pressure::atmospheres(1.0);
    println!(
        "1 atm = {:.0} Pa = {:.2} psi = {:.0} mmHg",
        atm.to_pascals(),
        atm.to_psi(),
        atm.to_millimeters_of_mercury()
    );

    // === Temperature ===
    let boiling = Temperature::celsius(100.0);
    println!(
        "Boiling point: {}°C = {}°F = {} K",
        boiling.to_celsius_scale(),
        boiling.to_fahrenheit_scale(),
        boiling.to_kelvin_scale()
    );

    // Mixed-scale arithmetic
    let room_temp = Temperature::fahrenheit(72.0);
    let increase = Temperature::celsius(5.0);
    let new_temp = room_temp + increase;
    println!(
        "Room at {}°F + {}°C increase = {:.1}°F",
        room_temp.value(),
        increase.value(),
        new_temp.value()
    );

    // === Ohm's Law: V = IR ===
    let voltage = ElectricPotential::volts(12.0);
    let resistance = ElectricalResistance::ohms(4.0);
    let current = voltage / resistance;
    let power = voltage * current;
    println!(
        "Circuit: {} / {} = {} A, power = {} W",
        voltage,
        resistance,
        current.to_amperes(),
        power.to_watts()
    );

    // === Information ===
    let file_size = Information::gigabytes(4.7); // DVD
    let speed = DataRate::megabytes_per_second(16.0); // USB 2.0
    let transfer_time = file_size / speed;
    println!(
        "Transfer {:.1} GB at {} MB/s: {:.0} seconds",
        file_size.to_gigabytes(),
        speed.to_megabytes_per_second(),
        transfer_time.to_seconds()
    );

    // === Quantity comparison ===
    let a = Length::meters(100.0);
    let b = Length::kilometers(0.1);
    println!(
        "{} == {}: {}",
        a,
        b,
        a == b // true: both are 100 meters
    );

    println!("\nAll examples completed successfully!");
}
