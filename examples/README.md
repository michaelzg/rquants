# RQuants Examples

Runnable examples showing every module in action with real-world scenarios.

## Running

Run a single example:

```bash
cargo run --example basic_usage
```

Run all examples:

```bash
for ex in basic_usage physics_calculations financial_calculations space_and_mass information_and_data thermal radiation_and_light; do
  echo "=== $ex ===" && cargo run --example "$ex" && echo
done
```

## Examples

### `basic_usage`
Start here. Covers creating quantities, DSL syntax (`100.0.meters()`), unit conversions, and equality across units. Features Usain Bolt, the Burj Khalifa, and a blue whale.

### `physics_calculations`
Kinematics, dynamics, energy, electromagnetism, and pressure. Tesla Plaid acceleration, SpaceX Falcon 9 thrust, Bugatti Chiron kinetic energy, Back to the Future's 1.21 GW flux capacitor, Mariana Trench pressure, and more.

### `financial_calculations`
Money arithmetic, currency exchange rates, and generic `Price<Q>` for pricing any quantity. Concert tickets, PS5 discounts, Tokyo trip budgets, Bitcoin conversions, gas prices, and cloud storage costs.

### `space_and_mass`
Length (hair width to light-years), area (soccer fields to Central Park), volume (soda cans to Olympic pools), angles (pizza slices to Earth's tilt), mass (smartphones to elephants), density (water vs mercury), chemical amounts, and frequency (heartbeats to CPU clocks).

### `information_and_data`
Storage sizes (photos, games, Wikipedia), download times on various connections, and streaming bandwidth math. Covers Netflix 4K, Twitch uploads, Zoom calls, 5G speeds, and data cap budgets.

### `thermal`
Temperature scales and conversions, extreme temps (absolute zero to the Sun's core), cooking temperatures (steak, pizza ovens, candy making), and thermal capacity (why metal heats faster than water).

### `radiation_and_light`
Radioactivity (bananas to Chernobyl), radiation dose (X-rays, flights, CT scans), solar irradiance (solar panel sizing), and photometry (sunlight vs moonlight, screen brightness, LED bulbs).
