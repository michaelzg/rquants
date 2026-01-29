//! Energy quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use crate::mass::{ChemicalAmount, ChemicalAmountUnit, Mass, MassUnit};
use crate::motion::{Force, ForceUnit};
use crate::space::{Length, LengthUnit, Volume, VolumeUnit};
use crate::time::{Time, TimeUnit};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of energy measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnergyUnit {
    // Watt-hours family (primary)
    /// Watt-hours (Wh) - primary unit
    WattHours,
    /// Milliwatt-hours (mWh)
    MilliwattHours,
    /// Kilowatt-hours (kWh)
    KilowattHours,
    /// Megawatt-hours (MWh)
    MegawattHours,
    /// Gigawatt-hours (GWh)
    GigawattHours,

    // Joules family (SI)
    /// Joules (J) - SI unit
    Joules,
    /// Picojoules (pJ)
    Picojoules,
    /// Nanojoules (nJ)
    Nanojoules,
    /// Microjoules (µJ)
    Microjoules,
    /// Millijoules (mJ)
    Millijoules,
    /// Kilojoules (kJ)
    Kilojoules,
    /// Megajoules (MJ)
    Megajoules,
    /// Gigajoules (GJ)
    Gigajoules,
    /// Terajoules (TJ)
    Terajoules,

    // BTU family
    /// British Thermal Units (BTU)
    BritishThermalUnits,
    /// Thousand BTU (MBtu)
    MBtus,
    /// Million BTU (MMBtu)
    MMBtus,

    // Electron-volts family
    /// Electron-volts (eV)
    ElectronVolts,
    /// Milli-electron-volts (meV)
    MilliElectronVolts,
    /// Kilo-electron-volts (keV)
    KiloElectronVolts,
    /// Mega-electron-volts (MeV)
    MegaElectronVolts,
    /// Giga-electron-volts (GeV)
    GigaElectronVolts,
    /// Tera-electron-volts (TeV)
    TeraElectronVolts,

    // Other
    /// Ergs (erg) - CGS unit
    Ergs,
    /// Calories (cal)
    Calories,
    /// Kilocalories (kcal)
    Kilocalories,
}

impl EnergyUnit {
    /// All available energy units.
    pub const ALL: &'static [EnergyUnit] = &[
        EnergyUnit::WattHours,
        EnergyUnit::MilliwattHours,
        EnergyUnit::KilowattHours,
        EnergyUnit::MegawattHours,
        EnergyUnit::GigawattHours,
        EnergyUnit::Joules,
        EnergyUnit::Picojoules,
        EnergyUnit::Nanojoules,
        EnergyUnit::Microjoules,
        EnergyUnit::Millijoules,
        EnergyUnit::Kilojoules,
        EnergyUnit::Megajoules,
        EnergyUnit::Gigajoules,
        EnergyUnit::Terajoules,
        EnergyUnit::BritishThermalUnits,
        EnergyUnit::MBtus,
        EnergyUnit::MMBtus,
        EnergyUnit::ElectronVolts,
        EnergyUnit::MilliElectronVolts,
        EnergyUnit::KiloElectronVolts,
        EnergyUnit::MegaElectronVolts,
        EnergyUnit::GigaElectronVolts,
        EnergyUnit::TeraElectronVolts,
        EnergyUnit::Ergs,
        EnergyUnit::Calories,
        EnergyUnit::Kilocalories,
    ];
}

// Conversion factors relative to WattHours (primary unit)
const SECONDS_PER_HOUR: f64 = 3600.0;

// Joules = Watt-seconds, so 1 Wh = 3600 J
// Joule conversion factor to Wh = 1/3600
const JOULE_TO_WH: f64 = 1.0 / SECONDS_PER_HOUR;

// BTU conversion (1 BTU ≈ 0.293071 Wh)
const BTU_TO_WH: f64 = 0.2930710701722222;

// Electron-volt (1 eV ≈ 1.602176565e-19 J)
const EV_TO_J: f64 = 1.602176565e-19;
const EV_TO_WH: f64 = EV_TO_J * JOULE_TO_WH;

// Calorie (1 cal = 4.184 J)
const CAL_TO_J: f64 = 4.184;
const CAL_TO_WH: f64 = CAL_TO_J * JOULE_TO_WH;

