//! Currency definitions for money quantities.

use std::fmt;

/// Currency represents a unit of money.
///
/// Each currency has a code (e.g., "USD"), name (e.g., "US Dollar"),
/// symbol (e.g., "$"), and the number of decimal places for formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    /// United States Dollar
    USD,
    /// Euro
    EUR,
    /// British Pound Sterling
    GBP,
    /// Japanese Yen
    JPY,
    /// Swiss Franc
    CHF,
    /// Canadian Dollar
    CAD,
    /// Australian Dollar
    AUD,
    /// Chinese Yuan Renminbi
    CNY,
    /// Indian Rupee
    INR,
    /// Bitcoin
    BTC,
}

impl Currency {
    /// Returns the currency code (e.g., "USD").
    pub fn code(&self) -> &'static str {
        match self {
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
            Currency::CHF => "CHF",
            Currency::CAD => "CAD",
            Currency::AUD => "AUD",
            Currency::CNY => "CNY",
            Currency::INR => "INR",
            Currency::BTC => "BTC",
        }
    }

    /// Returns the full currency name (e.g., "US Dollar").
    pub fn name(&self) -> &'static str {
        match self {
            Currency::USD => "US Dollar",
            Currency::EUR => "Euro",
            Currency::GBP => "British Pound Sterling",
            Currency::JPY => "Japanese Yen",
            Currency::CHF => "Swiss Franc",
            Currency::CAD => "Canadian Dollar",
            Currency::AUD => "Australian Dollar",
            Currency::CNY => "Chinese Yuan Renminbi",
            Currency::INR => "Indian Rupee",
            Currency::BTC => "Bitcoin",
        }
    }

    /// Returns the currency symbol (e.g., "$").
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::USD => "$",
            Currency::EUR => "€",
            Currency::GBP => "£",
            Currency::JPY => "¥",
            Currency::CHF => "CHF",
            Currency::CAD => "C$",
            Currency::AUD => "A$",
            Currency::CNY => "¥",
            Currency::INR => "₹",
            Currency::BTC => "₿",
        }
    }

    /// Returns the number of decimal places used for formatting (e.g., 2 for USD, 0 for JPY).
    pub fn format_decimals(&self) -> u8 {
        match self {
            Currency::USD => 2,
            Currency::EUR => 2,
            Currency::GBP => 2,
            Currency::JPY => 0,
            Currency::CHF => 2,
            Currency::CAD => 2,
            Currency::AUD => 2,
            Currency::CNY => 2,
            Currency::INR => 2,
            Currency::BTC => 8,
        }
    }

    /// All available currencies.
    pub const ALL: &'static [Currency] = &[
        Currency::USD,
        Currency::EUR,
        Currency::GBP,
        Currency::JPY,
        Currency::CHF,
        Currency::CAD,
        Currency::AUD,
        Currency::CNY,
        Currency::INR,
        Currency::BTC,
    ];

    /// Attempts to parse a currency from a code string.
    pub fn from_code(code: &str) -> Option<Currency> {
        match code {
            "USD" => Some(Currency::USD),
            "EUR" => Some(Currency::EUR),
            "GBP" => Some(Currency::GBP),
            "JPY" => Some(Currency::JPY),
            "CHF" => Some(Currency::CHF),
            "CAD" => Some(Currency::CAD),
            "AUD" => Some(Currency::AUD),
            "CNY" => Some(Currency::CNY),
            "INR" => Some(Currency::INR),
            "BTC" => Some(Currency::BTC),
            _ => None,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_properties() {
        assert_eq!(Currency::USD.code(), "USD");
        assert_eq!(Currency::USD.name(), "US Dollar");
        assert_eq!(Currency::USD.symbol(), "$");
        assert_eq!(Currency::USD.format_decimals(), 2);

        assert_eq!(Currency::JPY.format_decimals(), 0);
        assert_eq!(Currency::BTC.format_decimals(), 8);
    }

    #[test]
    fn test_currency_from_code() {
        assert_eq!(Currency::from_code("USD"), Some(Currency::USD));
        assert_eq!(Currency::from_code("EUR"), Some(Currency::EUR));
        assert_eq!(Currency::from_code("INVALID"), None);
    }

    #[test]
    fn test_currency_display() {
        assert_eq!(format!("{}", Currency::USD), "USD");
        assert_eq!(format!("{}", Currency::EUR), "EUR");
    }
}
