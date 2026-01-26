//! Core quantity trait and implementations.

use super::unit::UnitOfMeasure;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};

/// Core trait for all measurable quantities.
///
/// A quantity represents a measurable value with an associated unit of measure.
/// This trait provides the foundational interface for all quantity types in rquants.
///
/// # Type Parameters
///
/// Implementing types should be self-referential (e.g., `Length: Quantity<Unit = LengthUnit>`)
/// to ensure type safety when performing operations.
///
/// # Example
///
/// ```ignore
/// pub struct Length {
///     value: f64,
///     unit: LengthUnit,
/// }
///
/// impl Quantity for Length {
///     type Unit = LengthUnit;
///
///     fn new(value: f64, unit: Self::Unit) -> Self {
///         Length { value, unit }
///     }
///
///     fn value(&self) -> f64 {
///         self.value
///     }
///
///     fn unit(&self) -> Self::Unit {
///         self.unit
///     }
/// }
/// ```
pub trait Quantity: Clone + Copy + Debug + Display + PartialEq + PartialOrd {
    /// The unit type associated with this quantity.
    type Unit: UnitOfMeasure;

    /// Creates a new quantity with the given value and unit.
    fn new(value: f64, unit: Self::Unit) -> Self;

    /// Returns the numeric value of this quantity.
    fn value(&self) -> f64;

    /// Returns the unit of this quantity.
    fn unit(&self) -> Self::Unit;

    /// Returns the value converted to the primary unit.
    fn to_primary(&self) -> f64 {
        self.unit().convert_to_primary(self.value())
    }

    /// Converts this quantity to a value in the specified unit.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let length = Length::meters(1000.0);
    /// assert_eq!(length.to(LengthUnit::Kilometers), 1.0);
    /// ```
    fn to(&self, target_unit: Self::Unit) -> f64 {
        if self.unit() == target_unit {
            self.value()
        } else {
            self.unit().convert_to(self.value(), &target_unit)
        }
    }

    /// Returns a new quantity with the same value but expressed in a different unit.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let length = Length::meters(1000.0);
    /// let length_km = length.in_unit(LengthUnit::Kilometers);
    /// assert_eq!(length_km.value(), 1.0);
    /// assert_eq!(length_km.unit(), LengthUnit::Kilometers);
    /// ```
    fn in_unit(&self, target_unit: Self::Unit) -> Self {
        if self.unit() == target_unit {
            *self
        } else {
            Self::new(self.to(target_unit), target_unit)
        }
    }

    /// Returns the negation of this quantity.
    fn negate(&self) -> Self {
        Self::new(-self.value(), self.unit())
    }

    /// Returns the absolute value of this quantity.
    fn abs(&self) -> Self {
        Self::new(self.value().abs(), self.unit())
    }

    /// Returns the smallest integer value greater than or equal to this quantity.
    fn ceil(&self) -> Self {
        Self::new(self.value().ceil(), self.unit())
    }

    /// Returns the largest integer value less than or equal to this quantity.
    fn floor(&self) -> Self {
        Self::new(self.value().floor(), self.unit())
    }

    /// Returns the nearest integer to this quantity.
    fn round(&self) -> Self {
        Self::new(self.value().round(), self.unit())
    }

    /// Applies a function to the underlying value, returning a new quantity.
    fn map<F>(&self, f: F) -> Self
    where
        F: FnOnce(f64) -> f64,
    {
        Self::new(f(self.value()), self.unit())
    }

    /// Compares this quantity to another, returning the ordering.
    ///
    /// Quantities are compared by their values in the primary unit.
    fn compare(&self, other: &Self) -> Ordering {
        let self_primary = self.to_primary();
        let other_primary = other.to_primary();
        self_primary.partial_cmp(&other_primary).unwrap_or(Ordering::Equal)
    }

    /// Returns the maximum of this quantity and another.
    fn max(&self, other: Self) -> Self {
        if self.compare(&other) == Ordering::Greater {
            *self
        } else {
            other.in_unit(self.unit())
        }
    }

    /// Returns the minimum of this quantity and another.
    fn min(&self, other: Self) -> Self {
        if self.compare(&other) == Ordering::Less {
            *self
        } else {
            other.in_unit(self.unit())
        }
    }

    /// Returns a tuple of (value, symbol).
    fn to_tuple(&self) -> (f64, &'static str) {
        (self.value(), self.unit().symbol())
    }

