use alloc::vec;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Fields, Generics, ItemEnum, Meta, Result, Token, Variant};

pub struct Arguments {
    attrs: Punctuated<Meta, Token![,]>,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: input.parse_terminated(Meta::parse, Token![,])?,
        })
    }
}

pub fn marker_type(args: Arguments, item: ItemEnum) -> TokenStream {
    let marker = ItemEnum {
        variants: item
            .variants
            .iter()
            .map(|variant| Variant {
                fields: Fields::Unit,
                ..variant.clone()
            })
            .collect(),
        ident: format_ident!("{}Marker", item.ident, span = Span::call_site()),
        generics: Generics::default(),
        attrs: vec![],
        ..item.clone()
    };

    let attrs = args
        .attrs
        .into_iter()
        .map(|meta| {
            quote! {
                #[#meta]
            }
        })
        .collect::<TokenStream>();

    quote! {
        #item

        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
        #attrs
        #marker
    }
}
