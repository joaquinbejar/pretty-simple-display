//! # pretty-simple-display
//!
//! Custom derive macros for JSON serialization with pretty and simple formatting options.
//!
//! This crate provides four derive macros that implement `Debug` and `Display` traits using JSON serialization:
//!
//! - **`DebugPretty`**: Implements `Debug` with pretty-printed JSON output
//! - **`DisplayPretty`**: Implements `Display` with pretty-printed JSON output  
//! - **`DebugSimple`**: Implements `Debug` with compact JSON output
//! - **`DisplaySimple`**: Implements `Display` with compact JSON output
//!
//! ## Quick Start
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! pretty-simple-display = "0.1.0"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```
//!
//! ## Basic Usage
//!
//! ```rust
//! use pretty_simple_display::{DebugPretty, DisplaySimple};
//! use serde::Serialize;
//!
//! #[derive(Serialize, DebugPretty, DisplaySimple)]
//! struct User {
//!     id: u64,
//!     name: String,
//!     email: String,
//! }
//!
//! let user = User {
//!     id: 1,
//!     name: "Alice".to_string(),
//!     email: "alice@example.com".to_string(),
//! };
//!
//! // Pretty-printed JSON via Debug
//! format!("{:?}", user);
//! // Compact JSON via Display  
//! format!("{}", user);
//! ```
//!
//! ## Output Comparison
//!
//! ### Pretty vs Simple Formatting
//!
//! **Pretty formatting** (using `DebugPretty` or `DisplayPretty`):
//! ```json
//! {
//!   "id": 1,
//!   "name": "Alice",
//!   "email": "alice@example.com"
//! }
//! ```
//!
//! **Simple formatting** (using `DebugSimple` or `DisplaySimple`):
//! ```json
//! {"id":1,"name":"Alice","email":"alice@example.com"}
//! ```
//!
//! ## Advanced Usage
//!
//! ### Multiple Derives on Same Struct
//!
//! ```rust
//! use pretty_simple_display::{DebugPretty, DisplaySimple};
//! use serde::Serialize;
//!
//! #[derive(Serialize, DebugPretty, DisplaySimple)]
//! struct Product {
//!     id: u32,
//!     name: String,
//!     price: f64,
//! }
//!
//! let product = Product {
//!     id: 123,
//!     name: "Widget".to_string(),
//!     price: 29.99,
//! };
//!
//! // Debug uses pretty formatting
//! println!("{:?}", product);
//! // Output:
//! // {
//! //   "id": 123,
//! //   "name": "Widget",
//! //   "price": 29.99
//! // }
//!
//! // Display uses simple formatting  
//! println!("{}", product);
//! // Output: {"id":123,"name":"Widget","price":29.99}
//! ```
//!
//! ### Working with Enums
//!
//! ```rust
//! use pretty_simple_display::DebugPretty;
//! use serde::Serialize;
//!
//! #[derive(Serialize, DebugPretty)]
//! enum Status {
//!     Active,
//!     Inactive,
//!     Pending { reason: String },
//! }
//!
//! let status1 = Status::Active;
//! let status2 = Status::Pending { reason: "Verification".to_string() };
//!
//! println!("{:?}", status1); // "Active"
//! println!("{:?}", status2);
//! // {
//! //   "Pending": {
//! //     "reason": "Verification"
//! //   }
//! // }
//! ```
//!
//! ### Nested Structures
//!
//! ```rust
//! use pretty_simple_display::DisplaySimple;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct Address {
//!     street: String,
//!     city: String,
//! }
//!
//! #[derive(Serialize, DisplaySimple)]
//! struct Person {
//!     name: String,
//!     address: Address,
//!     tags: Vec<String>,
//! }
//!
//! let person = Person {
//!     name: "John".to_string(),
//!     address: Address {
//!         street: "123 Main St".to_string(),
//!         city: "Anytown".to_string(),
//!     },
//!     tags: vec!["developer".to_string(), "rust".to_string()],
//! };
//!
//! println!("{}", person);
//! // {"name":"John","address":{"street":"123 Main St","city":"Anytown"},"tags":["developer","rust"]}
//! ```
//!
//! ## Error Handling
//!
//! All macros handle serialization errors gracefully by displaying an error message instead of panicking:
//!
//! ```text
//! Error serializing to JSON: <error_details>
//! ```
//!
//! ## Requirements
//!
//! - Your struct must implement `serde::Serialize`
//! - The `serde` crate must be available in your dependencies
//! - Compatible with all types that serde can serialize (structs, enums, primitives, collections, etc.)
//!
//! ## Feature Comparison
//!
//! | Derive Macro | Trait | Format | Use Case |
//! |--------------|-------|---------|----------|
//! | `DebugPretty` | `Debug` | Pretty JSON | Development, debugging, logs |
//! | `DisplayPretty` | `Display` | Pretty JSON | User-facing output, formatted display |
//! | `DebugSimple` | `Debug` | Compact JSON | Performance-critical debugging |
//! | `DisplaySimple` | `Display` | Compact JSON | APIs, compact serialization |

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derive macro that implements Debug trait using pretty JSON serialization
///
/// This generates a Debug implementation that outputs the struct as
/// pretty-printed JSON using serde_json::to_string_pretty().
///
/// # Example
/// ```rust
/// use pretty_simple_display::DebugPretty;
/// use serde::Serialize;
///
/// #[derive(Serialize, DebugPretty)]
/// struct User {
///     id: u64,
///     name: String,
/// }
///
/// let user = User { id: 1, name: "Alice".to_string() };
/// println!("{:?}", user); // Pretty-printed JSON
/// ```
#[proc_macro_derive(DebugPretty)]
pub fn derive_debug_pretty(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match serde_json::to_string_pretty(self) {
                    Ok(pretty_json) => write!(f, "{}", pretty_json),
                    Err(e) => write!(f, "Error serializing to JSON: {}", e),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro that implements Display trait using pretty JSON serialization
///
/// This generates a Display implementation that outputs the struct as
/// pretty-printed JSON using serde_json::to_string_pretty().
///
/// # Example
/// ```rust
/// use pretty_simple_display::DisplayPretty;
/// use serde::Serialize;
///
/// #[derive(Serialize, DisplayPretty)]
/// struct User {
///     id: u64,
///     name: String,
/// }
///
/// let user = User { id: 1, name: "Alice".to_string() };
/// println!("{}", user); // Pretty-printed JSON
/// ```
#[proc_macro_derive(DisplayPretty)]
pub fn derive_display_pretty(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match serde_json::to_string_pretty(self) {
                    Ok(pretty_json) => write!(f, "{}", pretty_json),
                    Err(e) => write!(f, "Error serializing to JSON: {}", e),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro that implements Display trait using compact JSON serialization
///
/// This generates a Display implementation that outputs the struct as
/// compact JSON using serde_json::to_string().
///
/// # Example
/// ```rust
/// use pretty_simple_display::DisplaySimple;
/// use serde::Serialize;
///
/// #[derive(Serialize, DisplaySimple)]
/// struct User {
///     id: u64,
///     name: String,
/// }
///
/// let user = User { id: 1, name: "Alice".to_string() };
/// println!("{}", user); // Compact JSON
/// ```
#[proc_macro_derive(DisplaySimple)]
pub fn derive_display_simple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match serde_json::to_string(self) {
                    Ok(json) => write!(f, "{}", json),
                    Err(e) => write!(f, "Error serializing to JSON: {}", e),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro that implements Debug trait using compact JSON serialization
///
/// This generates a Debug implementation that outputs the struct as
/// compact JSON using serde_json::to_string().
///
/// # Example
/// ```rust
/// use pretty_simple_display::DebugSimple;
/// use serde::Serialize;
///
/// #[derive(Serialize, DebugSimple)]
/// struct User {
///     id: u64,
///     name: String,
/// }
///
/// let user = User { id: 1, name: "Alice".to_string() };
/// println!("{:?}", user); // Compact JSON
/// ```
#[proc_macro_derive(DebugSimple)]
pub fn derive_debug_simple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match serde_json::to_string(self) {
                    Ok(json) => write!(f, "{}", json),
                    Err(e) => write!(f, "Error serializing to JSON: {}", e),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

// Unit tests are included in the doctests above and integration tests in tests/ directory
