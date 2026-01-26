# RQuants: Porting Squants from Scala to Rust

## Overview

Port the Scala `squants` library (99 files, ~10,600 LOC) to Rust as `rquants`, maintaining API parity while leveraging Rust idioms for type-safe dimensional analysis.

**Source**: `squants/shared/src/main/scala/squants/`
**Target**: `rquants/`

---

## Architecture Decisions

### Core Design: Newtype + Trait Hybrid

```rust
// Each quantity is a newtype wrapping value and unit
pub struct Length {
    value: f64,
    unit: LengthUnit,
}

// Units are enums with conversion factors
pub enum LengthUnit {
    Meters,
    Kilometers,
    Feet,
    // ...
}

impl LengthUnit {
    pub const fn conversion_factor(&self) -> f64 { ... }
    pub const fn symbol(&self) -> &'static str { ... }
}
```

**Rationale**: Matches squants' runtime unit tracking, supports parsing ("10 km"), readable error messages.

### Key Traits

| Trait | Purpose |
|-------|---------|
| `Quantity` | Core trait: `value()`, `unit()`, `to()`, `in_unit()` |
| `UnitOfMeasure<Q>` | Unit definition: `symbol()`, `conversion_factor()` |
| `Dimension<Q>` | Dimension metadata: `name()`, `primary_unit()`, `parse()` |
| `TimeIntegral<D>` | Calculus: `per(time) -> D` (e.g., Length / Time = Velocity) |
| `TimeDerivative<I>` | Calculus: `times(time) -> I` (e.g., Velocity * Time = Length) |
| `ApproxEq` | Tolerance comparison: `approx_eq(&other, &tolerance)` |

### Operator Overloading via `std::ops`

- `Add`, `Sub` for same-type quantities
- `Mul<f64>`, `Div<f64>` for scalar operations
- Cross-quantity ops: `Div<Time> for Length` returns `Velocity`

### Special Cases

1. **Temperature**: Non-linear conversions (scale vs degree offsets)
2. **Money**: Runtime currency via `MoneyContext`, uses `rust_decimal` for precision

---

## Module Structure

```
rquants/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── prelude.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── quantity.rs         # Quantity trait
│   │   ├── unit.rs             # UnitOfMeasure trait
│   │   ├── dimension.rs        # Dimension trait
│   │   ├── dimensionless.rs
│   │   ├── quantity_range.rs
│   │   ├── ratio.rs
│   │   └── error.rs
│   ├── systems/
│   │   ├── mod.rs
│   │   ├── metric.rs           # KILO, MEGA, GIGA...
│   │   └── binary.rs           # KIBI, MEBI...
│   ├── time/                   # Time, Frequency, TimeDerivative traits
│   ├── space/                  # Length, Area, Volume, Angle, SolidAngle
│   ├── mass/                   # Mass, Density, ChemicalAmount, MomentOfInertia
│   ├── motion/                 # Velocity, Acceleration, Force, Momentum, Pressure...
│   ├── energy/                 # Energy, Power, PowerRamp, SpecificEnergy...
│   ├── thermal/                # Temperature (special), ThermalCapacity
│   ├── electro/                # 21 quantities: Current, Charge, Potential, Resistance...
│   ├── radio/                  # Activity, Dose, Irradiance, ParticleFlux...
│   ├── photo/                  # LuminousIntensity, Flux, Illuminance, Luminance...
│   ├── information/            # Information, DataRate
│   └── market/                 # Money, Currency, MoneyContext, Price<Q>
├── tests/
└── examples/
```

---

## Phased Implementation Plan

### Phase 1: Project Setup & Core Framework
**Commit**: `feat: initialize rquants with core quantity framework`

**Files**:
- `Cargo.toml` with dependencies
- `src/lib.rs`, `src/prelude.rs`
- `src/core/mod.rs`, `quantity.rs`, `unit.rs`, `dimension.rs`, `error.rs`
- `src/systems/mod.rs`, `metric.rs`, `binary.rs`

**Tests**: Core trait implementations, metric/binary prefix constants

---

### Phase 2: Base Quantities (No Dependencies)
**Commit**: `feat: implement base quantities (time, space, mass)`

**Modules**:
- `time/` - Time, Frequency, TimeSquared, TimeDerivative/TimeIntegral traits
- `space/` - Length (30+ units), Area, Volume, Angle, SolidAngle
- `mass/` - Mass, Density, AreaDensity, ChemicalAmount, MomentOfInertia

**Tests**: Unit conversions, arithmetic, parsing, cross-quantity ops (Area = Length * Length)

---

### Phase 3: Core Extensions
**Commit**: `feat: implement dimensionless, ranges, and ratios`

**Files**:
- `src/core/dimensionless.rs` - Each, Percent, Dozen
- `src/core/quantity_range.rs` - QuantityRange<Q>
- `src/core/ratio.rs` - Ratio types

**Tests**: Range operations, dimensionless arithmetic

---

### Phase 4: Motion Module
**Commit**: `feat: implement motion quantities`

**Dependencies**: time, space, mass

**Quantities** (14 files):
- Velocity, Acceleration, Jerk
- Force, Momentum, Yank
- Pressure, PressureChange
- Torque, MassFlow, VolumeFlow
- AngularVelocity, AngularAcceleration

