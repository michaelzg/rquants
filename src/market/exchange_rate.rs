//! Currency exchange rate functionality.

use super::currency::Currency;
use super::money::Money;
use crate::core::error::QuantityError;
use std::fmt;

/// Represents an exchange rate between two currencies.
///
/// An exchange rate defines how to convert money from one currency (base)
/// to another currency (counter).
///
/// # Example
///
/// ```rust
/// use rquants::prelude::*;
///
/// // 1 USD = 0.85 EUR
/// let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.85);
///
/// let dollars = Money::usd(100.0);
/// let euros = rate.convert(dollars).unwrap();
/// assert_eq!(euros.to_amount(), 85.0);
/// assert_eq!(euros.currency(), Currency::EUR);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurrencyExchangeRate {
    base: Currency,
    counter: Currency,
    rate: f64,
}

impl CurrencyExchangeRate {
    /// Creates a new exchange rate, validating input.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - base and counter currencies are the same
    /// - rate is not finite
    /// - rate is not strictly positive
    pub fn try_new(base: Currency, counter: Currency, rate: f64) -> Result<Self, QuantityError> {
        if base == counter {
            return Err(QuantityError::UnsupportedOperation(
                "Cannot create exchange rate with the same base and counter currency".to_string(),
            ));
        }
        if !rate.is_finite() {
            return Err(QuantityError::ConversionError(
                "Exchange rate must be finite".to_string(),
            ));
        }
        if rate <= 0.0 {
            return Err(QuantityError::ConversionError(
                "Exchange rate must be greater than zero".to_string(),
            ));
        }
        Ok(Self {
            base,
            counter,
            rate,
        })
    }

    /// Creates a new exchange rate.
    ///
    /// # Arguments
    ///
    /// * `base` - The base currency (e.g., USD)
    /// * `counter` - The counter currency (e.g., EUR)
    /// * `rate` - The exchange rate (e.g., 0.85 means 1 USD = 0.85 EUR)
    ///
    /// # Panics
    ///
    /// Panics if base and counter currencies are the same or if rate is invalid.
    pub fn new(base: Currency, counter: Currency, rate: f64) -> Self {
        Self::try_new(base, counter, rate)
            .unwrap_or_else(|e| panic!("Invalid exchange rate: {e}"))
    }

    /// Returns the base currency.
    pub fn base(&self) -> Currency {
        self.base
    }

    /// Returns the counter currency.
    pub fn counter(&self) -> Currency {
        self.counter
    }

    /// Returns the exchange rate.
    pub fn rate(&self) -> f64 {
        self.rate
    }

    /// Converts money from one currency to another using this exchange rate.
    ///
    /// This method can convert:
    /// - From base to counter currency
    /// - From counter to base currency
    ///
    /// # Arguments
    ///
    /// * `money` - The money to convert
    ///
    /// # Returns
    ///
    /// Returns the converted money in the target currency, or an error if the
    /// money's currency doesn't match either the base or counter currency.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.85);
    ///
    /// // Convert from base to counter
    /// let usd = Money::usd(100.0);
    /// let eur = rate.convert(usd).unwrap();
    /// assert_eq!(eur.to_amount(), 85.0);
    ///
    /// // Convert from counter to base
    /// let eur2 = Money::eur(85.0);
    /// let usd2 = rate.convert(eur2).unwrap();
    /// assert!((usd2.to_amount() - 100.0).abs() < 1e-10);
    /// ```
    pub fn convert(&self, money: Money) -> Result<Money, QuantityError> {
        if money.currency() == self.base {
            // Convert from base to counter
            Ok(Money::new(money.to_amount() * self.rate, self.counter))
        } else if money.currency() == self.counter {
            // Convert from counter to base
            Ok(Money::new(money.to_amount() / self.rate, self.base))
        } else {
            Err(QuantityError::ConversionError(format!(
                "Money currency {} does not match exchange rate currencies {} and {}",
                money.currency().code(),
                self.base.code(),
                self.counter.code()
            )))
        }
    }

