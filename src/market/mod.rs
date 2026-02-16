//! Market module - financial quantities including Money, Currency, and Price.
//!
//! This module provides types for working with money and financial calculations.
//! Unlike other quantities in this library, Money uses a Currency instead of a
//! simple UnitOfMeasure, as exchange rates between currencies are not fixed
//! and must be supplied at runtime.
//!
//! # Examples
//!
//! ## Money Operations
//!
//! ```rust
//! use rquants::prelude::*;
//!
//! let price = Money::usd(100.0);
//! let tax = Money::usd(10.0);
//! let total = (price + tax).unwrap();
//! assert_eq!(total.to_amount(), 110.0);
//! ```
//!
//! ## Currency Exchange
//!
//! ```rust
//! use rquants::prelude::*;
//!
//! // 1 USD = 0.85 EUR
//! let rate = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.85);
//!
//! let dollars = Money::usd(100.0);
//! let euros = rate.convert(dollars).unwrap();
//! assert_eq!(euros.to_amount(), 85.0);
//! ```
//!
//! ## Prices
//!
//! ```rust
//! use rquants::prelude::*;
//!
//! let distance = Length::meters(10.0);
//! let cost = Money::usd(50.0);
//! let price = cost / distance; // Price<Length>
//!
//! let longer_distance = Length::meters(25.0);
//! let total_cost = price * longer_distance;
//! assert_eq!(total_cost.to_amount(), 125.0);
//! ```

pub mod currency;
pub mod exchange_rate;
pub mod money;
pub mod price;

// Re-export main types
pub use currency::Currency;
pub use exchange_rate::CurrencyExchangeRate;
pub use money::{Money, MoneyConversions};
pub use price::Price;
