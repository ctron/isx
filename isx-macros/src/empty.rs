use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn derive_is_empty_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let is_empty_body = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let checks = fields_named.named.iter().map(|f| {
                    let ident = &f.ident;
                    quote! { self.#ident.is_empty() }
                });
                quote! { #(#checks)&&* }
            }
            Fields::Unnamed(fields_unnamed) => {
                let indices = (0..fields_unnamed.unnamed.len()).map(syn::Index::from);
                let checks = indices.clone().map(|i| {
                    quote! { self.#i.is_empty() }
                });
                quote! { #(#checks)&&* }
            }
            Fields::Unit => {
                quote! { true }
            }
        },
        Data::Enum(data_enum) => {
            let arms = data_enum.variants.iter().map(|variant| {
                let v_ident = &variant.ident;
                match &variant.fields {
                    Fields::Unit => {
                        quote! { Self::#v_ident => true }
                    }
                    Fields::Unnamed(fields) => {
                        let pats = (0..fields.unnamed.len())
                            .map(|i| syn::Ident::new(&format!("_f{i}"), name.span()));
                        let checks = pats.clone().map(|pat| quote! { #pat.is_empty() });
                        quote! { Self::#v_ident(#(#pats),*) => { #(#checks)&&* } }
                    }
                    Fields::Named(fields) => {
                        let pats = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
                        let checks = pats.clone().map(|pat| quote! { #pat.is_empty() });
                        quote! { Self::#v_ident { #(#pats),* } => { #(#checks)&&* } }
                    }
                }
            });
            quote! {
                match self {
                    #(#arms),*
                }
            }
        }
        _ => unimplemented!("IsEmpty can only be derived for structs and enums for now"),
    };

    let expanded = quote! {
        impl ::isx::IsEmpty for #name {
            fn is_empty(&self) -> bool {
                #is_empty_body
            }
        }
    };
    TokenStream::from(expanded)
}