impl fmt::Display for EnergyUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for EnergyUnit {
    fn symbol(&self) -> &'static str {
        match self {
            EnergyUnit::WattHours => "Wh",
            EnergyUnit::MilliwattHours => "mWh",
            EnergyUnit::KilowattHours => "kWh",
            EnergyUnit::MegawattHours => "MWh",
            EnergyUnit::GigawattHours => "GWh",
            EnergyUnit::Joules => "J",
            EnergyUnit::Picojoules => "pJ",
            EnergyUnit::Nanojoules => "nJ",
            EnergyUnit::Microjoules => "µJ",
            EnergyUnit::Millijoules => "mJ",
            EnergyUnit::Kilojoules => "kJ",
            EnergyUnit::Megajoules => "MJ",
            EnergyUnit::Gigajoules => "GJ",
            EnergyUnit::Terajoules => "TJ",
            EnergyUnit::BritishThermalUnits => "BTU",
            EnergyUnit::MBtus => "MBtu",
            EnergyUnit::MMBtus => "MMBtu",
            EnergyUnit::ElectronVolts => "eV",
            EnergyUnit::MilliElectronVolts => "meV",
            EnergyUnit::KiloElectronVolts => "keV",
            EnergyUnit::MegaElectronVolts => "MeV",
            EnergyUnit::GigaElectronVolts => "GeV",
            EnergyUnit::TeraElectronVolts => "TeV",
            EnergyUnit::Ergs => "erg",
            EnergyUnit::Calories => "cal",
            EnergyUnit::Kilocalories => "kcal",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            // Watt-hours family
            EnergyUnit::WattHours => 1.0,
            EnergyUnit::MilliwattHours => 1e-3,
            EnergyUnit::KilowattHours => 1e3,
            EnergyUnit::MegawattHours => 1e6,
            EnergyUnit::GigawattHours => 1e9,

            // Joules family (convert via J -> Wh)
            EnergyUnit::Joules => JOULE_TO_WH,
            EnergyUnit::Picojoules => JOULE_TO_WH * 1e-12,
            EnergyUnit::Nanojoules => JOULE_TO_WH * 1e-9,
            EnergyUnit::Microjoules => JOULE_TO_WH * 1e-6,
            EnergyUnit::Millijoules => JOULE_TO_WH * 1e-3,
            EnergyUnit::Kilojoules => JOULE_TO_WH * 1e3,
            EnergyUnit::Megajoules => JOULE_TO_WH * 1e6,
            EnergyUnit::Gigajoules => JOULE_TO_WH * 1e9,
            EnergyUnit::Terajoules => JOULE_TO_WH * 1e12,

            // BTU family
            EnergyUnit::BritishThermalUnits => BTU_TO_WH,
            EnergyUnit::MBtus => BTU_TO_WH * 1e3,
            EnergyUnit::MMBtus => BTU_TO_WH * 1e6,

            // Electron-volt family
            EnergyUnit::ElectronVolts => EV_TO_WH,
            EnergyUnit::MilliElectronVolts => EV_TO_WH * 1e-3,
            EnergyUnit::KiloElectronVolts => EV_TO_WH * 1e3,
            EnergyUnit::MegaElectronVolts => EV_TO_WH * 1e6,
            EnergyUnit::GigaElectronVolts => EV_TO_WH * 1e9,
            EnergyUnit::TeraElectronVolts => EV_TO_WH * 1e12,

            // Others
            EnergyUnit::Ergs => JOULE_TO_WH * 1e-7, // 1 erg = 1e-7 J
            EnergyUnit::Calories => CAL_TO_WH,
            EnergyUnit::Kilocalories => CAL_TO_WH * 1e3,
        }
    }

    fn is_si(&self) -> bool {
        matches!(
            self,
            EnergyUnit::Joules
                | EnergyUnit::Picojoules
                | EnergyUnit::Nanojoules
                | EnergyUnit::Microjoules
                | EnergyUnit::Millijoules
                | EnergyUnit::Kilojoules
                | EnergyUnit::Megajoules
                | EnergyUnit::Gigajoules
                | EnergyUnit::Terajoules
        )
    }
}

