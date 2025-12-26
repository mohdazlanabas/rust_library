# Rust Unit Conversion Library

A simple yet practical engineering unit conversion library for common measurements used in engineering projects.

## Overview

This project consists of two crates:
- **conversion-lib**: The core library providing unit conversion functions
- **conversion-user**: A demonstration application showing how to use the library

## Features

The library provides conversion functions for:

- **Length**: meters, feet, kilometers, miles
- **Temperature**: Celsius, Fahrenheit, Kelvin
- **Pressure**: bar, PSI, Pascal
- **Mass**: kilograms, pounds, tonnes, tons

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
conversion-lib = { path = "./conversion-lib" }
```

After publishing to crates.io, you'll be able to use:

```toml
[dependencies]
conversion-lib = "0.1.0"
```

## Usage

```rust
use conversion_lib::{length, temperature, pressure, mass};

fn main() {
    // Length conversions
    let meters = 100.0;
    let feet = length::meters_to_feet(meters);
    println!("{} meters = {:.2} feet", meters, feet);

    // Temperature conversions
    let celsius = 25.0;
    let fahrenheit = temperature::celsius_to_fahrenheit(celsius);
    println!("{}°C = {:.2}°F", celsius, fahrenheit);

    // Pressure conversions
    let bar = 2.5;
    let psi = pressure::bar_to_psi(bar);
    println!("{} bar = {:.2} PSI", bar, psi);

    // Mass conversions
    let kg = 75.0;
    let pounds = mass::kg_to_pounds(kg);
    println!("{} kg = {:.2} pounds", kg, pounds);
}
```

## Running the Demo

To see the library in action, run the demo application:

```bash
cd conversion-user
cargo run
```

## Available Conversions

### Length
- `meters_to_feet(meters: f64) -> f64`
- `feet_to_meters(feet: f64) -> f64`
- `km_to_miles(km: f64) -> f64`
- `miles_to_km(miles: f64) -> f64`

### Temperature
- `celsius_to_fahrenheit(celsius: f64) -> f64`
- `fahrenheit_to_celsius(fahrenheit: f64) -> f64`
- `celsius_to_kelvin(celsius: f64) -> f64`
- `kelvin_to_celsius(kelvin: f64) -> f64`

### Pressure
- `bar_to_psi(bar: f64) -> f64`
- `psi_to_bar(psi: f64) -> f64`
- `pascal_to_bar(pascal: f64) -> f64`
- `bar_to_pascal(bar: f64) -> f64`

### Mass
- `kg_to_pounds(kg: f64) -> f64`
- `pounds_to_kg(pounds: f64) -> f64`
- `tonnes_to_tons(tonnes: f64) -> f64`
- `tons_to_tonnes(tons: f64) -> f64`

## Testing

Run the test suite:

```bash
cd conversion-lib
cargo test
```

## Building

Build the library:

```bash
cd conversion-lib
cargo build --release
```

Build the demo application:

```bash
cd conversion-user
cargo build --release
```

## Project Structure

```
rust_library/
├── conversion-lib/       # Core library crate
│   ├── src/
│   │   └── lib.rs       # Library implementation
│   └── Cargo.toml
├── conversion-user/      # Demo application
│   ├── src/
│   │   └── main.rs      # Example usage
│   └── Cargo.toml
└── README.md
```

## License

MIT

## Author

Roger <roger@net1io.com>

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.
