use proc_macro::TokenStream;

mod default;
mod empty;

use default::derive_is_default_impl;
use empty::derive_is_empty_impl;

#[proc_macro_derive(IsDefault, attributes(default))]
/// Allow implementing `IsDefault` for a custom type.
pub fn derive_is_default(input: TokenStream) -> TokenStream {
    derive_is_default_impl(input)
}

#[proc_macro_derive(IsEmpty, attributes(empty))]
/// Allow implementing `IsEmpty` for a custom type.
pub fn derive_is_empty(input: TokenStream) -> TokenStream {
    derive_is_empty_impl(input)
}