/// A quantity of energy.
///
/// Energy represents the capacity to do work.
///
/// # Relationships
///
/// - Energy = Power × Time
/// - Energy = Force × Distance
/// - Energy / Time = Power
/// - Energy / Mass = SpecificEnergy
/// - Energy / Volume = EnergyDensity
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let energy = Energy::kilowatt_hours(1.0);
/// assert!((energy.to_joules() - 3_600_000.0).abs() < 1.0);
///
/// let power = Power::watts(100.0);
/// let time = Time::hours(2.0);
/// let work = power * time;
/// assert!((work.to_watt_hours() - 200.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Energy {
    value: f64,
    unit: EnergyUnit,
}

impl Energy {
    /// Creates a new Energy quantity.
    pub const fn new_const(value: f64, unit: EnergyUnit) -> Self {
        Self { value, unit }
    }

    // Watt-hour constructors
    /// Creates an Energy in watt-hours.
    pub fn watt_hours(value: f64) -> Self {
        Self::new(value, EnergyUnit::WattHours)
    }

    /// Creates an Energy in milliwatt-hours.
    pub fn milliwatt_hours(value: f64) -> Self {
        Self::new(value, EnergyUnit::MilliwattHours)
    }

    /// Creates an Energy in kilowatt-hours.
    pub fn kilowatt_hours(value: f64) -> Self {
        Self::new(value, EnergyUnit::KilowattHours)
    }

    /// Creates an Energy in megawatt-hours.
    pub fn megawatt_hours(value: f64) -> Self {
        Self::new(value, EnergyUnit::MegawattHours)
    }

    /// Creates an Energy in gigawatt-hours.
    pub fn gigawatt_hours(value: f64) -> Self {
        Self::new(value, EnergyUnit::GigawattHours)
    }

    // Joule constructors
    /// Creates an Energy in joules.
    pub fn joules(value: f64) -> Self {
        Self::new(value, EnergyUnit::Joules)
    }

    /// Creates an Energy in kilojoules.
    pub fn kilojoules(value: f64) -> Self {
        Self::new(value, EnergyUnit::Kilojoules)
    }

    /// Creates an Energy in megajoules.
    pub fn megajoules(value: f64) -> Self {
        Self::new(value, EnergyUnit::Megajoules)
    }

    // BTU constructors
    /// Creates an Energy in BTU.
    pub fn btus(value: f64) -> Self {
        Self::new(value, EnergyUnit::BritishThermalUnits)
    }

    // eV constructors
    /// Creates an Energy in electron-volts.
    pub fn electron_volts(value: f64) -> Self {
        Self::new(value, EnergyUnit::ElectronVolts)
    }

    // Calorie constructors
    /// Creates an Energy in calories.
    pub fn calories(value: f64) -> Self {
        Self::new(value, EnergyUnit::Calories)
    }

    /// Creates an Energy in kilocalories.
    pub fn kilocalories(value: f64) -> Self {
        Self::new(value, EnergyUnit::Kilocalories)
    }

    // Conversion methods
    /// Converts to watt-hours.
    pub fn to_watt_hours(&self) -> f64 {
        self.to(EnergyUnit::WattHours)
    }

    /// Converts to kilowatt-hours.
    pub fn to_kilowatt_hours(&self) -> f64 {
        self.to(EnergyUnit::KilowattHours)
    }

    /// Converts to megawatt-hours.
    pub fn to_megawatt_hours(&self) -> f64 {
        self.to(EnergyUnit::MegawattHours)
    }

    /// Converts to joules.
    pub fn to_joules(&self) -> f64 {
        self.to(EnergyUnit::Joules)
    }

    /// Converts to kilojoules.
    pub fn to_kilojoules(&self) -> f64 {
        self.to(EnergyUnit::Kilojoules)
    }

    /// Converts to BTU.
    pub fn to_btus(&self) -> f64 {
        self.to(EnergyUnit::BritishThermalUnits)
    }

    /// Converts to electron-volts.
    pub fn to_electron_volts(&self) -> f64 {
        self.to(EnergyUnit::ElectronVolts)
    }

    /// Converts to calories.
    pub fn to_calories(&self) -> f64 {
        self.to(EnergyUnit::Calories)
    }

