# RQuants

A Rust port of the Scala [squants](https://github.com/typelevel/squants) library for quantities, units of measure, and dimensional analysis.

RQuants provides type-safe dimensional analysis with 200+ units across 12 physical domains, an ergonomic DSL for creating quantities, and compile-time enforcement of dimensional correctness.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
rquants = "0.1.0"
```

```rust
use rquants::prelude::*;

// Create quantities with named constructors
let distance = Length::meters(100.0);
let time = Time::seconds(9.58);
let speed = distance / time; // Returns Velocity

// Or use the DSL syntax
let mass = 75.0.kilograms();
let gravity = 9.80665.meters_per_second_squared();
let weight = mass * gravity; // Returns Force

// Energy and power
let power = Power::kilowatts(1.5);
let duration = Time::hours(3.0);
let energy = power * duration; // Returns Energy
assert!((energy.to_kilowatt_hours() - 4.5).abs() < 1e-10);
```

## Features

- **Type-safe dimensional analysis** -- cross-quantity operations return correct types at compile time (`Length / Time` produces `Velocity`, not a generic number)
- **12 domain modules** -- time, space, mass, motion, energy, thermal, electro, information, radio, photo, market
- **200+ units** -- from angstroms to light-years, from picofarads to megawatt-hours
- **Ergonomic DSL** -- `100.0.meters()`, `5.0.seconds()`, `72.0.fahrenheit()`
- **Approximate equality** -- `approx_eq` for floating-point tolerance comparisons
- **Temperature** -- proper scale vs. degree conversions (Kelvin, Celsius, Fahrenheit, Rankine)
- **Financial** -- Money, Currency, exchange rates, and generic `Price<Q>` over any quantity

## Modules

| Module | Quantities |
|--------|-----------|
| `core` | `Quantity`, `UnitOfMeasure`, `Dimension` traits; `Dimensionless`, `QuantityRange`, `Ratio` |
| `systems` | Metric prefixes (kilo, mega, ...) and binary prefixes (kibi, mebi, ...) |
| `time` | `Time`, `Frequency` |
| `space` | `Length`, `Area`, `Volume`, `Angle`, `SolidAngle` |
| `mass` | `Mass`, `Density`, `AreaDensity`, `ChemicalAmount`, `MomentOfInertia` |
| `motion` | `Velocity`, `Acceleration`, `Force`, `Momentum`, `Pressure` |
| `energy` | `Energy`, `Power`, `PowerRamp`, `SpecificEnergy`, `EnergyDensity`, `PowerDensity`, `MolarEnergy` |
| `thermal` | `Temperature`, `ThermalCapacity` |
| `electro` | `ElectricCurrent`, `ElectricCharge`, `ElectricPotential`, `ElectricalResistance`, `ElectricalConductance`, `Capacitance`, `Inductance`, `MagneticFlux`, `MagneticFluxDensity`, `Resistivity`, `Conductivity` |
| `information` | `Information`, `DataRate` |
| `radio` | `Activity`, `Dose`, `Irradiance`, `Radiance`, `RadiantIntensity`, `SpectralPower`, `SpectralIrradiance`, `ParticleFlux` |
| `photo` | `LuminousIntensity`, `LuminousFlux`, `Illuminance`, `Luminance`, `LuminousEnergy`, `LuminousExposure` |
| `market` | `Money`, `Currency`, `CurrencyExchangeRate`, `Price<Q>` |

## Examples

Run any example with:

```bash
cargo run --example basic_usage
cargo run --example physics_calculations
cargo run --example financial_calculations
```

### Unit Conversions

```rust
use rquants::prelude::*;

let marathon = Length::kilometers(42.195);
println!("{:.2} miles", marathon.to_miles());       // 26.22 miles
println!("{:.0} feet", marathon.to_feet());         // 138435 feet

let pressure = Pressure::atmospheres(1.0);
println!("{:.0} Pa", pressure.to_pascals());        // 101325 Pa
println!("{:.2} psi", pressure.to_psi());           // 14.70 psi
```

### Cross-Quantity Operations

```rust
use rquants::prelude::*;

// Newton's second law: F = ma
let mass = Mass::kilograms(1000.0);
let accel = Acceleration::meters_per_second_squared(3.0);
let force = mass * accel; // Force

// Ohm's law: V = IR
let voltage = ElectricPotential::volts(230.0);
let resistance = ElectricalResistance::ohms(100.0);
let current = voltage / resistance; // ElectricCurrent
let power = voltage * current;      // Power
```

### Temperature

Temperature supports both scale conversions (thermometer readings with zero offsets) and degree conversions (magnitude-only):

```rust
use rquants::thermal::temperature::{Temperature, TemperatureScale};

let boiling = Temperature::celsius(100.0);
assert!((boiling.to_fahrenheit_scale() - 212.0).abs() < 1e-10);
assert!((boiling.to_kelvin_scale() - 373.15).abs() < 1e-10);

// Degree conversions (no zero offset)
let delta = Temperature::celsius(5.0);
assert!((delta.to_fahrenheit_degrees() - 9.0).abs() < 1e-10);

// Mixed-scale arithmetic: rhs treated as degrees
let t = Temperature::fahrenheit(100.0) - Temperature::celsius(5.0);
// 5 degrees C = 9 degrees F, so 100 - 9 = 91
assert!((t.value() - 91.0).abs() < 1e-10);
```

### Financial

```rust
use rquants::prelude::*;

// Basic money arithmetic
let price = Money::usd(29.99);
let total = price * 3.0;

// Currency exchange
let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.92);
let euros = rate.convert(Money::usd(100.0)).unwrap();

