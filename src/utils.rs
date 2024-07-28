use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, ItemEnum};

extern crate alloc;

pub fn impl_header(item: &ItemEnum) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let enum_ident = &item.ident;
    quote! {
        #impl_generics #enum_ident #ty_generics #where_clause
    }
}

pub fn ignore_fields_left(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Unit => TokenStream::new(),
        Fields::Unnamed(_) => quote! {(..)},
        Fields::Named(_) => quote! {{..}},
    }
}

pub fn default_fields_right(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Unit => TokenStream::new(),
        Fields::Unnamed(_) => {
            let defaults = alloc::vec![quote! {::core::default::Default::default()}; fields.len()];
            quote! {
                (#(#defaults),*)
            }
        }
        Fields::Named(fields) => {
            let defaults = fields.named.iter().map(|field| {
                let ident = field.ident.as_ref().unwrap();
                quote! {
                    #ident: ::core::default::Default::default()
                }
            });
            quote! {
                {#(#defaults),*}
            }
        }
    }
}
