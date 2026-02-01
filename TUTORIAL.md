# RQuants Tutorial: Scala squants to Rust

A hands-on guide for Scala developers migrating from squants to rquants. Assumes no prior Rust experience.

---

## Table of Contents

1. [Setting Up Rust](#1-setting-up-rust)
2. [Your First Rust Program](#2-your-first-rust-program)
3. [Rust Concepts You Need (and Only These)](#3-rust-concepts-you-need-and-only-these)
4. [Side-by-Side: squants vs rquants](#4-side-by-side-squants-vs-rquants)
5. [Creating Quantities](#5-creating-quantities)
6. [Unit Conversions](#6-unit-conversions)
7. [Arithmetic and Cross-Quantity Operations](#7-arithmetic-and-cross-quantity-operations)
8. [The DSL: Making Numbers Into Quantities](#8-the-dsl-making-numbers-into-quantities)
9. [Temperature (The Special One)](#9-temperature-the-special-one)
10. [Money and Currency](#10-money-and-currency)
11. [Comparing Quantities](#11-comparing-quantities)
12. [Common Patterns and Gotchas](#12-common-patterns-and-gotchas)
13. [Cheat Sheet: Scala to Rust Translation](#13-cheat-sheet-scala-to-rust-translation)

---

## 1. Setting Up Rust

Install Rust (takes about 60 seconds):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts (accept defaults). Then restart your terminal and verify:

```bash
rustc --version   # Should print something like: rustc 1.XX.0
cargo --version   # Cargo is Rust's build tool (like sbt for Scala)
```

**Key analogy**: `cargo` is to Rust what `sbt` is to Scala. It compiles, tests, manages dependencies, and runs your code.

| Scala (sbt) | Rust (cargo) |
|---|---|
| `sbt compile` | `cargo build` |
| `sbt test` | `cargo test` |
| `sbt run` | `cargo run` |
| `build.sbt` | `Cargo.toml` |
| `src/main/scala/` | `src/` |

---

## 2. Your First Rust Program

Create a new project that uses rquants:

```bash
cargo new my_project
cd my_project
```

Edit `Cargo.toml` (this is your `build.sbt`):

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
rquants = { path = "../rquants" }  # or from crates.io when published
```

Edit `src/main.rs`:

```rust
use rquants::prelude::*;

fn main() {
    let distance = Length::kilometers(42.195);
    let time = Time::hours(2.0) + Time::minutes(1.0) + Time::seconds(9.0);
    let pace = distance / time;

    println!("Marathon pace: {:.2} km/h", pace.to_kilometers_per_hour());
    println!("That's {:.2} mph", pace.to_miles_per_hour());
}
```

Run it:

```bash
cargo run
```

You should see output like:

```
Marathon pace: 20.86 km/h
That's 12.96 mph
```

---

## 3. Rust Concepts You Need (and Only These)

You don't need to learn all of Rust to use rquants. Here are the six concepts that matter:

### 3.1 Variables are Immutable by Default

```rust
let x = 5;        // immutable (like Scala's val)
let mut y = 5;    // mutable (like Scala's var)
y = 10;           // OK
// x = 10;        // ERROR: cannot assign twice to immutable variable
```

**Scala equivalent**: `val` vs `var`. In Rust, `let` is `val` and `let mut` is `var`.

### 3.2 Types Are Inferred (Usually)

```rust
let distance = Length::meters(100.0);  // Rust infers: distance is Length
let time = Time::seconds(9.58);       // Rust infers: time is Time
let speed = distance / time;           // Rust infers: speed is Velocity
```

Just like Scala, Rust's compiler figures out types. You rarely need to write them explicitly.

### 3.3 No Nulls -- Use Option and Result

```rust
// Scala: val x: Option[Int] = Some(5)
// Rust:
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

// Scala: x.getOrElse(0)
// Rust:
let value = x.unwrap_or(0);
```

Where Scala uses `Option[T]`, Rust uses `Option<T>`. Where Scala uses `Try[T]` or `Either[E, T]`, Rust uses `Result<T, E>`.

```rust
// Handling a Result (like Scala's Try)
let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.85);
match rate.convert(Money::usd(100.0)) {
    Ok(euros) => println!("Got: {}", euros),
    Err(e) => println!("Error: {}", e),
}

// Or if you're confident it won't fail:
let euros = rate.convert(Money::usd(100.0)).unwrap();
```

### 3.4 Structs and Enums (Not Classes and Case Classes)

```scala
// Scala
case class Length(value: Double, unit: LengthUnit)

sealed trait LengthUnit
case object Meters extends LengthUnit
case object Kilometers extends LengthUnit
```

```rust
// Rust equivalent
struct Length {
    value: f64,
    unit: LengthUnit,
}

enum LengthUnit {
    Meters,
    Kilometers,
}
```

**Key difference**: Rust has no class inheritance. Instead, it uses **traits** (similar to Scala traits but without implementation inheritance).

### 3.5 Traits (Like Scala Traits, But Simpler)

```scala
// Scala
trait Quantity[A] {
  def value: Double
  def unit: UnitOfMeasure[A]
  def to(target: UnitOfMeasure[A]): Double
}
```

```rust
// Rust
trait Quantity {
    type Unit: UnitOfMeasure;
    fn value(&self) -> f64;
    fn unit(&self) -> Self::Unit;
    fn to(&self, target: Self::Unit) -> f64;
}
```

The `&self` parameter is how Rust says "this is a method on an instance" (like Scala's implicit `this`). `Self::Unit` is an **associated type** -- it's how Rust says "each implementor chooses its own Unit type."

### 3.6 Use Statements (Like Scala Imports)

```scala
// Scala
import squants.space._
import squants.space.LengthConversions._
```

```rust
// Rust
use rquants::prelude::*;  // imports everything you need
```

The `*` glob import works like Scala's `_` wildcard import. The `prelude` module re-exports all the types and traits you'll commonly need.

---

## 4. Side-by-Side: squants vs rquants

Here's the same program in both languages:

### Scala (squants)

```scala
import squants.space.LengthConversions._
import squants.time.TimeConversions._
import squants.motion.VelocityConversions._
import squants.mass.MassConversions._

val distance = 100.meters
val time = 9.58.seconds
val speed = distance / time

println(s"Speed: ${speed.toMetersPerSecond} m/s")
println(s"Speed: ${speed.toKilometersPerHour} km/h")

val mass = 10.kilograms
val gravity = MetersPerSecondSquared(9.8)
val force = mass * gravity

println(s"Force: ${force.toNewtons} N")
```

### Rust (rquants)

```rust
use rquants::prelude::*;

fn main() {
    let distance = 100.0.meters();
    let time = 9.58.seconds();
    let speed = distance / time;

    println!("Speed: {} m/s", speed.to_meters_per_second());
    println!("Speed: {} km/h", speed.to_kilometers_per_hour());

    let mass = 10.0.kilograms();
    let gravity = Acceleration::meters_per_second_squared(9.8);
    let force = mass * gravity;

    println!("Force: {} N", force.to_newtons());
}
```

**Differences to notice**:

1. **`fn main() { }`** -- Rust programs start from a `main` function (Scala uses `object Main extends App`)
2. **`100.0` not `100`** -- The DSL works on `f64` (Rust's `Double`), so you write `100.0` not `100`
3. **`.meters()` not `.meters`** -- Rust requires parentheses on method calls (no parameterless method shorthand)
4. **`println!` with `{}`** -- Rust uses `{}` placeholders instead of `${}` interpolation. The `!` means it's a macro (don't worry about this, just include the `!`)
5. **`to_meters_per_second()` not `toMetersPerSecond`** -- Rust convention is `snake_case`, not `camelCase`
6. **Semicolons** -- Rust requires `;` at the end of statements

---

## 5. Creating Quantities

There are two ways to create quantities:

### Named Constructors (Most Common)

```rust
use rquants::prelude::*;

// Type::unit_name(value)
let length = Length::meters(100.0);
let length = Length::kilometers(42.195);
let length = Length::miles(26.2);
let length = Length::feet(5280.0);

let time = Time::seconds(60.0);
let time = Time::hours(2.5);
let time = Time::milliseconds(500.0);

let mass = Mass::kilograms(75.0);
let mass = Mass::pounds(165.0);

let energy = Energy::kilowatt_hours(10.0);
let energy = Energy::joules(3600.0);

let pressure = Pressure::atmospheres(1.0);
let pressure = Pressure::pascals(101325.0);
```

### Generic Constructor

```rust
// Type::new(value, UnitEnum::Variant)
let length = Length::new(100.0, LengthUnit::Meters);
let time = Time::new(60.0, TimeUnit::Seconds);
```

This is useful when the unit comes from a variable or configuration.

---

## 6. Unit Conversions

### Getting a Numeric Value in a Specific Unit

```rust
let marathon = Length::kilometers(42.195);

// to_<unit_name>() returns f64
let miles: f64 = marathon.to_miles();          // 26.2188...
let meters: f64 = marathon.to_meters();        // 42195.0
let feet: f64 = marathon.to_feet();            // 138435.0...
```

**Scala equivalent**: `marathon.toMiles`, `marathon.toMeters`, `marathon.toFeet`

### Getting a Quantity in a Different Unit

```rust
let marathon = Length::kilometers(42.195);

// in_unit() returns a new Length with the value re-expressed
let in_miles = marathon.in_unit(LengthUnit::Miles);
println!("{}", in_miles);  // "26.2188... mi"
```

**Scala equivalent**: `marathon.in(Miles)`

### Using the Quantity Trait's `to` Method

```rust
// .to(UnitEnum) returns f64 -- same as the named to_xxx() methods
let miles = marathon.to(LengthUnit::Miles);
```

---

## 7. Arithmetic and Cross-Quantity Operations

### Same-Type Arithmetic

```rust
let a = Length::meters(100.0);
let b = Length::meters(50.0);

let sum = a + b;          // 150 m
let diff = a - b;         // 50 m
let scaled = a * 2.0;     // 200 m
let halved = a / 2.0;     // 50 m
let ratio = a / b;        // 2.0 (f64)
let negated = -a;         // -100 m
```

This is identical to Scala squants.

### Cross-Quantity Operations (Dimensional Analysis)

This is where rquants shines -- the compiler enforces dimensional correctness:

```rust
// Length / Time = Velocity
let distance = Length::meters(100.0);
let time = Time::seconds(10.0);
let velocity: Velocity = distance / time;  // type annotation optional

// Velocity * Time = Length
let new_distance: Length = velocity * time;

// Mass * Acceleration = Force
let mass = Mass::kilograms(10.0);
let accel = Acceleration::meters_per_second_squared(9.8);
let force: Force = mass * accel;

// Force / Area = Pressure
let area = Area::square_meters(2.0);
let pressure: Pressure = force / area;

// Voltage / Resistance = Current
let v = ElectricPotential::volts(12.0);
let r = ElectricalResistance::ohms(4.0);
let current: ElectricCurrent = v / r;

// Voltage * Current = Power
let power: Power = v * current;

// Power * Time = Energy
let duration = Time::hours(1.0);
let energy: Energy = power * duration;
```

If you try an invalid combination, the compiler will stop you:

```rust
// This does NOT compile:
// let nonsense = Length::meters(5.0) + Mass::kilograms(3.0);
// Error: no implementation for `Length + Mass`
```

---

## 8. The DSL: Making Numbers Into Quantities

Import the prelude to get extension methods on `f64`:

```rust
use rquants::prelude::*;

// These all work:
let d = 100.0.meters();
let t = 5.0.seconds();
let m = 75.0.kilograms();
let v = 30.0.meters_per_second();
let e = 10.0.kilowatt_hours();
let p = 1.0.atmospheres();
let temp = 72.0.fahrenheit();
let money = 100.0.usd();
```

**Scala comparison**:

| Scala | Rust |
|-------|------|
| `100.meters` | `100.0.meters()` |
| `5.seconds` | `5.0.seconds()` |
| `75.kilograms` | `75.0.kilograms()` |
| `30.metersPerSecond` | `30.0.meters_per_second()` |
| `100.USD` | `100.0.usd()` |

Note the three differences: `100.0` (not `100`), parentheses `()`, and `snake_case`.

---

## 9. Temperature (The Special One)

Temperature is special because different scales have different zero points. RQuants handles this correctly:

### Scale Conversions (Thermometer Readings)

These account for the zero offset between scales:

```rust
use rquants::thermal::temperature::{Temperature, TemperatureScale};

let boiling = Temperature::celsius(100.0);
println!("{:.1}°F", boiling.to_fahrenheit_scale());  // 212.0°F
println!("{:.2} K", boiling.to_kelvin_scale());       // 373.15 K

let body_temp = Temperature::fahrenheit(98.6);
println!("{:.1}°C", body_temp.to_celsius_scale());    // 37.0°C
```

### Degree Conversions (Temperature Differences)

These convert magnitudes only, with no zero offset:

```rust
// A change of 5 degrees Celsius = a change of 9 degrees Fahrenheit
let delta = Temperature::celsius(5.0);
println!("{:.0}°F change", delta.to_fahrenheit_degrees());  // 9°F change

// A change of 1 Kelvin = a change of 1 Celsius
let one_k = Temperature::kelvin(1.0);
println!("{:.0}°C change", one_k.to_celsius_degrees());     // 1°C change
```

### Arithmetic (Right Operand Is Degrees)

When you add or subtract temperatures, the right operand is treated as a degree change, not a scale value:

```rust
// Room is 72°F, increase by 5°C worth of degrees
let room = Temperature::fahrenheit(72.0);
let increase = Temperature::celsius(5.0); // 5°C = 9°F as degrees
let warmer = room + increase;
println!("{:.0}°F", warmer.value());  // 81°F (72 + 9)
```

**Scala equivalent**: This is exactly how squants Temperature arithmetic works -- `plus` and `minus` treat the operand as degrees.

---

## 10. Money and Currency

### Creating Money

```rust
use rquants::prelude::*;

let price = Money::usd(29.99);
let euros = Money::eur(25.00);
let yen = Money::jpy(3000.0);

// Or with DSL:
let price = 29.99.usd();
let euros = 25.00.eur();
```

### Arithmetic (Same Currency Only)

```rust
let a = Money::usd(100.0);
let b = Money::usd(30.0);

let total = a + b;           // $130
let diff = a - b;            // $70
let doubled = a * 2.0;       // $200
let split = a / 4.0;         // $25
let ratio = a / b;           // 3.333... (f64)
```

Adding different currencies panics:

```rust
// PANICS at runtime:
// let _ = Money::usd(100.0) + Money::eur(50.0);
```

### Currency Exchange

```rust
// Define an exchange rate: 1 USD = 0.92 EUR
let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.92);

// Convert USD to EUR
let dollars = Money::usd(100.0);
let euros = rate.convert(dollars).unwrap();  // 92.00 EUR

// Convert EUR back to USD
let back = rate.convert(euros).unwrap();     // 100.00 USD
```

### Generic Pricing

`Price<Q>` works with any quantity type:

```rust
// Price per unit of energy
let electricity = Price::new(Money::usd(0.12), Energy::kilowatt_hours(1.0));
let bill = electricity * Energy::kilowatt_hours(900.0);
println!("Monthly bill: {}", bill);  // 108 USD

// Price per unit of mass
let gold = Price::new(Money::usd(65.0), Mass::grams(1.0));
let bar_value = gold * Mass::kilograms(1.0);
println!("1 kg gold bar: {}", bar_value);  // 65000 USD

// Price per unit of volume
let gas = Price::new(Money::usd(3.50), Volume::us_gallons(1.0));
let fill_cost = gas * Volume::us_gallons(15.0);
println!("Fill up: {}", fill_cost);  // 52.5 USD
```

---

## 11. Comparing Quantities

### Equality

Quantities in different units are compared by converting to a common base:

```rust
let a = Length::meters(1000.0);
let b = Length::kilometers(1.0);
assert_eq!(a, b);  // true: both are 1000 meters
```

### Ordering

```rust
let short = Length::meters(100.0);
let long = Length::kilometers(1.0);

assert!(short < long);
assert!(long > short);
```

### Approximate Equality

Floating-point arithmetic can produce tiny errors. Use `approx_eq`:

```rust
use rquants::core::quantity::ApproxEq;

let a = Length::meters(1.0);
let b = Length::meters(1.0000000001);
let tolerance = Length::meters(0.001);

assert!(a.approx_eq(&b, &tolerance));
```

### Quantity Ranges

```rust
use rquants::core::quantity_range::QuantityRange;

let range = QuantityRange::new(
    Length::meters(0.0),
    Length::meters(100.0),
);

assert!(range.contains(Length::meters(50.0)));
assert!(!range.contains(Length::meters(150.0)));
```

---

## 12. Common Patterns and Gotchas

### Gotcha 1: f64 Literals Need a Decimal Point

```rust
// Won't work with the DSL:
// let d = 100.meters();  // ERROR: i32 doesn't have .meters()

// Use f64:
let d = 100.0.meters();   // OK
// Or cast:
let d = (100 as f64).meters();
```

Some DSL traits are also implemented for `i32`, so `100.usd()` works for Money, but for consistency always use `f64` literals (`100.0`).

### Gotcha 2: Printing Quantities

```rust
let v = Velocity::kilometers_per_hour(100.0);

// Display trait gives you "value unit":
println!("{}", v);                          // "100 km/h"

// For formatted numbers, extract the value:
println!("{:.2} km/h", v.to_kilometers_per_hour());  // "100.00 km/h"
println!("{:.1} mph", v.to_miles_per_hour());         // "62.1 mph"
```

### Gotcha 3: Ownership -- Quantities Are Copy Types

Unlike many Rust types, quantities are `Copy` (like Scala value types). You can use them freely without worrying about ownership:

```rust
let distance = Length::meters(100.0);
let d1 = distance;  // copies, not moves
let d2 = distance;  // still valid!
let sum = d1 + d2;  // both still usable
```

This means you never need `&`, `clone()`, or borrow checker gymnastics with quantities.

### Gotcha 4: Temperature Is Not a Quantity

Unlike all other types, `Temperature` does not implement the `Quantity` trait. This is because temperature scales have non-linear conversions. You can't use `Temperature` where a generic `Quantity` is expected, but this rarely matters in practice.

### Gotcha 5: use rquants::prelude::*

If you get "method not found" errors, you probably forgot the prelude import. This single import brings in all types, units, and DSL traits:

```rust
use rquants::prelude::*;
```

Without it, you'd need to import each type and trait individually.

### Pattern: Extracting Value and Unit

```rust
let length = Length::kilometers(42.195);

// Get the raw value and unit
let (value, symbol) = length.to_tuple();
println!("{} {}", value, symbol);  // "42.195 km"

// Get value in a specific unit
let (value, symbol) = length.to_tuple_in(LengthUnit::Miles);
println!("{:.2} {}", value, symbol);  // "26.22 mi"
```

### Pattern: Working with Collections

```rust
let readings = vec![
    Length::meters(10.0),
    Length::meters(20.0),
    Length::meters(30.0),
];

// Sum (using fold since Rust doesn't have a built-in sum for custom types)
let total = readings.iter()
    .copied()
    .fold(Length::meters(0.0), |acc, x| acc + x);
println!("Total: {}", total);  // "60 m"

// Find max
let max = readings.iter()
    .copied()
    .max_by(|a, b| a.partial_cmp(b).unwrap());
println!("Max: {:?}", max);  // Some(30 m)
```

---

## 13. Cheat Sheet: Scala to Rust Translation

### Naming Conventions

| Scala | Rust |
|-------|------|
| `camelCase` | `snake_case` |
| `toMetersPerSecond` | `to_meters_per_second()` |
| `inKilometers` | `in_unit(LengthUnit::Kilometers)` |
| `Meters(100)` | `Length::meters(100.0)` |
| `100.meters` | `100.0.meters()` |

### Types

| Scala | Rust |
|-------|------|
| `Double` | `f64` |
| `Int` | `i32` |
| `String` | `String` (owned) or `&str` (borrowed) |
| `Option[T]` | `Option<T>` |
| `Try[T]` | `Result<T, E>` |
| `BigDecimal` | `f64` (or `rust_decimal::Decimal`) |

### Syntax

| Scala | Rust |
|-------|------|
| `val x = 5` | `let x = 5;` |
| `var x = 5` | `let mut x = 5;` |
| `def foo(x: Int): Int = x + 1` | `fn foo(x: i32) -> i32 { x + 1 }` |
| `s"Hello $name"` | `format!("Hello {}", name)` |
| `println(s"x = $x")` | `println!("x = {}", x);` |
| `x match { case ... }` | `match x { ... }` |
| `import foo._` | `use foo::*;` |
| `x.getOrElse(0)` | `x.unwrap_or(0)` |
| `x.get` | `x.unwrap()` |

### Operations

| Scala squants | Rust rquants |
|---|---|
| `val d = 100.meters` | `let d = 100.0.meters();` |
| `d.toKilometers` | `d.to_kilometers()` |
| `d.in(Kilometers)` | `d.in_unit(LengthUnit::Kilometers)` |
| `d1 + d2` | `d1 + d2` |
| `d * 2.0` | `d * 2.0` |
| `d / t` | `d / t` |
| `d =~ other` | `d.approx_eq(&other, &tolerance)` |
| `Meters(100)` | `Length::meters(100.0)` |
| `100.USD` | `100.0.usd()` |
| `rate.convert(m)` | `rate.convert(m).unwrap()` |

### Building and Running

| Task | Scala (sbt) | Rust (cargo) |
|------|-------------|-------------|
| Create project | `sbt new` | `cargo new` |
| Compile | `sbt compile` | `cargo build` |
| Run | `sbt run` | `cargo run` |
| Test | `sbt test` | `cargo test` |
| Add dependency | `libraryDependencies += ...` | `[dependencies]` in Cargo.toml |
| REPL | `sbt console` | `cargo install evcxr_repl && evcxr` |
| Docs | `sbt doc` | `cargo doc --open` |
| Lint | (scalastyle/scalafix) | `cargo clippy` |
| Format | (scalafmt) | `cargo fmt` |

---

## Next Steps

1. Browse the examples: `cargo run --example basic_usage`
2. Read the API docs: `cargo doc --open`
3. Look at the [README](README.md) for the full list of modules and design decisions
4. Explore the test files in each module for more usage examples -- Rust tests are in the same file as the code (look for `#[cfg(test)] mod tests { ... }` at the bottom of each `.rs` file)
