# Advanced Library Topics

## 1. Organizing Complex Libraries

### Multi-file Module Structure

Instead of putting everything in `lib.rs`, you can split into multiple files:

```
conversion-lib/src/
├── lib.rs              # Main entry, re-exports modules
├── length.rs           # Length conversions
├── temperature.rs      # Temperature conversions
├── pressure.rs         # Pressure conversions
└── mass.rs            # Mass conversions
```

In `lib.rs`:
```rust
//! Conversion library

pub mod length;
pub mod temperature;
pub mod pressure;
pub mod mass;

// Re-export commonly used items
pub use length::{meters_to_feet, feet_to_meters};
pub use temperature::{celsius_to_fahrenheit, fahrenheit_to_celsius};
```

Each module file (e.g., `length.rs`):
```rust
//! Length conversion functions

/// Converts meters to feet
pub fn meters_to_feet(meters: f64) -> f64 {
    meters * 3.28084
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_meters_to_feet() {
        assert_eq!(meters_to_feet(1.0), 3.28084);
    }
}
```

## 2. Error Handling

### Using Result Types

```rust
#[derive(Debug)]
pub enum ConversionError {
    NegativeKelvin,
    InvalidInput,
}

pub fn celsius_to_kelvin(celsius: f64) -> Result<f64, ConversionError> {
    let kelvin = celsius + 273.15;
    if kelvin < 0.0 {
        Err(ConversionError::NegativeKelvin)
    } else {
        Ok(kelvin)
    }
}
```

Usage:
```rust
match celsius_to_kelvin(-300.0) {
    Ok(kelvin) => println!("{}K", kelvin),
    Err(e) => println!("Error: {:?}", e),
}
```

## 3. Generic Functions

### Type Parameters

```rust
pub trait Convertible {
    fn convert(&self, factor: f64) -> Self;
}

impl Convertible for f64 {
    fn convert(&self, factor: f64) -> Self {
        self * factor
    }
}

pub fn generic_conversion<T: Convertible>(value: T, factor: f64) -> T {
    value.convert(factor)
}
```

## 4. Creating Traits

### Define Conversion Trait

```rust
pub trait UnitConversion {
    type Output;
    fn to_metric(&self) -> Self::Output;
    fn from_metric(metric: Self::Output) -> Self;
}

pub struct Fahrenheit(pub f64);
pub struct Celsius(pub f64);

impl UnitConversion for Fahrenheit {
    type Output = Celsius;
    
    fn to_metric(&self) -> Celsius {
        Celsius((self.0 - 32.0) * 5.0 / 9.0)
    }
    
    fn from_metric(celsius: Celsius) -> Self {
        Fahrenheit(celsius.0 * 9.0 / 5.0 + 32.0)
    }
}
```

Usage:
```rust
let temp = Fahrenheit(98.6);
let celsius = temp.to_metric();
println!("Body temperature: {}°C", celsius.0);
```

## 5. Integration Tests

Create `conversion-lib/tests/integration_test.rs`:

```rust
use conversion_lib::length;
use conversion_lib::temperature;

#[test]
fn test_combined_conversions() {
    // Test a real-world scenario
    let pipe_length_m = 100.0;
    let pipe_length_ft = length::meters_to_feet(pipe_length_m);
    
    let operating_temp_c = 55.0;
    let operating_temp_f = temperature::celsius_to_fahrenheit(operating_temp_c);
    
    assert!(pipe_length_ft > 300.0);
    assert!(operating_temp_f > 100.0);
}
```

Run integration tests only:
```bash
cargo test --test integration_test
```

## 6. Benchmarking

Add to `Cargo.toml`:
```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "conversions"
harness = false
```

Create `benches/conversions.rs`:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use conversion_lib::length;

fn benchmark_conversions(c: &mut Criterion) {
    c.bench_function("meters_to_feet", |b| {
        b.iter(|| length::meters_to_feet(black_box(100.0)))
    });
}

criterion_group!(benches, benchmark_conversions);
criterion_main!(benches);
```

Run benchmarks:
```bash
cargo bench
```

## 7. Feature Flags

Add optional features in `Cargo.toml`:
```toml
[features]
default = ["length", "temperature"]
length = []
temperature = []
pressure = []
mass = []
all = ["length", "temperature", "pressure", "mass"]
```

In `lib.rs`:
```rust
#[cfg(feature = "length")]
pub mod length;

#[cfg(feature = "temperature")]
pub mod temperature;
```

Users can opt-in:
```toml
[dependencies]
conversion-lib = { version = "0.1", features = ["pressure"] }
```

## 8. Const Functions

For compile-time conversions:
```rust
pub const fn meters_to_feet_const(meters: f64) -> f64 {
    meters * 3.28084
}

const FIELD_LENGTH: f64 = meters_to_feet_const(100.0);
```

## 9. Macros for Repetitive Code

```rust
macro_rules! define_conversion {
    ($name:ident, $from:ty, $to:ty, $factor:expr) => {
        pub fn $name(value: $from) -> $to {
            (value as f64 * $factor) as $to
        }
    };
}

define_conversion!(mm_to_inches, f64, f64, 0.0393701);
define_conversion!(inches_to_mm, f64, f64, 25.4);
```

## 10. Documentation Examples as Tests

Examples in doc comments are automatically tested:

```rust
/// Converts meters to feet
///
/// # Examples
///
/// ```
/// use conversion_lib::length::meters_to_feet;
/// 
/// let feet = meters_to_feet(100.0);
/// assert_eq!(feet, 328.084);
/// ```
pub fn meters_to_feet(meters: f64) -> f64 {
    meters * 3.28084
}
```

## 11. Publishing Checklist

Before publishing to crates.io:

- [ ] All tests pass
- [ ] Documentation is complete
- [ ] README has clear examples
- [ ] License file exists
- [ ] Version follows SemVer
- [ ] No security vulnerabilities (`cargo audit`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No warnings (`cargo clippy`)
- [ ] Examples work
- [ ] Changelog is updated

Commands:
```bash
cargo test
cargo fmt
cargo clippy
cargo publish --dry-run
cargo publish
```

## 12. Workspace for Multiple Crates

Create a workspace for related libraries:

```
my-engineering-tools/
├── Cargo.toml          # Workspace configuration
├── conversion-lib/
│   ├── Cargo.toml
│   └── src/
├── calculation-lib/
│   ├── Cargo.toml
│   └── src/
└── engineering-cli/
    ├── Cargo.toml
    └── src/
```

Root `Cargo.toml`:
```toml
[workspace]
members = [
    "conversion-lib",
    "calculation-lib",
    "engineering-cli",
]
```

Benefits:
- Shared dependencies
- Build all with one command
- Easier to maintain related crates

## 13. Continuous Integration

Example GitHub Actions (`.github/workflows/rust.yml`):

```yaml
name: Rust

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - run: cargo test --all
    - run: cargo clippy -- -D warnings
    - run: cargo fmt -- --check
```

## Key Principles for Library Design

1. **Keep API surface small** - Less to maintain, easier to learn
2. **Make common cases easy** - Optimize for typical usage
3. **Provide good defaults** - Sensible behavior out of the box
4. **Document everything public** - Users shouldn't read source
5. **Version carefully** - Don't break existing code
6. **Test thoroughly** - Users depend on correctness
7. **Consider performance** - But readability first
8. **Handle errors gracefully** - Return Result when appropriate
9. **Use semantic types** - Newtype pattern for clarity
10. **Think about extension** - Traits over concrete types

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [crates.io](https://crates.io/)
- [docs.rs](https://docs.rs/) - Automatic documentation hosting
