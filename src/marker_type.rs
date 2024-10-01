use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Fields, Generics, ItemEnum, Variant};

pub fn marker_type(item: ItemEnum) -> TokenStream {
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
        ..item.clone()
    };

    quote! {
        #item

        #[derive(Clone, Copy)]
        #marker
    }
}
