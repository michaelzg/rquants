//! Quantity range types.
//!
//! Represents ranges of quantities with operations for containment checking,
//! iteration, and range manipulation.

use crate::core::error::QuantityError;
use crate::core::{Quantity, UnitOfMeasure};
use std::fmt;

/// A range of quantities from a lower bound to an upper bound.
///
/// The range is defined by two quantities of the same type, with the
/// lower bound strictly less than the upper bound.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
/// use rquants::core::quantity_range::QuantityRange;
///
/// let range = QuantityRange::new(
///     Length::meters(0.0),
///     Length::meters(100.0)
/// ).unwrap();
///
/// assert!(range.contains(&Length::meters(50.0)));
/// assert!(!range.contains(&Length::meters(150.0)));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct QuantityRange<Q: Quantity> {
    lower: Q,
    upper: Q,
}

impl<Q: Quantity> QuantityRange<Q> {
    /// Creates a new quantity range.
    ///
    /// Returns an error if lower >= upper.
    pub fn new(lower: Q, upper: Q) -> Result<Self, QuantityError> {
        if lower.to_primary() >= upper.to_primary() {
            return Err(QuantityError::RangeError(
                "QuantityRange upper bound must be strictly greater than the lower bound"
                    .to_string(),
            ));
        }
        Ok(Self { lower, upper })
    }

    /// Creates a new quantity range without checking bounds.
    ///
    /// # Safety
    /// Caller must ensure lower < upper.
    pub fn new_unchecked(lower: Q, upper: Q) -> Self {
        Self { lower, upper }
    }

    /// Returns the lower bound of the range.
    pub fn lower(&self) -> Q {
        self.lower
    }

    /// Returns the upper bound of the range.
    pub fn upper(&self) -> Q {
        self.upper
    }

    /// Returns the size of the range (upper - lower) as a quantity.
    pub fn size(&self) -> Q {
        let diff = self.upper.to_primary() - self.lower.to_primary();
        Q::new(self.lower.unit().convert_from_primary(diff), self.lower.unit())
    }

    /// Returns true if the quantity is contained within this range.
    ///
    /// This check is exclusive of the upper limit: lower <= q < upper.
    pub fn contains(&self, q: &Q) -> bool {
        let v = q.to_primary();
        v >= self.lower.to_primary() && v < self.upper.to_primary()
    }

    /// Returns true if the quantity is included within this range.
    ///
    /// This check is inclusive of both limits: lower <= q <= upper.
    pub fn includes(&self, q: &Q) -> bool {
        let v = q.to_primary();
        v >= self.lower.to_primary() && v <= self.upper.to_primary()
    }

    /// Returns true if `that` range is completely contained within `this` range.
    ///
    /// Uses exclusive upper bound semantics.
    pub fn contains_range(&self, that: &QuantityRange<Q>) -> bool {
        that.lower.to_primary() >= self.lower.to_primary()
            && that.lower.to_primary() < self.upper.to_primary()
            && that.upper.to_primary() > self.lower.to_primary()
            && that.upper.to_primary() <= self.upper.to_primary()
    }

    /// Returns true if `that` range is completely included in `this` range.
    ///
    /// Uses inclusive semantics.
    pub fn includes_range(&self, that: &QuantityRange<Q>) -> bool {
        that.lower.to_primary() >= self.lower.to_primary()
            && that.lower.to_primary() <= self.upper.to_primary()
            && that.upper.to_primary() >= self.lower.to_primary()
            && that.upper.to_primary() <= self.upper.to_primary()
    }

    /// Returns true if `that` range overlaps with `this` range.
    pub fn overlaps(&self, that: &QuantityRange<Q>) -> bool {
        that.lower.to_primary() < self.upper.to_primary()
            && that.upper.to_primary() > self.lower.to_primary()
    }

    /// Increments both bounds by the given quantity.
    pub fn shift(&self, amount: Q) -> Self {
        let new_lower = self.lower.to_primary() + amount.to_primary();
        let new_upper = self.upper.to_primary() + amount.to_primary();
        Self {
            lower: Q::new(
                self.lower.unit().convert_from_primary(new_lower),
                self.lower.unit(),
            ),
            upper: Q::new(
                self.upper.unit().convert_from_primary(new_upper),
                self.upper.unit(),
            ),
        }
    }

    /// Increments both bounds by the range size.
    pub fn increment(&self) -> Self {
        self.shift(self.size())
    }

    /// Decrements both bounds by the range size.
    pub fn decrement(&self) -> Self {
        let size = self.size();
        let neg_size = Q::new(-size.value(), size.unit());
        self.shift(neg_size)
    }

    /// Expands the range by the given amount on both sides.
    pub fn expand(&self, amount: Q) -> Self {
        let amount_primary = amount.to_primary();
        let new_lower = self.lower.to_primary() - amount_primary;
        let new_upper = self.upper.to_primary() + amount_primary;
        Self {
            lower: Q::new(
                self.lower.unit().convert_from_primary(new_lower),
                self.lower.unit(),
            ),
            upper: Q::new(
                self.upper.unit().convert_from_primary(new_upper),
                self.upper.unit(),
            ),
        }
    }