**Tests**: F=ma, dimensional chain (position -> velocity -> acceleration -> jerk)

---

### Phase 5: Energy Module
**Commit**: `feat: implement energy quantities`

**Dependencies**: motion, time, mass

**Quantities** (8 files):
- Energy (Joules, WattHours, BTU, etc.)
- Power, PowerRamp
- PowerDensity, EnergyDensity
- SpecificEnergy, MolarEnergy

**Tests**: Power = Energy / Time, Work = Force * Distance

---

### Phase 6: Thermal Module
**Commit**: `feat: implement thermal quantities`

**Dependencies**: energy

**Special handling**: Temperature requires scale vs degree conversions
- Kelvin ↔ Celsius ↔ Fahrenheit ↔ Rankine with zero offsets

**Quantities**:
- Temperature (custom conversion logic)
- ThermalCapacity

**Tests**: Scale conversions (5°C = 41°F), degree conversions (5°C delta = 9°F delta)

---

### Phase 7: Electro Module
**Commit**: `feat: implement electromagnetic quantities`

**Dependencies**: energy, thermal, mass, time

**Quantities** (21 files):
- ElectricCurrent, ElectricCharge, ElectricPotential
- Resistance, Conductance, Capacitance, Inductance
- MagneticFlux, MagneticFluxDensity
- Resistivity, Conductivity
- Permittivity, Permeability
- FieldStrengths, CurrentDensity, ChargeDensities

**Tests**: Ohm's law (V = IR), Power = V * I

---

### Phase 8: Information Module
**Commit**: `feat: implement information quantities`

**Standalone module**

**Quantities**:
- Information (Bytes, Bits with metric/binary prefixes)
- DataRate = Information / Time

**Tests**: Byte/bit conversions, metric vs binary prefixes (KB vs KiB)

---

### Phase 9: Radio Module
**Commit**: `feat: implement radiation quantities`

**Dependencies**: energy, space, time

**Quantities** (11 files):
- Activity (Becquerel)
- Dose (Gray, Sievert)
- Irradiance, Radiance, RadiantIntensity
- SpectralIntensity, SpectralIrradiance, SpectralPower
- ParticleFlux, AreaTime

**Tests**: Radiation dose calculations

---

### Phase 10: Photo Module
**Commit**: `feat: implement photometric quantities`

**Dependencies**: energy, space, time

**Quantities** (7 files):
- LuminousIntensity (Candela)
- LuminousFlux (Lumen)
- Illuminance (Lux)
- Luminance, LuminousEnergy, LuminousExposure

**Tests**: Photometric relationships

---

### Phase 11: Market Module
**Commit**: `feat: implement financial quantities`

**Standalone, complex** - requires runtime currency handling

**Quantities**:
- Currency (USD, EUR, JPY, etc.)
- Money (uses `rust_decimal` for precision)
- MoneyContext (exchange rates)
- Price<Q> (generic over any quantity)
- CurrencyExchangeRate

**Tests**: Currency conversions, Price calculations

---

### Phase 12: Integration & Polish
**Commit**: `feat: add prelude and cross-module integration`

**Files**:
- Update `src/prelude.rs` with common re-exports
- Add feature flags in `Cargo.toml`
- Integration tests across modules

**Commit**: `docs: add comprehensive documentation and examples`

- README.md with usage examples
- Doc comments for all public APIs
- `examples/basic_usage.rs`
- `examples/physics_calculations.rs`
- `examples/financial_calculations.rs`

---

## Dependencies (Cargo.toml)

```toml
[package]
name = "rquants"
version = "0.1.0"
edition = "2021"

[dependencies]
thiserror = "1.0"           # Error handling
rust_decimal = "1.0"        # Money precision
lazy_static = "1.0"         # Static unit collections

[dev-dependencies]
proptest = "1.0"            # Property-based testing

[features]
default = []
serde = ["dep:serde"]       # Optional serialization
```

---

## Required Permissions

Add these to Claude Code settings as allowed commands:

```
# Cargo commands
cargo init
cargo build
cargo test
cargo test --all-features
cargo clippy
cargo fmt
cargo doc
cargo bench

# Git commands
git init
git add
git commit
git status
git diff
git log
```

---

## Testing Strategy

1. **Unit tests** per module: conversions, arithmetic, parsing
2. **Integration tests**: cross-module operations (F=ma, P=IV)
3. **Property tests**: conversion roundtrips, commutativity
4. **Doc tests**: all public API examples

---

## Verification Plan

After each phase:
1. `cargo build` - compiles without errors
2. `cargo test` - all tests pass
3. `cargo clippy` - no warnings
4. `cargo doc` - documentation builds

Final verification:
1. Run all examples
2. Verify cross-module physics calculations work correctly
3. Verify Money/Currency operations with exchange rates
4. Check that Temperature scale/degree conversions match squants behavior

---

## Critical Reference Files

- `squants/shared/src/main/scala/squants/Quantity.scala` - Core pattern
- `squants/shared/src/main/scala/squants/UnitOfMeasure.scala` - Unit traits
- `squants/shared/src/main/scala/squants/thermal/Temperature.scala` - Non-linear conversions
- `squants/shared/src/main/scala/squants/market/Money.scala` - Runtime currency handling
- `squants/shared/src/main/scala/squants/time/Time.scala` - TimeIntegral example
