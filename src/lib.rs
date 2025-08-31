use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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