// Generic pricing: Price<Q> works with any quantity
let electricity = Price::new(Money::usd(0.12), Energy::kilowatt_hours(1.0));
let bill = electricity * Energy::kilowatt_hours(900.0);
assert!((bill.to_amount() - 108.0).abs() < 1e-10);
```

---

## Design Choices and Divergences from Scala squants

RQuants is a faithful port, but Rust's type system and idioms differ from Scala's. This section documents every deliberate design divergence.

### 1. Newtype Structs Instead of Abstract Classes

**Scala**: `abstract class Quantity[A <: Quantity[A]]` with self-type `self: A` (F-bounded polymorphism).

**Rust**: Each quantity is a concrete struct wrapping `(f64, UnitEnum)`, implementing the `Quantity` trait.

```rust
// Rust
pub struct Length { value: f64, unit: LengthUnit }
impl Quantity for Length { type Unit = LengthUnit; ... }
```

**Why**: Rust has no class inheritance. Traits + structs achieve the same API surface. The `Quantity` trait with an associated type (`type Unit`) replaces F-bounded polymorphism.

### 2. Unit Enums Instead of Singleton Objects

**Scala**: Units are singleton objects (`object Meters extends LengthUnit`).

**Rust**: Units are enum variants (`LengthUnit::Meters`).

```rust
// Rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LengthUnit { Meters, Kilometers, Feet, ... }
```

**Why**: Rust enums are sum types -- they naturally model a fixed set of unit variants. They derive `Copy`, `Eq`, and `Hash` for free, and pattern matching replaces `isInstanceOf` checks. Enum variants also have no heap allocation overhead.

### 3. f64 Instead of BigDecimal for Money

**Scala**: `Money` uses `BigDecimal` for arbitrary-precision arithmetic.

**Rust**: `Money` uses `f64`.

**Why**: Keeping `Money` consistent with all other quantities (which use `f64`) simplifies the API and allows the library to remain dependency-free. For applications requiring exact decimal arithmetic (e.g., financial accounting), users can convert to a decimal type at system boundaries.

### 4. No Implicit MoneyContext

**Scala**: Cross-currency operations use an implicit `MoneyContext` carrying exchange rates, allowing `usd + eur` if a context is in scope.

**Rust**: Cross-currency arithmetic panics. Currency conversion requires an explicit `CurrencyExchangeRate::convert()` call.

```rust
// This panics in Rust:
// let _ = Money::usd(100.0) + Money::eur(50.0);

// Use explicit conversion instead:
let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.85);
let eur_as_usd = rate.convert(Money::eur(50.0)).unwrap();
let total = Money::usd(100.0) + eur_as_usd;
```

**Why**: Rust has no implicit parameters. Passing a context explicitly is the idiomatic approach and avoids hidden dependencies. This makes the code clearer about when and how currency conversion happens.

### 5. No Pattern Matching Extractors on Units

**Scala**: Units have `unapply` methods enabling `Meters(value)` pattern matching.

**Rust**: Use `quantity.value()` and `quantity.unit()` directly, or `match` on the unit enum.

```rust
let length = Length::meters(5.0);
match length.unit() {
    LengthUnit::Meters => println!("in meters: {}", length.value()),
    _ => println!("other unit"),
}
```

**Why**: Rust's pattern matching is on enum variants, not on types with extractors. The explicit `value()` + `unit()` approach is idiomatic.

### 6. DSL via Extension Traits Instead of Implicit Conversions

**Scala**: `import squants.space.LengthConversions._` enables `5.meters`, `100.kilometers` via implicit conversions.

**Rust**: Extension traits on `f64` (and `i32`) provide the same syntax.

```rust
use rquants::prelude::*;  // imports all conversion traits

