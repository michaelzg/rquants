//! Price functionality - represents money per unit of quantity.

use super::money::Money;
use crate::core::Quantity;
use std::fmt;
use std::ops::{Div, Mul};

/// Represents a price - the ratio of Money to some Quantity.
///
/// A Price is generic over any quantity type Q and represents
/// how much money is required per unit of that quantity.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let price = Price::new(Money::usd(10.0), Length::meters(1.0));
/// let cost = price.clone() * Length::meters(5.0);
/// assert_eq!(cost.to_amount(), 50.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Price<Q: Quantity> {
    money: Money,
    quantity: Q,
}

impl<Q: Quantity + Mul<f64, Output = Q>> Price<Q> {
    /// Creates a new Price.
    ///
    /// # Arguments
    ///
    /// * `money` - The money amount
    /// * `quantity` - The quantity amount
    pub fn new(money: Money, quantity: Q) -> Self {
        Self { money, quantity }
    }

    /// Returns the money component of this price.
    pub fn money(&self) -> Money {
        self.money
    }

    /// Returns the quantity component of this price.
    pub fn quantity(&self) -> Q {
        self.quantity
    }

    /// Returns the price per unit (money amount / quantity amount).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let price = Price::new(Money::usd(10.0), Length::meters(2.0));
    /// assert_eq!(price.per_unit_amount(), 5.0);
    /// ```
    pub fn per_unit_amount(&self) -> f64 {
        self.money.to_amount() / self.quantity.value()
    }

    /// Calculates how much quantity can be purchased with the given money.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let price = Price::new(Money::usd(10.0), Length::meters(1.0));
    /// let budget = Money::usd(25.0);
    /// let qty = price.in_currency(budget);
    /// assert_eq!(qty.value(), 2.5);
    /// ```
    pub fn in_currency(&self, money: Money) -> Q {
        let ratio = money.to_amount() / self.money.to_amount();
        self.quantity * ratio
    }
}

impl<Q: Quantity + Mul<f64, Output = Q>> PartialEq for Price<Q> {
    fn eq(&self, other: &Self) -> bool {
        self.money == other.money && (self.quantity.value() - other.quantity.value()).abs() < f64::EPSILON
    }
}

impl<Q: Quantity + Mul<f64, Output = Q>> fmt::Display for Price<Q> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.money, self.quantity)
    }
}

// Price * Quantity = Money
impl<Q: Quantity + Mul<f64, Output = Q>> Mul<Q> for Price<Q> {
    type Output = Money;

    fn mul(self, rhs: Q) -> Self::Output {
        // Use primary units for accurate comparison
        let ratio = rhs.to_primary() / self.quantity.to_primary();
        self.money * ratio
    }
}

// Price * f64 = Price (scale the money)
impl<Q: Quantity + Mul<f64, Output = Q>> Mul<f64> for Price<Q> {
    type Output = Price<Q>;

    fn mul(self, rhs: f64) -> Self::Output {
        Price::new(self.money * rhs, self.quantity)
    }
}

// f64 * Price = Price
impl<Q: Quantity + Mul<f64, Output = Q>> Mul<Price<Q>> for f64 {
    type Output = Price<Q>;

    fn mul(self, rhs: Price<Q>) -> Self::Output {
        Price::new(rhs.money * self, rhs.quantity)
    }
}

// Price / f64 = Price (scale the money)
impl<Q: Quantity + Mul<f64, Output = Q>> Div<f64> for Price<Q> {
    type Output = Price<Q>;

    fn div(self, rhs: f64) -> Self::Output {
        Price::new(self.money / rhs, self.quantity)
    }
}

// Money / Quantity = Price
impl<Q: Quantity + Mul<f64, Output = Q>> Div<Q> for Money {
    type Output = Price<Q>;

    fn div(self, rhs: Q) -> Self::Output {
        Price::new(self, rhs)
    }
}

// Money / Price = Quantity
impl<Q: Quantity + Mul<f64, Output = Q>> Div<Price<Q>> for Money {
    type Output = Q;

    fn div(self, rhs: Price<Q>) -> Self::Output {
        let ratio = self.to_amount() / rhs.money.to_amount();
        rhs.quantity * ratio
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::space::Length;

    #[test]
    fn test_price_creation() {
        let price = Price::new(Money::usd(10.0), Length::meters(2.0));
        assert_eq!(price.money().to_amount(), 10.0);
        assert_eq!(price.quantity().value(), 2.0);
    }

    #[test]
    fn test_price_per_unit_amount() {
        let price = Price::new(Money::usd(10.0), Length::meters(2.0));
        assert_eq!(price.per_unit_amount(), 5.0);
    }

    #[test]
    fn test_price_in_currency() {
        let price = Price::new(Money::usd(10.0), Length::meters(1.0));
        let budget = Money::usd(25.0);
        let qty = price.in_currency(budget);
        assert_eq!(qty.value(), 2.5);
    }

    #[test]
    fn test_price_times_quantity() {
        let price = Price::new(Money::usd(10.0), Length::meters(1.0));
        let qty = Length::meters(5.0);
        let cost = price * qty;
        assert_eq!(cost.to_amount(), 50.0);
        assert_eq!(cost.currency(), crate::market::Currency::USD);
    }

    #[test]
    fn test_price_times_scalar() {
        let price = Price::new(Money::usd(10.0), Length::meters(1.0));
        let scaled = price * 2.0;
        assert_eq!(scaled.money().to_amount(), 20.0);
        assert_eq!(scaled.quantity().value(), 1.0);
    }

    #[test]
    fn test_price_div_scalar() {
        let price = Price::new(Money::usd(10.0), Length::meters(1.0));
        let scaled = price / 2.0;
        assert_eq!(scaled.money().to_amount(), 5.0);
        assert_eq!(scaled.quantity().value(), 1.0);
    }

    #[test]
    fn test_money_div_quantity() {
        let money = Money::usd(10.0);
        let qty = Length::meters(2.0);
        let price = money / qty;
        assert_eq!(price.money().to_amount(), 10.0);
        assert_eq!(price.quantity().value(), 2.0);
    }

    #[test]
    fn test_money_div_price() {
        let price = Price::new(Money::usd(10.0), Length::meters(1.0));
        let budget = Money::usd(25.0);
        let qty = budget / price;
        assert_eq!(qty.value(), 2.5);
    }

    #[test]
    fn test_price_display() {
        let price = Price::new(Money::usd(10.0), Length::meters(1.0));
        let display = format!("{}", price);
        assert!(display.contains("10"));
        assert!(display.contains("USD"));
        assert!(display.contains("1"));
        assert!(display.contains("m"));
    }

    #[test]
    fn test_price_equality() {
        let price1 = Price::new(Money::usd(10.0), Length::meters(1.0));
        let price2 = Price::new(Money::usd(10.0), Length::meters(1.0));
        let price3 = Price::new(Money::usd(20.0), Length::meters(1.0));

        assert_eq!(price1, price2);
        assert_ne!(price1, price3);
    }

    #[test]
    fn test_price_with_different_units() {
        use crate::energy::Energy;

        let price = Price::new(Money::usd(100.0), Energy::joules(1000.0));
        let energy = Energy::kilojoules(5.0);
        let cost = price * energy;

        // 5 kJ = 5000 J
        // Price is $100 / 1000 J = $0.1 / J
        // 5000 J * $0.1/J = $500
        assert_eq!(cost.to_amount(), 500.0);
    }
}
