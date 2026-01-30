//! Activity quantity and units.

use crate::core::{Dimension, Quantity, UnitOfMeasure};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Units of radioactivity measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivityUnit {
    /// Becquerels (Bq) - SI unit
    Becquerels,
    /// Curies (Ci)
    Curies,
}

impl ActivityUnit {
    /// All available activity units.
    pub const ALL: &'static [ActivityUnit] = &[ActivityUnit::Becquerels, ActivityUnit::Curies];
}

// Conversion factor
const CURIE_TO_BECQUEREL: f64 = 3.7e10;

impl fmt::Display for ActivityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl UnitOfMeasure for ActivityUnit {
    fn symbol(&self) -> &'static str {
        match self {
            ActivityUnit::Becquerels => "Bq",
            ActivityUnit::Curies => "Ci",
        }
    }

    fn conversion_factor(&self) -> f64 {
        match self {
            ActivityUnit::Becquerels => 1.0,
            ActivityUnit::Curies => CURIE_TO_BECQUEREL,
        }
    }

    fn is_si(&self) -> bool {
        matches!(self, ActivityUnit::Becquerels)
    }
}

/// A quantity of radioactivity.
///
/// Activity represents the rate of radioactive decay.
/// SI unit: Becquerel (Bq) = 1 decay per second
/// 1 Ci = 3.7×10¹⁰ Bq
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let activity = Activity::becquerels(3.7e10);
/// assert!((activity.to_curies() - 1.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Activity {
    value: f64,
    unit: ActivityUnit,
}

impl Activity {
    /// Creates a new Activity quantity.
    pub const fn new_const(value: f64, unit: ActivityUnit) -> Self {
        Self { value, unit }
    }

    // Constructors
    /// Creates an Activity in becquerels.
    pub fn becquerels(value: f64) -> Self {
        Self::new(value, ActivityUnit::Becquerels)
    }

    /// Creates an Activity in curies.
    pub fn curies(value: f64) -> Self {
        Self::new(value, ActivityUnit::Curies)
    }

    // Conversion methods
    /// Converts to becquerels.
    pub fn to_becquerels(&self) -> f64 {
        self.to(ActivityUnit::Becquerels)
    }

    /// Converts to curies.
    pub fn to_curies(&self) -> f64 {
        self.to(ActivityUnit::Curies)
    }
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol())
    }
}

impl PartialEq for Activity {
    fn eq(&self, other: &Self) -> bool {
        (self.to_primary() - other.to_primary()).abs() < f64::EPSILON
    }
}

impl PartialOrd for Activity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Quantity for Activity {
    type Unit = ActivityUnit;

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

impl Add for Activity {
    type Output = Activity;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.to_primary() + rhs.to_primary();
        Activity::new(self.unit.convert_from_primary(sum), self.unit)
    }
}

impl Sub for Activity {
    type Output = Activity;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.to_primary() - rhs.to_primary();
        Activity::new(self.unit.convert_from_primary(diff), self.unit)
    }
}

impl Mul<f64> for Activity {
    type Output = Activity;

    fn mul(self, rhs: f64) -> Self::Output {
        Activity::new(self.value * rhs, self.unit)
    }
}

impl Mul<Activity> for f64 {
    type Output = Activity;

    fn mul(self, rhs: Activity) -> Self::Output {
        Activity::new(self * rhs.value, rhs.unit)
    }
}

impl Div<f64> for Activity {
    type Output = Activity;

    fn div(self, rhs: f64) -> Self::Output {
        Activity::new(self.value / rhs, self.unit)
    }
}

impl Div<Activity> for Activity {
    type Output = f64;

    fn div(self, rhs: Activity) -> Self::Output {
        self.to_primary() / rhs.to_primary()
    }
}

impl Neg for Activity {
    type Output = Activity;

    fn neg(self) -> Self::Output {
        Activity::new(-self.value, self.unit)
    }
}

/// Dimension for Activity.
pub struct ActivityDimension;

impl Dimension for ActivityDimension {
    type Quantity = Activity;
    type Unit = ActivityUnit;

    fn name() -> &'static str {
        "Activity"
    }

    fn primary_unit() -> Self::Unit {
        ActivityUnit::Becquerels
    }

    fn si_unit() -> Self::Unit {
        ActivityUnit::Becquerels
    }

    fn units() -> &'static [Self::Unit] {
        ActivityUnit::ALL
    }
}

/// Extension trait for creating Activity quantities from numeric types.
pub trait ActivityConversions {
    /// Creates an Activity in becquerels.
    fn becquerels(self) -> Activity;
    /// Creates an Activity in curies.
    fn curies(self) -> Activity;
}

impl ActivityConversions for f64 {
    fn becquerels(self) -> Activity {
        Activity::becquerels(self)
    }
    fn curies(self) -> Activity {
        Activity::curies(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_creation() {
        let a = Activity::becquerels(1000.0);
        assert_eq!(a.value(), 1000.0);
        assert_eq!(a.unit(), ActivityUnit::Becquerels);
    }

    #[test]
    fn test_activity_conversions() {
        let a = Activity::curies(1.0);
        assert_eq!(a.to_becquerels(), 3.7e10);

        let b = Activity::becquerels(3.7e10);
        assert!((b.to_curies() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_activity_arithmetic() {
        let a1 = Activity::becquerels(1000.0);
        let a2 = Activity::becquerels(500.0);
        let sum = a1 + a2;
        assert_eq!(sum.to_becquerels(), 1500.0);
    }
}