let d = 100.0.meters();
let t = 5.0.seconds();
```

**Why**: Rust's `impl Trait for f64` is the equivalent of Scala's implicit class pattern. Both achieve `5.0.meters()` syntax. The prelude re-exports all conversion traits for convenience.

### 7. Operator Panics for Invalid Cross-Type Operations

**Scala**: Some operations return `Try[A]` or use implicit contexts.

**Rust**: Invalid operations (e.g., adding different currencies) panic via `unwrap()` in the `Add` impl.

**Why**: Rust's `std::ops::Add` requires `type Output`, not `Result<Output>`. Returning `Result` from `+` would break ergonomics. The trade-off is a runtime panic for programmer errors (mixing currencies), which is caught by tests. Future versions could explore a `checked_add` method returning `Result`.

### 8. Temperature Does Not Implement the Quantity Trait

**Scala**: `Temperature extends Quantity[Temperature]` with overridden arithmetic.

**Rust**: `Temperature` is a standalone struct with its own `Add`, `Sub`, `Mul` impls but does *not* implement the `Quantity` trait.

**Why**: Temperature's non-linear conversions (scale vs. degree) are incompatible with the `Quantity` trait's assumption that units have linear conversion factors. Making Temperature standalone avoids incorrect behavior and makes the scale/degree distinction explicit in the API.

### 9. Currency as an Enum, Not Extensible at Runtime

**Scala**: Currencies are objects extending an abstract `Currency` class. Users can define custom currencies.

**Rust**: `Currency` is a closed enum with 10 predefined currencies (USD, EUR, GBP, JPY, CHF, CAD, AUD, CNY, INR, BTC).

**Why**: A closed enum is simpler and avoids dynamic dispatch. For applications needing custom currencies, this can be extended to a more open design in a future version. The current set covers the most common use cases.

### 10. Fewer Currencies Than Scala squants

**Scala squants**: 30+ currencies including cryptocurrencies (ETH, XRP, etc.) and many national currencies.

**Rust rquants**: 10 currencies. The most commonly used global currencies plus Bitcoin.

**Why**: Keeping the initial set small reduces code size. Additional currencies can be added trivially by extending the `Currency` enum.

### 11. No QuantityRange Operators (+-) or (to)

**Scala**: `quantity +- tolerance` creates a `QuantityRange`, and `quantity.to(other)` creates a range.

**Rust**: `QuantityRange::new(lower, upper)` is the explicit constructor.

**Why**: Rust reserves `..` for its own range syntax. Custom range operators would require macro-based DSLs that hurt readability. The explicit constructor is clear and sufficient.

### 12. Module Naming Mirrors Scala Source Structure

Module names like `energy::energy`, `mass::mass`, `time::time` intentionally mirror the Scala package structure (`squants.energy.Energy`, `squants.mass.Mass`). Clippy's `module_inception` lint is suppressed at the crate level for this reason.

### 13. No Serialization by Default

**Scala squants**: Extends `Serializable` (JVM serialization).

**Rust rquants**: No built-in serialization. Users can implement serialization as needed for their use case.

### 14. Missing Quantities vs. Scala squants

The following quantities from Scala squants are not yet ported:

- **Motion**: Jerk, Yank, PressureChange, MassFlow, VolumeFlow, AngularVelocity, AngularAcceleration, Torque, SurfaceTension, Viscosity
- **Electro**: Permittivity, Permeability, ElectricFieldStrength, MagneticFieldStrength, ElectricCurrentDensity, ElectricChargeDensity, LinearElectricChargeDensity, AreaElectricChargeDensity
- **Radio**: AreaTime, SpectralIntensity
- **Space**: TimeSquared (used internally by squants for calculus operations)
- **Market**: `MoneyContext` (implicit exchange rate context)

These can be added in future versions following the same patterns established in the existing modules.

---

## API Reference

Run `cargo doc --open` to browse the full API documentation locally.

## Testing

```bash
cargo test          # 320 unit tests + 83 doc tests
cargo clippy        # zero warnings
cargo doc --no-deps # zero warnings
```

## License

Apache-2.0