    /// Contracts the range by the given amount on both sides.
    ///
    /// Returns None if the contraction would make lower >= upper.
    pub fn contract(&self, amount: Q) -> Option<Self> {
        let amount_primary = amount.to_primary();
        let new_lower = self.lower.to_primary() + amount_primary;
        let new_upper = self.upper.to_primary() - amount_primary;
        if new_lower >= new_upper {
            None
        } else {
            Some(Self {
                lower: Q::new(
                    self.lower.unit().convert_from_primary(new_lower),
                    self.lower.unit(),
                ),
                upper: Q::new(
                    self.upper.unit().convert_from_primary(new_upper),
                    self.upper.unit(),
                ),
            })
        }
    }

    /// Divides the range into n equal parts.
    ///
    /// Returns a Vec of QuantityRanges.
    pub fn divide(&self, n: usize) -> Vec<QuantityRange<Q>> {
        if n == 0 {
            return vec![];
        }
        let step_size = (self.upper.to_primary() - self.lower.to_primary()) / n as f64;
        let unit = self.lower.unit();
        (0..n)
            .map(|i| {
                let start = self.lower.to_primary() + step_size * i as f64;
                let end = self.lower.to_primary() + step_size * (i + 1) as f64;
                QuantityRange {
                    lower: Q::new(unit.convert_from_primary(start), unit),
                    upper: Q::new(unit.convert_from_primary(end), unit),
                }
            })
            .collect()
    }

    /// Returns the range boundaries as a tuple.
    pub fn to_tuple(&self) -> (Q, Q) {
        (self.lower, self.upper)
    }
}

impl<Q: Quantity + fmt::Display> fmt::Display for QuantityRange<Q> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {})", self.lower, self.upper)
    }
}

impl<Q: Quantity> PartialEq for QuantityRange<Q> {
    fn eq(&self, other: &Self) -> bool {
        (self.lower.to_primary() - other.lower.to_primary()).abs() < f64::EPSILON
            && (self.upper.to_primary() - other.upper.to_primary()).abs() < f64::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::space::length::Length;

    #[test]
    fn test_range_creation() {
        let range = QuantityRange::new(Length::meters(0.0), Length::meters(100.0)).unwrap();
        assert_eq!(range.lower().to_meters(), 0.0);
        assert_eq!(range.upper().to_meters(), 100.0);
    }

    #[test]
    fn test_range_invalid() {
        let result = QuantityRange::new(Length::meters(100.0), Length::meters(0.0));
        assert!(result.is_err());

        let result2 = QuantityRange::new(Length::meters(50.0), Length::meters(50.0));
        assert!(result2.is_err());
    }

    #[test]
    fn test_range_size() {
        let range = QuantityRange::new(Length::meters(10.0), Length::meters(50.0)).unwrap();
        assert_eq!(range.size().to_meters(), 40.0);
    }

    #[test]
    fn test_range_contains() {
        let range = QuantityRange::new(Length::meters(0.0), Length::meters(100.0)).unwrap();
        assert!(range.contains(&Length::meters(50.0)));
        assert!(range.contains(&Length::meters(0.0)));
        assert!(!range.contains(&Length::meters(100.0))); // exclusive upper
        assert!(!range.contains(&Length::meters(-10.0)));
    }

    #[test]
    fn test_range_includes() {
        let range = QuantityRange::new(Length::meters(0.0), Length::meters(100.0)).unwrap();
        assert!(range.includes(&Length::meters(100.0))); // inclusive upper
    }

    #[test]
    fn test_range_shift() {
        let range = QuantityRange::new(Length::meters(0.0), Length::meters(100.0)).unwrap();
        let shifted = range.shift(Length::meters(50.0));
        assert_eq!(shifted.lower().to_meters(), 50.0);
        assert_eq!(shifted.upper().to_meters(), 150.0);
    }

    #[test]
    fn test_range_increment() {
        let range = QuantityRange::new(Length::meters(0.0), Length::meters(10.0)).unwrap();
        let next = range.increment();
        assert_eq!(next.lower().to_meters(), 10.0);
        assert_eq!(next.upper().to_meters(), 20.0);
    }

    #[test]
    fn test_range_divide() {
        let range = QuantityRange::new(Length::meters(0.0), Length::meters(100.0)).unwrap();
        let parts = range.divide(4);
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0].lower().to_meters(), 0.0);
        assert_eq!(parts[0].upper().to_meters(), 25.0);
        assert_eq!(parts[3].lower().to_meters(), 75.0);
        assert_eq!(parts[3].upper().to_meters(), 100.0);
    }

    #[test]
    fn test_range_overlaps() {
        let r1 = QuantityRange::new(Length::meters(0.0), Length::meters(50.0)).unwrap();
        let r2 = QuantityRange::new(Length::meters(25.0), Length::meters(75.0)).unwrap();
        let r3 = QuantityRange::new(Length::meters(50.0), Length::meters(100.0)).unwrap();

        assert!(r1.overlaps(&r2));
        assert!(!r1.overlaps(&r3)); // touching but not overlapping
    }

    #[test]
    fn test_range_expand_contract() {
        let range = QuantityRange::new(Length::meters(20.0), Length::meters(80.0)).unwrap();

        let expanded = range.expand(Length::meters(10.0));
        assert_eq!(expanded.lower().to_meters(), 10.0);
        assert_eq!(expanded.upper().to_meters(), 90.0);

        let contracted = range.contract(Length::meters(10.0)).unwrap();
        assert_eq!(contracted.lower().to_meters(), 30.0);
        assert_eq!(contracted.upper().to_meters(), 70.0);

        // Too much contraction
        let over_contracted = range.contract(Length::meters(40.0));
        assert!(over_contracted.is_none());
    }
}
