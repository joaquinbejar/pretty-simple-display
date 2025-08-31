# PRETTY SIMPLE DISPLAY

[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![Dual License](https://img.shields.io/badge/license-MIT%20and%20Apache%202.0-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/pretty-simple-display.svg)](https://crates.io/crates/pretty-simple-display)
[![Downloads](https://img.shields.io/crates/d/pretty-simple-display.svg)](https://crates.io/crates/pretty-simple-display)
[![Stars](https://img.shields.io/github/stars/joaquinbejar/pretty-simple-display.svg)](https://github.com/joaquinbejar/pretty-simple-display/stargazers)
[![Issues](https://img.shields.io/github/issues/joaquinbejar/pretty-simple-display.svg)](https://github.com/joaquinbejar/pretty-simple-display/issues)
[![PRs](https://img.shields.io/github/issues-pr/joaquinbejar/pretty-simple-display.svg)](https://github.com/joaquinbejar/pretty-simple-display/pulls)
[![Build Status](https://img.shields.io/github/workflow/status/joaquinbejar/pretty-simple-display/CI)](https://github.com/joaquinbejar/pretty-simple-display/actions)
[![Coverage](https://img.shields.io/codecov/c/github/joaquinbejar/pretty-simple-display)](https://codecov.io/gh/joaquinbejar/pretty-simple-display)
[![Dependencies](https://img.shields.io/librariesio/github/joaquinbejar/pretty-simple-display)](https://libraries.io/github/joaquinbejar/pretty-simple-display)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/pretty-simple-display)
[![Wiki](https://img.shields.io/badge/wiki-latest-blue.svg)](https://deepwiki.com/joaquinbejar/pretty-simple-display)


## pretty-simple-display

Custom derive macros for JSON serialization with pretty and simple formatting options.

This crate provides four derive macros that implement `Debug` and `Display` traits using JSON serialization:

- **`DebugPretty`**: Implements `Debug` with pretty-printed JSON output
- **`DisplayPretty`**: Implements `Display` with pretty-printed JSON output
- **`DebugSimple`**: Implements `Debug` with compact JSON output
- **`DisplaySimple`**: Implements `Display` with compact JSON output

### Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
pretty-simple-display = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Basic Usage

```rust
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::Serialize;

#[derive(Serialize, DebugPretty, DisplaySimple)]
struct User {
    id: u64,
    name: String,
    email: String,
}

let user = User {
    id: 1,
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
};

// Pretty-printed JSON via Debug
format!("{:?}", user);
// Compact JSON via Display
format!("{}", user);
```

### Output Comparison

#### Pretty vs Simple Formatting

**Pretty formatting** (using `DebugPretty` or `DisplayPretty`):
```json
{
  "id": 1,
  "name": "Alice",
  "email": "alice@example.com"
}
```

**Simple formatting** (using `DebugSimple` or `DisplaySimple`):
```json
{"id":1,"name":"Alice","email":"alice@example.com"}
```

### Advanced Usage

#### Multiple Derives on Same Struct

```rust
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::Serialize;

#[derive(Serialize, DebugPretty, DisplaySimple)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

let product = Product {
    id: 123,
    name: "Widget".to_string(),
    price: 29.99,
};

// Debug uses pretty formatting
println!("{:?}", product);
// Output:
// {
//   "id": 123,
//   "name": "Widget",
//   "price": 29.99
// }

// Display uses simple formatting
println!("{}", product);
// Output: {"id":123,"name":"Widget","price":29.99}
```

#### Working with Enums

```rust
use pretty_simple_display::DebugPretty;
use serde::Serialize;

#[derive(Serialize, DebugPretty)]
enum Status {
    Active,
    Inactive,
    Pending { reason: String },
}

let status1 = Status::Active;
let status2 = Status::Pending { reason: "Verification".to_string() };

println!("{:?}", status1); // "Active"
println!("{:?}", status2);
// {
//   "Pending": {
//     "reason": "Verification"
//   }
// }
```

#### Nested Structures

```rust
use pretty_simple_display::DisplaySimple;
use serde::Serialize;

#[derive(Serialize)]
struct Address {
    street: String,
    city: String,
}

#[derive(Serialize, DisplaySimple)]
struct Person {
    name: String,
    address: Address,
    tags: Vec<String>,
}

let person = Person {
    name: "John".to_string(),
    address: Address {
        street: "123 Main St".to_string(),
        city: "Anytown".to_string(),
    },
    tags: vec!["developer".to_string(), "rust".to_string()],
};

println!("{}", person);
// {"name":"John","address":{"street":"123 Main St","city":"Anytown"},"tags":["developer","rust"]}
```

### Error Handling

All macros handle serialization errors gracefully by displaying an error message instead of panicking:

```
Error serializing to JSON: <error_details>
```

### Requirements

- Your struct must implement `serde::Serialize`
- The `serde` crate must be available in your dependencies
- Compatible with all types that serde can serialize (structs, enums, primitives, collections, etc.)

### Feature Comparison

| Derive Macro | Trait | Format | Use Case |
|--------------|-------|---------|----------|
| `DebugPretty` | `Debug` | Pretty JSON | Development, debugging, logs |
| `DisplayPretty` | `Display` | Pretty JSON | User-facing output, formatted display |
| `DebugSimple` | `Debug` | Compact JSON | Performance-critical debugging |
| `DisplaySimple` | `Display` | Compact JSON | APIs, compact serialization |



## Contribution and Contact

We welcome contributions to this project! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the project still builds and all tests pass.
4. Commit your changes and push your branch to your forked repository.
5. Submit a pull request to the main repository.

If you have any questions, issues, or would like to provide feedback, please feel free to contact the project maintainer:

**Joaquín Béjar García**
- Email: jb@taunais.com
- GitHub: [joaquinbejar](https://github.com/joaquinbejar)

We appreciate your interest and look forward to your contributions!

## ✍️ License

Licensed under MIT license
