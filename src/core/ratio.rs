//! Ratio types for relationships between quantities.
//!
//! Provides traits and types for representing ratios between quantities,
//! enabling conversions between related quantities.

use crate::core::{Quantity, UnitOfMeasure};
use std::marker::PhantomData;

/// A ratio between two quantities of potentially different types.
///
/// This trait defines the relationship between a "base" quantity and a
/// "counter" quantity, enabling conversions between them.
///
/// # Example
///
/// A price ratio could relate Money to Volume (e.g., $3.50 per gallon),
/// allowing conversion between dollars and gallons.
pub trait Ratio<A: Quantity, B: Quantity> {
    /// The base quantity of the ratio.
    fn base(&self) -> A;

    /// The counter quantity of the ratio.
    fn counter(&self) -> B;

    /// Converts a quantity of type B to type A using this ratio.
    ///
    /// If the ratio is base:counter = a:b, then converting quantity q of type B
    /// returns (q / b) * a.
    fn convert_to_base(&self, q: B) -> A {
        let ratio = q.to_primary() / self.counter().to_primary();
        let result = self.base().to_primary() * ratio;
        A::new(
            self.base().unit().convert_from_primary(result),
            self.base().unit(),
        )
    }

    /// Converts a quantity of type A to type B using this ratio.
    ///
    /// If the ratio is base:counter = a:b, then converting quantity q of type A
    /// returns (q / a) * b.
    fn convert_to_counter(&self, q: A) -> B {
        let ratio = q.to_primary() / self.base().to_primary();
        let result = self.counter().to_primary() * ratio;
        B::new(
            self.counter().unit().convert_from_primary(result),
            self.counter().unit(),
        )
    }
}

/// A ratio between two quantities of the same type.
///
/// This is useful for representing dimensionless ratios like scale factors,
/// efficiency percentages, or comparison ratios.
pub trait LikeRatio<A: Quantity>: Ratio<A, A> {
    /// Returns the ratio as a dimensionless number (base / counter).
    fn ratio(&self) -> f64 {
        self.base().to_primary() / self.counter().to_primary()
    }

    /// Returns the inverse ratio (counter / base).
    fn inverse_ratio(&self) -> f64 {
        self.counter().to_primary() / self.base().to_primary()
    }
}

/// A concrete implementation of a ratio between two quantities.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
/// use rquants::core::ratio::{QuantityRatio, Ratio};
///
/// // Define a ratio: 1 kilogram per liter (water density at ~4°C)
/// let density_ratio = QuantityRatio::new(
///     Mass::kilograms(1.0),
///     Volume::liters(1.0)
/// );
///
/// // How much mass for 5 liters?
/// let volume = Volume::liters(5.0);
/// let mass = density_ratio.convert_to_base(volume);
/// assert!((mass.to_kilograms() - 5.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct QuantityRatio<A: Quantity, B: Quantity> {
    base: A,
    counter: B,
}

impl<A: Quantity, B: Quantity> QuantityRatio<A, B> {
    /// Creates a new ratio between two quantities.
    pub fn new(base: A, counter: B) -> Self {
        Self { base, counter }
    }

    /// Returns the inverse ratio (counter:base instead of base:counter).
    pub fn inverse(&self) -> QuantityRatio<B, A> {
        QuantityRatio {
            base: self.counter,
            counter: self.base,
        }
    }
}

impl<A: Quantity, B: Quantity> Ratio<A, B> for QuantityRatio<A, B> {
    fn base(&self) -> A {
        self.base
    }

    fn counter(&self) -> B {
        self.counter
    }
}

/// A concrete implementation of a like ratio (same quantity type).
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
/// use rquants::core::ratio::{LikeQuantityRatio, LikeRatio};
///
/// // Define a ratio comparing two masses
/// let ratio = LikeQuantityRatio::new(
///     Mass::kilograms(10.0),
///     Mass::kilograms(2.0)
/// );
///
/// // The ratio is 10/2 = 5
/// assert!((ratio.ratio() - 5.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct LikeQuantityRatio<A: Quantity> {
    base: A,
    counter: A,
}

