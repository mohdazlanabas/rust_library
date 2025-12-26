# Conversion Library

A simple Rust library for engineering unit conversions.

## Features

- **Length conversions**: meters ↔ feet, kilometers ↔ miles
- **Temperature conversions**: Celsius ↔ Fahrenheit ↔ Kelvin
- **Pressure conversions**: bar ↔ PSI ↔ Pascal
- **Mass conversions**: kilograms ↔ pounds, tonnes ↔ tons

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
conversion-lib = { path = "../conversion-lib" }
```

For published crates on crates.io (after publishing):
```toml
[dependencies]
conversion-lib = "0.1.0"
```

## Usage

```rust
use conversion_lib::length;
use conversion_lib::temperature;
use conversion_lib::pressure;
use conversion_lib::mass;

fn main() {
    // Length conversions
    let feet = length::meters_to_feet(100.0);
    println!("100 meters = {:.2} feet", feet);

    // Temperature conversions
    let fahrenheit = temperature::celsius_to_fahrenheit(25.0);
    println!("25°C = {:.2}°F", fahrenheit);

    // Pressure conversions
    let psi = pressure::bar_to_psi(2.5);
    println!("2.5 bar = {:.2} PSI", psi);

    // Mass conversions
    let pounds = mass::kg_to_pounds(75.0);
    println!("75 kg = {:.2} pounds", pounds);
}
```

## Building the Library

```bash
# Build the library
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_length_conversions
```

## Documentation

Generate and view the documentation:

```bash
cargo doc --open
```

This will build the HTML documentation and open it in your browser.

## Project Structure

```
conversion-lib/
├── Cargo.toml          # Package configuration
├── src/
│   └── lib.rs          # Library source code
└── README.md           # This file
```

## Publishing to crates.io

1. **Create an account** at https://crates.io
2. **Login via cargo**:
   ```bash
   cargo login <your-api-token>
   ```
3. **Publish**:
   ```bash
   cargo publish
   ```

## License

MIT
