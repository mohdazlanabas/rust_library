# Rust Library Creation and Usage Guide

## Complete Workflow: From Library Creation to Usage

### Project Structure

```
rust-library-practice/
├── conversion-lib/          # The library package
│   ├── Cargo.toml          # Library configuration
│   ├── README.md           # Library documentation
│   └── src/
│       └── lib.rs          # Library implementation
├── conversion-user/         # Application using the library
│   ├── Cargo.toml          # App configuration (references library)
│   └── src/
│       └── main.rs         # Application code
└── reference/               # This documentation
    └── GUIDE.md
```

## Step 1: Understanding the Library Structure

### Cargo.toml (Library Configuration)

Key differences from a binary project:
- Uses `[lib]` section instead of `[[bin]]`
- Specifies `name` and `path` for the library
- Can include metadata for publishing (description, keywords, license)

### lib.rs vs main.rs

- **lib.rs**: Entry point for libraries, contains reusable code
- **main.rs**: Entry point for executables, contains `fn main()`

### Module Organization

```rust
// Public modules are exposed to library users
pub mod length { ... }
pub mod temperature { ... }

// Private modules (without pub) are internal only
mod internal_helpers { ... }
```

## Step 2: Building and Testing the Library

### Build the library
```bash
cd conversion-lib
cargo build
```

### Run tests
```bash
cargo test
```

### Generate documentation
```bash
cargo doc --open
```

This creates HTML docs from your /// comments and opens them in a browser.

## Step 3: Using the Library Locally

### In conversion-user/Cargo.toml

```toml
[dependencies]
conversion-lib = { path = "../conversion-lib" }
```

The `path` dependency tells Cargo to use the local version.

### Import and use in main.rs

```rust
use conversion_lib::length;
use conversion_lib::temperature;

fn main() {
    let feet = length::meters_to_feet(100.0);
    println!("100m = {}ft", feet);
}
```

## Step 4: Running the Application

```bash
cd conversion-user
cargo run
```

Cargo will:
1. Build the library dependency first
2. Build your application
3. Link them together
4. Run the application

## Step 5: Documentation Best Practices

### Module-level documentation (//!)
```rust
//! # Module Title
//! Module description
//! 
//! # Examples
//! ```
//! use conversion_lib::length;
//! let feet = length::meters_to_feet(10.0);
//! ```
```

### Function-level documentation (///)
```rust
/// Converts meters to feet
///
/// # Arguments
/// * `meters` - Length in meters
///
/// # Returns
/// Length in feet
///
/// # Examples
/// ```
/// let feet = meters_to_feet(10.0);
/// ```
pub fn meters_to_feet(meters: f64) -> f64 {
    meters * 3.28084
}
```

## Step 6: Testing Your Library

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let result = length::meters_to_feet(1.0);
        assert!((result - 3.28084).abs() < 0.0001);
    }
}
```

### Run specific tests
```bash
cargo test test_length_conversions
cargo test temperature -- --nocapture  # Show println! output
```

## Step 7: Publishing to crates.io (Optional)

### Prerequisites
1. Account on https://crates.io
2. Unique crate name
3. Complete metadata in Cargo.toml

### Publishing steps
```bash
# Login (one time)
cargo login <your-api-token>

# Publish
cargo publish
```

### After publishing, users can:
```toml
[dependencies]
conversion-lib = "0.1.0"
```

## Key Concepts

### Public vs Private

- `pub fn` - Public function, accessible to library users
- `fn` - Private function, internal use only
- `pub mod` - Public module
- `mod` - Private module

### Dependencies

- **Path dependency**: Local development
  ```toml
  my-lib = { path = "../my-lib" }
  ```

- **Published dependency**: From crates.io
  ```toml
  my-lib = "0.1.0"
  ```

- **Git dependency**: From repository
  ```toml
  my-lib = { git = "https://github.com/user/my-lib" }
  ```

### Versioning

Follow Semantic Versioning (SemVer):
- MAJOR.MINOR.PATCH (e.g., 1.2.3)
- MAJOR: Breaking changes
- MINOR: New features, backward compatible
- PATCH: Bug fixes, backward compatible

## Common Commands Cheat Sheet

```bash
# Library development
cargo new --lib my-library        # Create new library
cargo build                       # Build library
cargo test                        # Run tests
cargo doc --open                  # Generate and view docs
cargo publish                     # Publish to crates.io

# Using the library
cargo new my-app                  # Create new application
cargo build                       # Build app and dependencies
cargo run                         # Build and run
cargo check                       # Fast compilation check

# Helpful flags
cargo build --release             # Optimized build
cargo test -- --nocapture         # Show test output
cargo doc --no-deps              # Skip dependency docs
```

## Best Practices

1. **Write comprehensive documentation** - Your users will thank you
2. **Include examples in docs** - They're tested by `cargo test`
3. **Write tests** - Aim for high coverage
4. **Use semantic versioning** - Respect backward compatibility
5. **Keep modules focused** - Single responsibility principle
6. **Make APIs intuitive** - Easy to use correctly, hard to use incorrectly
7. **Provide README** - Quick start and usage examples

## Troubleshooting

### "Cannot find crate"
- Check path in Cargo.toml is correct
- Ensure library builds successfully
- Run `cargo clean` and rebuild

### "Function is private"
- Add `pub` keyword to function/module
- Check module hierarchy is public

### Documentation not showing
- Use /// for functions (three slashes)
- Use //! for modules (inside the module)
- Run `cargo doc --open` from library directory

## Next Steps

1. **Add more functionality** to your library
2. **Create integration tests** in `tests/` directory
3. **Add benchmarks** using criterion
4. **Set up CI/CD** for automated testing
5. **Write examples** in `examples/` directory
6. **Consider workspace** for multiple related crates