    /// Returns a tuple of (value, symbol) in the specified unit.
    fn to_tuple_in(&self, unit: Self::Unit) -> (f64, &'static str) {
        (self.to(unit), unit.symbol())
    }
}

/// Trait for approximate equality comparisons.
///
/// This is useful when comparing quantities that may have small floating-point errors.
pub trait ApproxEq<Rhs = Self> {
    /// The type used for tolerance.
    type Tolerance;

    /// Returns true if self and other are approximately equal within the given tolerance.
    fn approx_eq(&self, other: &Rhs, tolerance: &Self::Tolerance) -> bool;

    /// Returns true if self and other are approximately equal within 1e-10.
    fn approx_eq_default(&self, other: &Rhs) -> bool;
}

impl<Q: Quantity> ApproxEq for Q {
    type Tolerance = Q;

    fn approx_eq(&self, other: &Q, tolerance: &Q) -> bool {
        let diff = (self.to_primary() - other.to_primary()).abs();
        diff <= tolerance.to_primary().abs()
    }

    fn approx_eq_default(&self, other: &Q) -> bool {
        let diff = (self.to_primary() - other.to_primary()).abs();
        diff <= 1e-10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A simple test quantity for unit testing
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct TestQuantity {
        value: f64,
        unit: TestUnit,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestUnit {
        Base,
        Kilo,
    }

    impl std::fmt::Display for TestUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.symbol())
        }
    }

    impl UnitOfMeasure for TestUnit {
        fn symbol(&self) -> &'static str {
            match self {
                TestUnit::Base => "b",
                TestUnit::Kilo => "kb",
            }
        }

        fn conversion_factor(&self) -> f64 {
            match self {
                TestUnit::Base => 1.0,
                TestUnit::Kilo => 1000.0,
            }
        }
    }

    impl std::fmt::Display for TestQuantity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}", self.value, self.unit.symbol())
        }
    }

    impl PartialOrd for TestQuantity {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.compare(other))
        }
    }

    impl Quantity for TestQuantity {
        type Unit = TestUnit;

        fn new(value: f64, unit: Self::Unit) -> Self {
            TestQuantity { value, unit }
        }

        fn value(&self) -> f64 {
            self.value
        }

        fn unit(&self) -> Self::Unit {
            self.unit
        }
    }

    #[test]
    fn test_to_primary() {
        let q = TestQuantity::new(5.0, TestUnit::Kilo);
        assert_eq!(q.to_primary(), 5000.0);
    }

    #[test]
    fn test_to() {
        let q = TestQuantity::new(1000.0, TestUnit::Base);
        assert_eq!(q.to(TestUnit::Kilo), 1.0);
    }

    #[test]
    fn test_in_unit() {
        let q = TestQuantity::new(1000.0, TestUnit::Base);
        let q_kilo = q.in_unit(TestUnit::Kilo);
        assert_eq!(q_kilo.value(), 1.0);
        assert_eq!(q_kilo.unit(), TestUnit::Kilo);
    }

    #[test]
    fn test_negate() {
        let q = TestQuantity::new(5.0, TestUnit::Base);
        let neg = q.negate();
        assert_eq!(neg.value(), -5.0);
    }

    #[test]
    fn test_abs() {
        let q = TestQuantity::new(-5.0, TestUnit::Base);
        assert_eq!(q.abs().value(), 5.0);
    }

    #[test]
    fn test_compare() {
        let q1 = TestQuantity::new(1.0, TestUnit::Kilo);
        let q2 = TestQuantity::new(500.0, TestUnit::Base);
        assert_eq!(q1.compare(&q2), Ordering::Greater);
    }

    #[test]
    fn test_max_min() {
        let q1 = TestQuantity::new(1.0, TestUnit::Kilo);
        let q2 = TestQuantity::new(500.0, TestUnit::Base);

        assert_eq!(q1.max(q2).to_primary(), 1000.0);
        assert_eq!(q1.min(q2).to_primary(), 500.0);
    }

    #[test]
    fn test_approx_eq() {
        let q1 = TestQuantity::new(1.0, TestUnit::Base);
        let q2 = TestQuantity::new(1.0001, TestUnit::Base);
        let tolerance = TestQuantity::new(0.001, TestUnit::Base);

        assert!(q1.approx_eq(&q2, &tolerance));
    }
}