    /// Converts to kilocalories.
    pub fn to_kilocalories(&self) -> f64 {
        self.to(EnergyUnit::Kilocalories)
    }

    /// Calculates kinetic energy from mass and velocity.
    /// KE = 0.5 * m * v²
    pub fn kinetic(mass: Mass, velocity: crate::motion::Velocity) -> Self {
        let v = velocity.to_meters_per_second();
        let m = mass.to_kilograms();
        Energy::joules(0.5 * m * v * v)
    }
}

impl fmt::Display for Energy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Energy {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Energy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Energy {
    type Unit = EnergyUnit;

    fn new(value: f64, unit: Self::Unit) -> Self {
        Self { value, unit }
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn unit(&self) -> Self::Unit {
        self.unit
    }
}

// Arithmetic operations

impl Add for Energy {
    type Output = Energy;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Energy::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Energy {
    type Output = Energy;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Energy::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Energy {
    type Output = Energy;

    fn mul(self, rhs: f64) -> Self::Output {
        Energy::new(self.value * rhs, self.unit)
    }
}

impl Mul<Energy> for f64 {
    type Output = Energy;

    fn mul(self, rhs: Energy) -> Self::Output {
        Energy::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for Energy {
    type Output = Energy;

    fn div(self, rhs: f64) -> Self::Output {
        Energy::new(self.value / rhs, self.unit)
    }
}

impl Div<Energy> for Energy {
    type Output = f64;

    fn div(self, rhs: Energy) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Energy {
    type Output = Energy;

    fn neg(self) -> Self::Output {
        Energy::new(-self.value, self.unit)
    }
}

// Forward declarations for cross-quantity operations
// These will be implemented after Power is defined
use super::energy_density::{EnergyDensity, EnergyDensityUnit};
use super::molar_energy::{MolarEnergy, MolarEnergyUnit};
use super::power::{Power, PowerUnit};
use super::specific_energy::{SpecificEnergy, SpecificEnergyUnit};

// Energy / Time = Power
impl Div<Time> for Energy {
    type Output = Power;

    fn div(self, rhs: Time) -> Self::Output {
        let watts = self.to_joules() / rhs.to_seconds();
        Power::new(watts, PowerUnit::Watts)
    }
}

// Energy / Power = Time
impl Div<Power> for Energy {
    type Output = Time;

    fn div(self, rhs: Power) -> Self::Output {
        let seconds = self.to_joules() / rhs.to_watts();
        Time::new(seconds, TimeUnit::Seconds)
    }
}

// Energy / Mass = SpecificEnergy
impl Div<Mass> for Energy {
    type Output = SpecificEnergy;

    fn div(self, rhs: Mass) -> Self::Output {
        let grays = self.to_joules() / rhs.to_kilograms();
        SpecificEnergy::new(grays, SpecificEnergyUnit::Grays)
    }
}

// Energy / SpecificEnergy = Mass
impl Div<SpecificEnergy> for Energy {
    type Output = Mass;

    fn div(self, rhs: SpecificEnergy) -> Self::Output {
        let kg = self.to_joules() / rhs.to_grays();
        Mass::new(kg, MassUnit::Kilograms)
    }
}

// Energy / Volume = EnergyDensity
impl Div<Volume> for Energy {
    type Output = EnergyDensity;

    fn div(self, rhs: Volume) -> Self::Output {
        let jpcm = self.to_joules() / rhs.to_cubic_meters();
        EnergyDensity::new(jpcm, EnergyDensityUnit::JoulesPerCubicMeter)
    }
}

// Energy / EnergyDensity = Volume
impl Div<EnergyDensity> for Energy {
    type Output = Volume;

    fn div(self, rhs: EnergyDensity) -> Self::Output {
        let m3 = self.to_joules() / rhs.to_joules_per_cubic_meter();
        Volume::new(m3, VolumeUnit::CubicMeters)
    }
}

// Energy / ChemicalAmount = MolarEnergy
impl Div<ChemicalAmount> for Energy {
    type Output = MolarEnergy;

    fn div(self, rhs: ChemicalAmount) -> Self::Output {
        let jpm = self.to_joules() / rhs.to_moles();
        MolarEnergy::new(jpm, MolarEnergyUnit::JoulesPerMole)
    }
}

// Energy / MolarEnergy = ChemicalAmount
impl Div<MolarEnergy> for Energy {
    type Output = ChemicalAmount;

    fn div(self, rhs: MolarEnergy) -> Self::Output {
        let mol = self.to_joules() / rhs.to_joules_per_mole();
        ChemicalAmount::new(mol, ChemicalAmountUnit::Moles)
    }
}

// Energy / Force = Length (Work = Force × Distance)
impl Div<Force> for Energy {
    type Output = Length;

    fn div(self, rhs: Force) -> Self::Output {
        let meters = self.to_joules() / rhs.to_newtons();
        Length::new(meters, LengthUnit::Meters)
    }
}

// Energy / Length = Force
impl Div<Length> for Energy {
    type Output = Force;

    fn div(self, rhs: Length) -> Self::Output {
        let newtons = self.to_joules() / rhs.to_meters();
        Force::new(newtons, ForceUnit::Newtons)
    }
}

/// Dimension for Energy.
pub struct EnergyDimension;

impl Dimension for EnergyDimension {
    type Quantity = Energy;
    type Unit = EnergyUnit;

    fn name() -> &'static str {
        "Energy"
    }

    fn primary_unit() -> Self::Unit {
        EnergyUnit::WattHours
    }

    fn si_unit() -> Self::Unit {
        EnergyUnit::Joules
    }

    fn units() -> &'static [Self::Unit] {
        EnergyUnit::ALL
    }
}

/// Extension trait for creating Energy quantities from numeric types.
pub trait EnergyConversions {
    /// Creates an Energy in joules.
    fn joules(self) -> Energy;
    /// Creates an Energy in kilojoules.
    fn kilojoules(self) -> Energy;
    /// Creates an Energy in watt-hours.
    fn watt_hours(self) -> Energy;
    /// Creates an Energy in kilowatt-hours.
    fn kilowatt_hours(self) -> Energy;
    /// Creates an Energy in BTU.
    fn btus(self) -> Energy;
    /// Creates an Energy in calories.
    fn calories(self) -> Energy;
    /// Creates an Energy in kilocalories.
    fn kilocalories(self) -> Energy;
}

impl EnergyConversions for f64 {
    fn joules(self) -> Energy {
        Energy::joules(self)
    }
    fn kilojoules(self) -> Energy {
        Energy::kilojoules(self)
    }
    fn watt_hours(self) -> Energy {
        Energy::watt_hours(self)
    }
    fn kilowatt_hours(self) -> Energy {
        Energy::kilowatt_hours(self)
    }
    fn btus(self) -> Energy {
        Energy::btus(self)
    }
    fn calories(self) -> Energy {
        Energy::calories(self)
    }
    fn kilocalories(self) -> Energy {
        Energy::kilocalories(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_creation() {
        let e = Energy::joules(1000.0);
        assert_eq!(e.value(), 1000.0);
        assert_eq!(e.unit(), EnergyUnit::Joules);
    }

    #[test]
    fn test_joule_to_watt_hour() {
        let e = Energy::joules(3600.0);
        assert!((e.to_watt_hours() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_kwh_to_joules() {
        let e = Energy::kilowatt_hours(1.0);
        assert!((e.to_joules() - 3_600_000.0).abs() < 1.0);
    }

    #[test]
    fn test_btu_conversion() {
        let e = Energy::btus(1.0);
        // 1 BTU ≈ 1055.06 J
        assert!((e.to_joules() - 1055.06).abs() < 1.0);
    }

    #[test]
    fn test_calorie_conversion() {
        let e = Energy::calories(1.0);
        // 1 cal = 4.184 J
        assert!((e.to_joules() - 4.184).abs() < 0.001);
    }

    #[test]
    fn test_kilocalorie_conversion() {
        let e = Energy::kilocalories(1.0);
        // 1 kcal = 4184 J
        assert!((e.to_joules() - 4184.0).abs() < 1.0);
    }

    #[test]
    fn test_energy_arithmetic() {
        let e1 = Energy::joules(100.0);
        let e2 = Energy::joules(50.0);
        assert_eq!((e1 + e2).to_joules(), 150.0);
        assert_eq!((e1 - e2).to_joules(), 50.0);
    }
}
