//! Chemical amount (substance) quantity and units.

use crate::core::macros::{impl_dimension, impl_quantity, impl_unit_display};
use crate::core::{Quantity, UnitOfMeasure};

/// Units of chemical amount (substance) measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChemicalAmountUnit {
    /// Moles (mol) - SI base unit
    Moles,
    /// Pound-moles (lb-mol)
    PoundMoles,
}

impl ChemicalAmountUnit {
    /// All available chemical amount units.
    pub const ALL: &'static [ChemicalAmountUnit] = &[
        ChemicalAmountUnit::Moles,
        ChemicalAmountUnit::PoundMoles,
    ];
}

// Conversion factors to moles (primary unit)
// 1 lb-mol = 453.59237 mol (same as pounds to grams ratio)
const POUND_MOLE_FACTOR: f64 = 453.59237;

impl_unit_display!(ChemicalAmountUnit);

impl UnitOfMeasure for ChemicalAmountUnit {
    fn symbol(&self) -> &'static str {
        match self {
            ChemicalAmountUnit::Moles => "mol",
            ChemicalAmountUnit::PoundMoles => "lb-mol",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            ChemicalAmountUnit::Moles => 1.0,
            ChemicalAmountUnit::PoundMoles => POUND_MOLE_FACTOR,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, ChemicalAmountUnit::Moles)
    }
}

/// A quantity of chemical amount (amount of substance).
///
/// Chemical amount, measured in moles, represents the amount of a substance
/// containing as many elementary entities as there are atoms in 12 grams
/// of carbon-12 (Avogadro's number).
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let amount = ChemicalAmount::moles(2.0);
/// let amount_lb = ChemicalAmount::pound_moles(1.0);
///
/// // 1 pound-mole ≈ 453.59 moles
/// assert!((amount_lb.to_moles() - 453.59237).abs() < 0.001);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ChemicalAmount {
    value: f64,
    unit: ChemicalAmountUnit,
}

impl ChemicalAmount {
    /// Creates a new ChemicalAmount quantity.
    pub const fn new_const(value: f64, unit: ChemicalAmountUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates a ChemicalAmount in moles.
    pub fn moles(value: f64) -> Self {
        Self::new(value, ChemicalAmountUnit::Moles)
    }

    /// Creates a ChemicalAmount in pound-moles.
    pub fn pound_moles(value: f64) -> Self {
        Self::new(value, ChemicalAmountUnit::PoundMoles)
    }

    // Conversion methods
    /// Converts to moles.
    pub fn to_moles(&self) -> f64 {
        self.to(ChemicalAmountUnit::Moles)
    }

    /// Converts to pound-moles.
    pub fn to_pound_moles(&self) -> f64 {
        self.to(ChemicalAmountUnit::PoundMoles)
    }
}

impl_quantity!(ChemicalAmount, ChemicalAmountUnit);

impl_dimension!(
    ChemicalAmountDimension,
    ChemicalAmount,
    ChemicalAmountUnit,
    "ChemicalAmount",
    ChemicalAmountUnit::Moles,
    ChemicalAmountUnit::Moles
);

/// Extension trait for creating ChemicalAmount quantities from numeric types.
pub trait ChemicalAmountConversions {
    /// Creates a ChemicalAmount in moles.
    fn moles(self) -> ChemicalAmount;
    /// Creates a ChemicalAmount in pound-moles.
    fn pound_moles(self) -> ChemicalAmount;
}

impl ChemicalAmountConversions for f64 {
    fn moles(self) -> ChemicalAmount {
        ChemicalAmount::moles(self)
    }
    fn pound_moles(self) -> ChemicalAmount {
        ChemicalAmount::pound_moles(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chemical_amount_creation() {
        let n = ChemicalAmount::moles(1.0);
        assert_eq!(n.value(), 1.0);
        assert_eq!(n.unit(), ChemicalAmountUnit::Moles);
    }

    #[test]
    fn test_chemical_amount_conversions() {
        let n = ChemicalAmount::pound_moles(1.0);
        assert!((n.to_moles() - 453.59237).abs() < 0.001);

        let n2 = ChemicalAmount::moles(453.59237);
        assert!((n2.to_pound_moles() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_chemical_amount_arithmetic() {
        let n1 = ChemicalAmount::moles(2.0);
        let n2 = ChemicalAmount::moles(3.0);
        let sum = n1 + n2;
        assert_eq!(sum.to_moles(), 5.0);
    }
}
