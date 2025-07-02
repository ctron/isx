use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

fn has_default_attr(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("default"))
}

pub fn derive_is_default_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let is_default_body = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let checks = fields_named.named.iter().map(|f| {
                    let ident = &f.ident;
                    quote! { self.#ident.is_default() }
                });
                quote! { #(#checks)&&* }
            }
            Fields::Unnamed(fields_unnamed) => {
                let indices = (0..fields_unnamed.unnamed.len()).map(syn::Index::from);
                let checks = indices.clone().map(|i| {
                    quote! { self.#i.is_default() }
                });
                quote! { #(#checks)&&* }
            }
            Fields::Unit => {
                quote! { true }
            }
        },
        Data::Enum(data_enum) => {
            // Find the variant with #[default]
            let mut default_variant = None;
            for variant in &data_enum.variants {
                if has_default_attr(&variant.attrs) {
                    if default_variant.is_some() {
                        return syn::Error::new_spanned(
                            &variant.ident,
                            "Multiple #[default] variants found",
                        )
                        .to_compile_error()
                        .into();
                    }
                    default_variant = Some(variant);
                }
            }
            if let Some(variant) = default_variant {
                let v_ident = &variant.ident;
                match &variant.fields {
                    Fields::Unit => {
                        quote! {
                            matches!(self, Self::#v_ident)
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let pats = (0..fields.unnamed.len())
                            .map(|i| syn::Ident::new(&format!("_f{i}"), name.span()));
                        let checks = pats.clone().map(|pat| quote! { #pat.is_default() });
                        quote! {
                            match self {
                                Self::#v_ident(#(#pats),*) => { #(#checks)&&* },
                                _ => false
                            }
                        }
                    }
                    Fields::Named(fields) => {
                        let pats = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
                        let checks = pats.clone().map(|pat| quote! { #pat.is_default() });
                        quote! {
                            match self {
                                Self::#v_ident { #(#pats),* } => { #(#checks)&&* },
                                _ => false
                            }
                        }
                    }
                }
            } else {
                // Fallback: compare to Default::default()
                quote! {
                    self == &<Self as ::core::default::Default>::default()
                }
            }
        }
        _ => unimplemented!("IsDefault can only be derived for structs and enums for now"),
    };

    let expanded = quote! {
        impl ::isx::IsDefault for #name {
            fn is_default(&self) -> bool {
                #is_default_body
            }
        }
    };
    TokenStream::from(expanded)
}
