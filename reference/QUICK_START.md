# Quick Start Guide - Rust Library Practice

## Prerequisites

Install Rust and Cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

## Step-by-Step Execution

### 1. Test the Library

```bash
cd conversion-lib
cargo test
```

Expected output:
```
running 4 tests
test tests::test_length_conversions ... ok
test tests::test_temperature_conversions ... ok
test tests::test_pressure_conversions ... ok
test tests::test_mass_conversions ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured
```

### 2. Build the Library

```bash
cargo build
```

This creates `target/debug/libconversion_lib.rlib`

### 3. Generate Documentation

```bash
cargo doc --open
```

This will:
- Generate HTML documentation
- Open it in your default browser
- Show all modules, functions, and examples

### 4. Run the Application

```bash
cd ../conversion-user
cargo run
```

Expected output:
```
=== Engineering Unit Conversion Demo ===

LENGTH CONVERSIONS:
  100 meters = 328.08 feet
  42.195 km (marathon) = 26.22 miles

TEMPERATURE CONVERSIONS:
  25°C = 77.00°F = 298.15K
  Boiling point: 100°C = 212.00°F

PRESSURE CONVERSIONS:
  2.5 bar = 36.26 PSI = 250000 Pa

MASS CONVERSIONS:
  75 kg = 165.35 pounds
  500 tonnes/day = 551.16 tons/day

=== Practical Engineering Example ===
AD Facility Specifications:
  Capacity: 500 tonnes/day (551.16 tons/day)
  Operating Temperature: 55°C (131.00°F)
  Operating Pressure: 1.5 bar (21.76 PSI)
```

### 5. Experiment with the Library

Try modifying `conversion-user/src/main.rs`:

```rust
use conversion_lib::length;

fn main() {
    // Your custom conversions
    let pipeline_length_m = 1500.0;
    let pipeline_length_ft = length::meters_to_feet(pipeline_length_m);
    
    println!("Pipeline: {} meters = {:.2} feet", 
             pipeline_length_m, pipeline_length_ft);
}
```

Then run:
```bash
cargo run
```

### 6. Add Your Own Conversion Function

Edit `conversion-lib/src/lib.rs`, add to the `length` module:

```rust
/// Converts inches to centimeters
pub fn inches_to_cm(inches: f64) -> f64 {
    inches * 2.54
}
```

Add a test:
```rust
#[test]
fn test_inches_to_cm() {
    let result = length::inches_to_cm(12.0);
    assert!((result - 30.48).abs() < 0.001);
}
```

Run tests:
```bash
cd ../conversion-lib
cargo test
```

### 7. Use Your New Function

Update `conversion-user/src/main.rs`:

```rust
use conversion_lib::length;

fn main() {
    let inches = 24.0;
    let cm = length::inches_to_cm(inches);
    println!("{} inches = {:.2} cm", inches, cm);
}
```

```bash
cd ../conversion-user
cargo run
```

## Troubleshooting

### Error: "cannot find crate"
Solution: Make sure you're in the correct directory and the path in Cargo.toml is correct.

### Error: "function is private"
Solution: Add `pub` before the function in lib.rs

### Tests failing
Solution: Check your formulas. Run with output to see details:
```bash
cargo test -- --nocapture
```

## Understanding the Output

### When you run `cargo build`:
- Creates `target/debug/` directory
- Compiles library to `.rlib` file
- Shows any compilation errors

### When you run `cargo test`:
- Runs all `#[test]` functions
- Shows pass/fail for each test
- Displays test statistics

### When you run `cargo doc`:
- Parses /// and //! comments
- Generates HTML documentation
- Creates clickable API reference

## Next Exercise Ideas

1. **Add a new module** - Create `pub mod energy` for energy conversions
2. **Add error handling** - Return `Result<f64, String>` for invalid inputs
3. **Add validation** - Check for negative temperatures in Kelvin conversion
4. **Create a CLI** - Use `clap` crate to make a command-line tool
5. **Add integration tests** - Create `tests/integration_test.rs`

## Key Takeaways

- Libraries use `lib.rs` instead of `main.rs`
- The `pub` keyword exposes items to library users
- Path dependencies work for local development
- Documentation is built from code comments
- Tests run automatically with `cargo test`
- The same library can be used by multiple applications
