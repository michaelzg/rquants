//! Energy quantity and units.

use crate::core::Quantity;
use crate::mass::{ChemicalAmount, ChemicalAmountUnit, Mass, MassUnit};
use crate::motion::{Force, ForceUnit};
use crate::space::{Length, LengthUnit, Volume, VolumeUnit};
use crate::time::{Time, TimeUnit};
use std::ops::Div;

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
crate::quantity! {
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
    pub quantity Energy {
        unit: EnergyUnit;
        dimension: EnergyDimension;
        conversions: EnergyConversions;
        name: "Energy";
        primary: WattHours;
        si: Joules;

        units {
            /// Watt-hours (Wh) - primary unit
            WattHours {
                symbol: "Wh",
                factor: 1.0,
                ctor: watt_hours,
                to: to_watt_hours,
                si: false
            },
            /// Milliwatt-hours (mWh)
            MilliwattHours {
                symbol: "mWh",
                factor: 1e-3,
                ctor: milliwatt_hours,
                to: to_milliwatt_hours,
                si: false
            },
            /// Kilowatt-hours (kWh)
            KilowattHours {
                symbol: "kWh",
                factor: 1e3,
                ctor: kilowatt_hours,
                to: to_kilowatt_hours,
                si: false
            },
            /// Megawatt-hours (MWh)
            MegawattHours {
                symbol: "MWh",
                factor: 1e6,
                ctor: megawatt_hours,
                to: to_megawatt_hours,
                si: false
            },
            /// Gigawatt-hours (GWh)
            GigawattHours {
                symbol: "GWh",
                factor: 1e9,
                ctor: gigawatt_hours,
                to: to_gigawatt_hours,
                si: false
            },
            /// Joules (J) - SI unit
            Joules {
                symbol: "J",
                factor: JOULE_TO_WH,
                ctor: joules,
                to: to_joules,
                si: true
            },
            /// Picojoules (pJ)
            Picojoules {
                symbol: "pJ",
                factor: JOULE_TO_WH * 1e-12,
                ctor: picojoules,
                to: to_picojoules,
                si: true
            },
            /// Nanojoules (nJ)
            Nanojoules {
                symbol: "nJ",
                factor: JOULE_TO_WH * 1e-9,
                ctor: nanojoules,
                to: to_nanojoules,
                si: true
            },
            /// Microjoules (µJ)
            Microjoules {
                symbol: "µJ",
                factor: JOULE_TO_WH * 1e-6,
                ctor: microjoules,
                to: to_microjoules,
                si: true
            },
            /// Millijoules (mJ)
            Millijoules {
                symbol: "mJ",
                factor: JOULE_TO_WH * 1e-3,
                ctor: millijoules,
                to: to_millijoules,
                si: true
            },
            /// Kilojoules (kJ)
            Kilojoules {
                symbol: "kJ",
                factor: JOULE_TO_WH * 1e3,
                ctor: kilojoules,
                to: to_kilojoules,
                si: true
            },
            /// Megajoules (MJ)
            Megajoules {
                symbol: "MJ",
                factor: JOULE_TO_WH * 1e6,
                ctor: megajoules,
                to: to_megajoules,
                si: true
            },
            /// Gigajoules (GJ)
            Gigajoules {
                symbol: "GJ",
                factor: JOULE_TO_WH * 1e9,
                ctor: gigajoules,
                to: to_gigajoules,
                si: true
            },
            /// Terajoules (TJ)
            Terajoules {
                symbol: "TJ",
                factor: JOULE_TO_WH * 1e12,
                ctor: terajoules,
                to: to_terajoules,
                si: true
            },
            /// British Thermal Units (BTU)
            BritishThermalUnits {
                symbol: "BTU",
                factor: BTU_TO_WH,
                ctor: btus,
                to: to_btus,
                si: false
            },
            /// Thousand BTU (MBtu)
            MBtus {
                symbol: "MBtu",
                factor: BTU_TO_WH * 1e3,
                ctor: mbtus,
                to: to_mbtus,
                si: false
            },
            /// Million BTU (MMBtu)
            MMBtus {
                symbol: "MMBtu",
                factor: BTU_TO_WH * 1e6,
                ctor: mmbtus,
                to: to_mmbtus,
                si: false
            },
            /// Electron-volts (eV)
            ElectronVolts {
                symbol: "eV",
                factor: EV_TO_WH,
                ctor: electron_volts,
                to: to_electron_volts,
                si: false
            },
            /// Milli-electron-volts (meV)
            MilliElectronVolts {
                symbol: "meV",
                factor: EV_TO_WH * 1e-3,
                ctor: milli_electron_volts,
                to: to_milli_electron_volts,
                si: false
            },
            /// Kilo-electron-volts (keV)
            KiloElectronVolts {
                symbol: "keV",
                factor: EV_TO_WH * 1e3,
                ctor: kilo_electron_volts,
                to: to_kilo_electron_volts,
                si: false
            },
            /// Mega-electron-volts (MeV)
            MegaElectronVolts {
                symbol: "MeV",
                factor: EV_TO_WH * 1e6,
                ctor: mega_electron_volts,
                to: to_mega_electron_volts,
                si: false
            },
            /// Giga-electron-volts (GeV)
            GigaElectronVolts {
                symbol: "GeV",
                factor: EV_TO_WH * 1e9,
                ctor: giga_electron_volts,
                to: to_giga_electron_volts,
                si: false
            },
            /// Tera-electron-volts (TeV)
            TeraElectronVolts {
                symbol: "TeV",
                factor: EV_TO_WH * 1e12,
                ctor: tera_electron_volts,
                to: to_tera_electron_volts,
                si: false
            },
            /// Ergs (erg) - CGS unit
            Ergs {
                symbol: "erg",
                factor: JOULE_TO_WH * 1e-7,
                ctor: ergs,
                to: to_ergs,
                si: false
            },
            /// Calories (cal)
            Calories {
                symbol: "cal",
                factor: CAL_TO_WH,
                ctor: calories,
                to: to_calories,
                si: false
            },
            /// Kilocalories (kcal)
            Kilocalories {
                symbol: "kcal",
                factor: CAL_TO_WH * 1e3,
                ctor: kilocalories,
                to: to_kilocalories,
                si: false
            }
        }
    }
}
impl Energy {
    /// Calculates kinetic energy from mass and velocity.
    /// KE = 0.5 * m * v²
    pub fn kinetic(mass: Mass, velocity: crate::motion::Velocity) -> Self {
        let v = velocity.to_meters_per_second();
        let m = mass.to_kilograms();
        Energy::joules(0.5 * m * v * v)
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Quantity;

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
