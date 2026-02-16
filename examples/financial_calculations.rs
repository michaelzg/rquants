//! Financial calculations — shopping, travel money, and pricing anything.
//!
//! Run with: `cargo run --example financial_calculations`

use rquants::market::currency::Currency;
use rquants::market::exchange_rate::CurrencyExchangeRate;
use rquants::market::money::Money;
use rquants::market::price::Price;
use rquants::prelude::*;

fn main() {
    println!("==========================================");
    println!("  RQuants Finance — Money Talks Edition");
    println!("==========================================\n");

    money_basics();
    exchange_rates();
    pricing_anything();
}

/// Money: add, subtract, multiply, like a calculator that knows currencies
fn money_basics() {
    println!("--- Money Basics ---\n");

    // Concert ticket math
    let ticket = Money::usd(95.00);
    let fees = Money::usd(23.50);
    let total = (ticket + fees).unwrap();
    println!("Concert ticket: {} + {} fees = {}", ticket, fees, total);

    // Group dinner: split the bill
    let dinner = Money::usd(186.40);
    let people = 4.0;
    let per_person = dinner / people;
    println!(
        "Dinner for 4: {} / {} = {} each",
        dinner, people, per_person
    );

    // Sale: 30% off a PS5
    let ps5 = Money::usd(499.99);
    let discount = ps5 * 0.30;
    let sale_price = (ps5 - discount).unwrap();
    println!(
        "PS5 on sale: {} - 30% ({}) = {}",
        ps5, discount, sale_price
    );

    // Saving up: $50/week for 52 weeks
    let weekly = Money::usd(50.0);
    let annual = weekly * 52.0;
    println!("Save {} per week = {} per year", weekly, annual);

    println!();
}

/// Exchange rates: converting money between currencies
fn exchange_rates() {
    println!("--- Exchange Rates ---\n");

    // Summer trip to Tokyo: USD to JPY
    let usd_jpy = CurrencyExchangeRate::new(Currency::USD, Currency::JPY, 155.0);
    let budget = Money::usd(2000.0);
    match usd_jpy.convert(budget) {
        Ok(yen) => println!("Tokyo trip: {} = {} (ramen for days)", budget, yen),
        Err(e) => println!("Error: {}", e),
    }

    // Study abroad in London: USD to GBP
    let usd_gbp = CurrencyExchangeRate::new(Currency::USD, Currency::GBP, 0.79);
    let rent = Money::usd(1500.0);
    match usd_gbp.convert(rent) {
        Ok(pounds) => println!("London rent: {} = {}", rent, pounds),
        Err(e) => println!("Error: {}", e),
    }

    // Freelance gig paid in EUR, you need USD
    let eur_usd = CurrencyExchangeRate::new(Currency::EUR, Currency::USD, 1.08);
    let payment = Money::new(3500.0, Currency::EUR);
    match eur_usd.convert(payment) {
        Ok(dollars) => println!("Freelance pay: {} = {}", payment, dollars),
        Err(e) => println!("Error: {}", e),
    }

    // Bitcoin price check
    let btc_usd = CurrencyExchangeRate::new(Currency::BTC, Currency::USD, 100_000.0);
    let holdings = Money::new(0.05, Currency::BTC);
    match btc_usd.convert(holdings) {
        Ok(value) => println!("Crypto portfolio: {} = {}", holdings, value),
        Err(e) => println!("Error: {}", e),
    }

    println!();
}

/// Price<Q>: attach a dollar amount to any physical quantity
fn pricing_anything() {
    println!("--- Pricing Anything ---\n");

    // Electricity bill: $0.15 per kWh
    let rate = Price::new(Money::usd(0.15), Energy::kilowatt_hours(1.0));
    let summer_usage = Energy::kilowatt_hours(1200.0);
    let bill = rate * summer_usage;
    println!(
        "Summer electric bill: {} per kWh x {:.0} kWh = {}",
        rate.money(),
        summer_usage.to_kilowatt_hours(),
        bill
    );

    // Gas station: $3.89 per gallon
    let gas_price = Price::new(Money::usd(3.89), Volume::us_gallons(1.0));
    let fill_up = Volume::us_gallons(14.0);
    let cost = gas_price * fill_up;
    println!(
        "Fill-up: {} per gal x {:.0} gal = {}",
        gas_price.money(),
        fill_up.to_us_gallons(),
        cost
    );

    // Gold: ~$65 per gram
    let gold = Price::new(Money::usd(65.0), Mass::grams(1.0));
    let chain = Mass::grams(30.0);
    let chain_value = gold * chain;
    println!(
        "Gold chain ({:.0}g): {} per gram = {}",
        chain.to_grams(),
        gold.money(),
        chain_value
    );

    // Cloud storage: $0.023 per GB per month
    let cloud = Price::new(Money::usd(0.023), Information::gigabytes(1.0));
    let storage = Information::terabytes(5.0);
    let monthly = cloud * storage;
    println!(
        "5 TB cloud storage: {} per GB = {} per month",
        cloud.money(),
        monthly
    );

    println!("\nAll financial examples done!");
}