impl<A: Quantity> LikeQuantityRatio<A> {
    /// Creates a new like ratio between two quantities of the same type.
    pub fn new(base: A, counter: A) -> Self {
        Self { base, counter }
    }

    /// Returns the inverse ratio.
    pub fn inverse(&self) -> Self {
        Self {
            base: self.counter,
            counter: self.base,
        }
    }
}

impl<A: Quantity> Ratio<A, A> for LikeQuantityRatio<A> {
    fn base(&self) -> A {
        self.base
    }

    fn counter(&self) -> A {
        self.counter
    }
}

impl<A: Quantity> LikeRatio<A> for LikeQuantityRatio<A> {}

/// A rate representing one quantity per another (e.g., velocity = length/time).
///
/// This is similar to Ratio but emphasizes the "per" relationship and is
/// typically used for derived quantities.
#[derive(Debug, Clone, Copy)]
pub struct Rate<N: Quantity, D: Quantity> {
    numerator: N,
    denominator: D,
    _marker: PhantomData<(N, D)>,
}

impl<N: Quantity, D: Quantity> Rate<N, D> {
    /// Creates a new rate (numerator per denominator).
    pub fn new(numerator: N, denominator: D) -> Self {
        Self {
            numerator,
            denominator,
            _marker: PhantomData,
        }
    }

    /// Returns the numerator quantity.
    pub fn numerator(&self) -> N {
        self.numerator
    }

    /// Returns the denominator quantity.
    pub fn denominator(&self) -> D {
        self.denominator
    }

    /// Returns the rate as a raw number (numerator_primary / denominator_primary).
    pub fn value(&self) -> f64 {
        self.numerator.to_primary() / self.denominator.to_primary()
    }

    /// Multiplies the rate by a denominator quantity to get a numerator quantity.
    ///
    /// (N / D) * D = N
    pub fn times(&self, d: D) -> N {
        let ratio = d.to_primary() / self.denominator.to_primary();
        let result = self.numerator.to_primary() * ratio;
        N::new(
            self.numerator.unit().convert_from_primary(result),
            self.numerator.unit(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mass::Mass;
    use crate::space::{Length, Volume};

    #[test]
    fn test_quantity_ratio() {
        // 1 kg per liter
        let ratio = QuantityRatio::new(Mass::kilograms(1.0), Volume::liters(1.0));

        // 5 liters should give 5 kg
        let mass = ratio.convert_to_base(Volume::liters(5.0));
        assert!((mass.to_kilograms() - 5.0).abs() < 1e-10);

        // 3 kg should give 3 liters
        let volume = ratio.convert_to_counter(Mass::kilograms(3.0));
        assert!((volume.to_liters() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_like_quantity_ratio() {
        let ratio = LikeQuantityRatio::new(Length::meters(100.0), Length::meters(25.0));

        assert!((ratio.ratio() - 4.0).abs() < 1e-10);
        assert!((ratio.inverse_ratio() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn test_ratio_inverse() {
        let ratio = QuantityRatio::new(Mass::kilograms(1.0), Volume::liters(1.0));
        let inverse = ratio.inverse();

        // Convert 5 kg to volume using inverse
        let volume = inverse.convert_to_base(Mass::kilograms(5.0));
        assert!((volume.to_liters() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_rate() {
        // 10 meters per second
        use crate::time::Time;
        let rate = Rate::new(Length::meters(10.0), Time::seconds(1.0));

        assert_eq!(rate.value(), 10.0);

        // In 5 seconds, travel 50 meters
        let distance = rate.times(Time::seconds(5.0));
        assert!((distance.to_meters() - 50.0).abs() < 1e-10);
    }
}
