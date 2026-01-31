//! Financial calculation examples using the market module.

use rquants::market::currency::Currency;
use rquants::market::exchange_rate::CurrencyExchangeRate;
use rquants::market::money::Money;
use rquants::market::price::Price;
use rquants::prelude::*;

fn main() {
    println!("=== Money Basics ===");
    money_basics();

    println!("\n=== Exchange Rates ===");
    exchange_rates();

    println!("\n=== Pricing ===");
    pricing();
}

fn money_basics() {
    let price = Money::usd(29.99);
    let quantity = 3.0;
    let total = price * quantity;
    println!("Item: {}, Qty: {}, Total: {}", price, quantity, total);

    let subtotal = Money::usd(100.0);
    let discount = Money::usd(15.0);
    let final_price = subtotal - discount;
    println!("Subtotal: {}, Discount: {}, Final: {}", subtotal, discount, final_price);
}

fn exchange_rates() {
    // USD to EUR
    let usd_eur = CurrencyExchangeRate::new(Currency::USD, Currency::EUR, 0.92);
    let dollars = Money::usd(1000.0);
    match usd_eur.convert(dollars) {
        Ok(euros) => println!("{} = {}", dollars, euros),
        Err(e) => println!("Error: {}", e),
    }

    // JPY to USD
    let jpy_usd = CurrencyExchangeRate::new(Currency::JPY, Currency::USD, 0.0067);
    let yen = Money::new(10000.0, Currency::JPY);
    match jpy_usd.convert(yen) {
        Ok(dollars) => println!("{} = {}", yen, dollars),
        Err(e) => println!("Error: {}", e),
    }

    // Reverse conversion
    let usd_amount = Money::usd(100.0);
    match usd_eur.convert(usd_amount) {
        Ok(eur) => {
            println!("{} = {}", usd_amount, eur);
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn pricing() {
    // Price per unit of energy
    let electricity_cost = Money::usd(0.12);
    let per_kwh = Energy::kilowatt_hours(1.0);
    let price = Price::new(electricity_cost, per_kwh);

    let monthly_usage = Energy::kilowatt_hours(900.0);
    let monthly_bill = price * monthly_usage;
    println!(
        "Electricity: {} per kWh, usage: {:.0} kWh, bill: {}",
        price.money(),
        monthly_usage.to_kilowatt_hours(),
        monthly_bill
    );

    // Price per unit of mass
    let gold_price = Money::usd(65.0);
    let per_gram = Mass::grams(1.0);
    let gold = Price::new(gold_price, per_gram);

    let bar = Mass::kilograms(1.0);
    let bar_value = gold * bar;
    println!(
        "Gold: {} per gram, 1 kg bar: {}",
        gold.money(),
        bar_value
    );

    // Price per unit of volume (fuel)
    let fuel_price = Money::usd(3.50);
    let per_gallon = Volume::us_gallons(1.0);
    let fuel = Price::new(fuel_price, per_gallon);

    let tank = Volume::us_gallons(15.0);
    let fill_up = fuel * tank;
    println!(
        "Fuel: {} per gallon, fill {:.0} gal: {}",
        fuel.money(),
        tank.to_us_gallons(),
        fill_up
    );
}