    /// Returns the inverse exchange rate.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rquants::prelude::*;
    ///
    /// let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.85);
    /// let inverse = rate.inverse();
    ///
    /// assert_eq!(inverse.base(), Currency::EUR);
    /// assert_eq!(inverse.counter(), Currency::USD);
    /// assert!((inverse.rate() - 1.0 / 0.85).abs() < 1e-10);
    /// ```
    pub fn inverse(&self) -> Self {
        Self {
            base: self.counter,
            counter: self.base,
            rate: 1.0 / self.rate,
        }
    }
}

impl fmt::Display for CurrencyExchangeRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{} {}",
            self.base.code(),
            self.counter.code(),
            self.rate
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_rate_creation() {
        let rate = CurrencyExchangeRate::try_new(Currency::USD, Currency::EUR, 0.85).unwrap();
        assert_eq!(rate.base(), Currency::USD);
        assert_eq!(rate.counter(), Currency::EUR);
        assert_eq!(rate.rate(), 0.85);
    }

    #[test]
    fn test_exchange_rate_try_new_validates_input() {
        assert!(CurrencyExchangeRate::try_new(Currency::USD, Currency::USD, 1.0).is_err());
        assert!(CurrencyExchangeRate::try_new(Currency::USD, Currency::EUR, 0.0).is_err());
        assert!(CurrencyExchangeRate::try_new(Currency::USD, Currency::EUR, -1.0).is_err());
        assert!(CurrencyExchangeRate::try_new(Currency::USD, Currency::EUR, f64::INFINITY).is_err());
        assert!(CurrencyExchangeRate::try_new(Currency::USD, Currency::EUR, f64::NAN).is_err());
    }

    #[test]
    #[should_panic(expected = "Invalid exchange rate: Unsupported operation: Cannot create exchange rate with the same base and counter currency")]
    fn test_exchange_rate_same_currency() {
        CurrencyExchangeRate::new(Currency::USD, Currency::USD, 1.0);
    }

    #[test]
    fn test_exchange_rate_convert_base_to_counter() {
        let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.85);
        let usd = Money::usd(100.0);
        let eur = rate.convert(usd).unwrap();

        assert_eq!(eur.currency(), Currency::EUR);
        assert_eq!(eur.to_amount(), 85.0);
    }

    #[test]
    fn test_exchange_rate_convert_counter_to_base() {
        let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.85);
        let eur = Money::eur(85.0);
        let usd = rate.convert(eur).unwrap();

        assert_eq!(usd.currency(), Currency::USD);
        assert!((usd.to_amount() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_exchange_rate_convert_invalid_currency() {
        let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.85);
        let gbp = Money::gbp(100.0);
        let result = rate.convert(gbp);

        assert!(result.is_err());
        match result {
            Err(QuantityError::ConversionError(msg)) => {
                assert!(msg.contains("does not match"));
            }
            _ => panic!("Expected ConversionError"),
        }
    }

    #[test]
    fn test_exchange_rate_inverse() {
        let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.85);
        let inverse = rate.inverse();

        assert_eq!(inverse.base(), Currency::EUR);
        assert_eq!(inverse.counter(), Currency::USD);
        assert!((inverse.rate() - 1.0 / 0.85).abs() < 1e-10);

        // Test round-trip conversion
        let usd = Money::usd(100.0);
        let eur = rate.convert(usd).unwrap();
        let usd2 = inverse.convert(eur).unwrap();

        assert_eq!(usd2.currency(), Currency::USD);
        assert!((usd2.to_amount() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_exchange_rate_display() {
        let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.85);
        assert_eq!(format!("{}", rate), "USD/EUR 0.85");
    }

    #[test]
    fn test_exchange_rate_roundtrip() {
        let rate = CurrencyExchangeRate::new(Currency::USD, Currency::JPY, 110.0);

        let original = Money::usd(100.0);
        let converted = rate.convert(original).unwrap();
        let back = rate.convert(converted).unwrap();

        assert_eq!(back.currency(), Currency::USD);
        assert!((back.to_amount() - original.to_amount()).abs() < 1e-10);
    }
}
