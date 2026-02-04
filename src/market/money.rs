//! Money quantity and operations.

use super::currency::Currency;
use crate::core::error::QuantityError;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Represents a quantity of money in a specific currency.
///
/// Money is unique among quantities in that it uses a Currency instead of
/// a simple UnitOfMeasure. This is because exchange rates between currencies
/// are not fixed and must be supplied at runtime.
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// let price = Money::usd(100.0);
/// let tax = Money::usd(10.0);
/// let total = price + tax;
/// assert_eq!(total.to_amount(), 110.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Money {
    amount: f64,
    currency: Currency,
}

impl Money {
    /// Creates a new Money quantity.
    pub fn new(amount: f64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    /// Returns the amount of money.
    pub fn to_amount(&self) -> f64 {
        self.amount
    }

    /// Returns the currency of this money.
    pub fn currency(&self) -> Currency {
        self.currency
    }

    // Constructors for common currencies

    /// Creates a Money in US Dollars.
    pub fn usd(amount: f64) -> Self {
        Self::new(amount, Currency::USD)
    }

    /// Creates a Money in Euros.
    pub fn eur(amount: f64) -> Self {
        Self::new(amount, Currency::EUR)
    }

    /// Creates a Money in British Pounds.
    pub fn gbp(amount: f64) -> Self {
        Self::new(amount, Currency::GBP)
    }

    /// Creates a Money in Japanese Yen.
    pub fn jpy(amount: f64) -> Self {
        Self::new(amount, Currency::JPY)
    }

    /// Creates a Money in Swiss Francs.
    pub fn chf(amount: f64) -> Self {
        Self::new(amount, Currency::CHF)
    }

    /// Creates a Money in Canadian Dollars.
    pub fn cad(amount: f64) -> Self {
        Self::new(amount, Currency::CAD)
    }

    /// Creates a Money in Australian Dollars.
    pub fn aud(amount: f64) -> Self {
        Self::new(amount, Currency::AUD)
    }

    /// Creates a Money in Chinese Yuan.
    pub fn cny(amount: f64) -> Self {
        Self::new(amount, Currency::CNY)
    }

    /// Creates a Money in Indian Rupees.
    pub fn inr(amount: f64) -> Self {
        Self::new(amount, Currency::INR)
    }

    /// Creates a Money in Bitcoin.
    pub fn btc(amount: f64) -> Self {
        Self::new(amount, Currency::BTC)
    }

    /// Returns a formatted string with the currency symbol.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let money = Money::usd(123.456);
    /// assert_eq!(money.to_formatted_string(), "$123.46");
    /// ```
    pub fn to_formatted_string(&self) -> String {
        let decimals = self.currency.format_decimals();
        format!(
            "{}{:.decimals$}",
            self.currency.symbol(),
            self.amount,
            decimals = decimals as usize
        )
    }

    /// Checks if this money can be operated with another (same currency).
    fn can_operate_with(&self, other: &Money) -> Result<(), QuantityError> {
        if self.currency == other.currency {
            Ok(())
        } else {
            Err(QuantityError::UnsupportedOperation(format!(
                "Cannot operate on different currencies: {} and {}",
                self.currency.code(),
                other.currency.code()
            )))
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.currency.code())
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency && (self.amount - other.amount).abs() < f64::EPSILON
    }
}

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.currency != other.currency {
            None
        } else {
            self.amount.partial_cmp(&other.amount)
        }
    }
}

// Arithmetic operations - only work on same currency

impl Add for Money {
    type Output = Money;

    fn add(self, rhs: Self) -> Self::Output {
        self.can_operate_with(&rhs).unwrap();
        Money::new(self.amount + rhs.amount, self.currency)
    }
}

impl Sub for Money {
    type Output = Money;

    fn sub(self, rhs: Self) -> Self::Output {
        self.can_operate_with(&rhs).unwrap();
        Money::new(self.amount - rhs.amount, self.currency)
    }
}

impl Mul<f64> for Money {
    type Output = Money;

    fn mul(self, rhs: f64) -> Self::Output {
        Money::new(self.amount * rhs, self.currency)
    }
}

impl Mul<Money> for f64 {
    type Output = Money;

    fn mul(self, rhs: Money) -> Self::Output {
        Money::new(self * rhs.amount, rhs.currency)
    }
}

impl Div<f64> for Money {
    type Output = Money;

    fn div(self, rhs: f64) -> Self::Output {
        Money::new(self.amount / rhs, self.currency)
    }
}

impl Div<Money> for Money {
    type Output = f64;

    fn div(self, rhs: Money) -> Self::Output {
        self.can_operate_with(&rhs).unwrap();
        self.amount / rhs.amount
    }
}

