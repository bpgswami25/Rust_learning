//! # Procedural Macros in Rust
//!
//! This crate demonstrates how procedural macros work in Rust. Procedural macros are a powerful feature
//! that allow you to write code that generates other code at compile time. Unlike declarative macros
//! (which use `macro_rules!`), procedural macros operate on the abstract syntax tree (AST) of your code.
//!
//! ## What is a Procedural Macro?
//!
//! Procedural macros come in three types:
//! - **Function-like macros**: Look like function calls, e.g., `my_macro!(...)`
//! - **Derive macros**: Add `#[derive(MyMacro)]` to structs/enums to auto-implement traits
//! - **Attribute macros**: Add `#[my_macro(...)]` to items to modify them
//!
//! This crate focuses on a **derive macro** called `MyDebug`.
//!
//! ## What Does This Crate Do?
//!
//! The `MyDebug` derive macro automatically implements the `Debug` trait for enums.
//! Instead of the default debug output (just the variant name), it uses custom messages
//! specified with the `#[debug("message")]` attribute. This is similar to the `thiserror` crate.
//!
//! ## How Procedural Macros Work (Simplified)
//!
//! **Input**: The macro receives a `TokenStream` - a sequence of tokens from your source code.
//!    Example: `TokenStream [Ident { ident: "enum", span: #0 bytes(52..56) }, ...]`
//!
//! **Parsing**: Use `parse_macro_input!` to convert the `TokenStream` into a structured
//!    Rust Abstract Syntax Tree (AST). This gives you usable data structures like `DeriveInput`.
//!    Example: `DeriveInput { attrs: [], vis: Visibility::Inherited, ident: Ident { ... }, ... }`
//!
//! **Processing**: Analyze the AST, extract information (like enum variants and attributes),
//!    and decide what code to generate.
//!
//! **Code Generation**: Use `quote!` to write the code you want to insert. This converts
//!    your Rust-like code back into a `TokenStream`.
//!
//! **Output**: Return the `TokenStream` to be inserted into your code.
//!
//! ## Example Usage
//!
//! ```rust
//! use procedural_macros::MyDebug;
//!
//! #[derive(MyDebug)]
//! enum ServerError {
//!     #[debug("The connection timed out")]
//!     Timeout,
//!     #[debug("Permission was denied by the OS")]
//!     PermissionDenied,
//!     // If no #[debug] attribute, uses the variant name
//!     Unknown,
//! }
//!
//! let err = ServerError::Timeout;
//! println!("{:?}", err); // Output: DEBUG: The connection timed out
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, LitStr};


/// A derive macro that implements `Debug` for enums with custom messages.
///
/// This macro looks for `#[debug("message")]` attributes on enum variants.
/// If found, it uses that message in the debug output. Otherwise, it falls back
/// to the variant name.
///
/// ## Example
/// ```rust
/// use procedural_macros::MyDebug;
///
/// #[derive(MyDebug)]
/// enum MyEnum {
///     #[debug("Something went wrong")]
///     Error,
///     #[debug("All good")]
///     Ok,
/// }
///
/// let e = MyEnum::Error;
/// println!("{:?}", e); // DEBUG: Something went wrong
/// ```
#[proc_macro_derive(MyDebug, attributes(debug))]
pub fn derive_my_debug(input: TokenStream) -> TokenStream {
    // Parse the input TokenStream into a DeriveInput (the AST)
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the enum
    let name = &input.ident;

    // Ensure it's an enum
    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("MyDebug only works on enums"),
    };

    // Generate match arms for each variant
    let debug_arms = variants.iter().map(|variant| {
        let var_ident = &variant.ident;

        // Find the #[debug("...")] attribute, if any
        let msg = variant.attrs.iter()
            .find(|attr| attr.path().is_ident("debug"))
            .and_then(|attr| attr.parse_args::<LitStr>().ok())
            .map(|lit| lit.value())
            .unwrap_or_else(|| var_ident.to_string()); // Default to variant name

        // Generate the match arm
        quote! {
            #name::#var_ident => write!(f, "DEBUG: {}", #msg),
        }
    });

    // Generate the full impl block
    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#debug_arms)*
                }
            }
        }
    };

    // Convert back to TokenStream and return
    TokenStream::from(expanded)
}
