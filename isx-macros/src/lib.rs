use proc_macro::TokenStream;

mod default;
mod empty;

use default::derive_is_default_impl;
use empty::derive_is_empty_impl;

/// Allow implementing `IsDefault` for a custom type.
#[proc_macro_derive(IsDefault, attributes(default))]
pub fn derive_is_default(input: TokenStream) -> TokenStream {
    derive_is_default_impl(input)
}

/// Allow implementing `IsEmpty` for a custom type.
#[proc_macro_derive(IsEmpty, attributes(empty))]
pub fn derive_is_empty(input: TokenStream) -> TokenStream {
    derive_is_empty_impl(input)
}