impl Neg for Money {
    type Output = Money;

    fn neg(self) -> Self::Output {
        Money::new(-self.amount, self.currency)
    }
}

/// Extension trait for creating Money quantities from numeric types.
pub trait MoneyConversions {
    /// Creates Money in US Dollars.
    fn usd(self) -> Money;
    /// Creates Money in Euros.
    fn eur(self) -> Money;
    /// Creates Money in British Pounds.
    fn gbp(self) -> Money;
    /// Creates Money in Japanese Yen.
    fn jpy(self) -> Money;
    /// Creates Money in Swiss Francs.
    fn chf(self) -> Money;
    /// Creates Money in Canadian Dollars.
    fn cad(self) -> Money;
    /// Creates Money in Australian Dollars.
    fn aud(self) -> Money;
    /// Creates Money in Chinese Yuan.
    fn cny(self) -> Money;
    /// Creates Money in Indian Rupees.
    fn inr(self) -> Money;
    /// Creates Money in Bitcoin.
    fn btc(self) -> Money;
}

impl MoneyConversions for f64 {
    fn usd(self) -> Money {
        Money::usd(self)
    }
    fn eur(self) -> Money {
        Money::eur(self)
    }
    fn gbp(self) -> Money {
        Money::gbp(self)
    }
    fn jpy(self) -> Money {
        Money::jpy(self)
    }
    fn chf(self) -> Money {
        Money::chf(self)
    }
    fn cad(self) -> Money {
        Money::cad(self)
    }
    fn aud(self) -> Money {
        Money::aud(self)
    }
    fn cny(self) -> Money {
        Money::cny(self)
    }
    fn inr(self) -> Money {
        Money::inr(self)
    }
    fn btc(self) -> Money {
        Money::btc(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_creation() {
        let m = Money::usd(100.0);
        assert_eq!(m.to_amount(), 100.0);
        assert_eq!(m.currency(), Currency::USD);
    }

    #[test]
    fn test_money_display() {
        let m = Money::usd(100.50);
        assert_eq!(format!("{}", m), "100.5 USD");
    }

    #[test]
    fn test_money_formatted_string() {
        let m = Money::usd(123.456);
        assert_eq!(m.to_formatted_string(), "$123.46");

        let yen = Money::jpy(1234.567);
        assert_eq!(yen.to_formatted_string(), "¥1235");
    }

    #[test]
    fn test_money_arithmetic_same_currency() {
        let m1 = Money::usd(100.0);
        let m2 = Money::usd(50.0);

        let sum = m1 + m2;
        assert_eq!(sum.to_amount(), 150.0);

        let diff = m1 - m2;
        assert_eq!(diff.to_amount(), 50.0);

        let product = m1 * 2.0;
        assert_eq!(product.to_amount(), 200.0);

        let quotient = m1 / 2.0;
        assert_eq!(quotient.to_amount(), 50.0);

        let ratio = m1 / m2;
        assert_eq!(ratio, 2.0);
    }

    #[test]
    #[should_panic(expected = "Cannot operate on different currencies")]
    fn test_money_arithmetic_different_currency_add() {
        let m1 = Money::usd(100.0);
        let m2 = Money::eur(50.0);
        let _ = m1 + m2; // Should panic
    }

    #[test]
    #[should_panic(expected = "Cannot operate on different currencies")]
    fn test_money_arithmetic_different_currency_div() {
        let m1 = Money::usd(100.0);
        let m2 = Money::eur(50.0);
        let _ = m1 / m2; // Should panic
    }

    #[test]
    fn test_money_conversions_trait() {
        let m1 = 100.0.usd();
        assert_eq!(m1.to_amount(), 100.0);
        assert_eq!(m1.currency(), Currency::USD);

        let m2 = 50.0.eur();
        assert_eq!(m2.to_amount(), 50.0);
        assert_eq!(m2.currency(), Currency::EUR);
    }

    #[test]
    fn test_money_equality() {
        let m1 = Money::usd(100.0);
        let m2 = Money::usd(100.0);
        let m3 = Money::usd(50.0);
        let m4 = Money::eur(100.0);

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
        assert_ne!(m1, m4);
    }

    #[test]
    fn test_money_comparison() {
        let m1 = Money::usd(100.0);
        let m2 = Money::usd(50.0);
        let m3 = Money::eur(100.0);

        assert!(m1 > m2);
        assert!(m2 < m1);
        assert_eq!(m1.partial_cmp(&m3), None); // Different currencies
    }

    #[test]
    fn test_money_negation() {
        let m = Money::usd(100.0);
        let neg = -m;
        assert_eq!(neg.to_amount(), -100.0);
    }
}
