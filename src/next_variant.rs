use crate::utils::{default_fields_right, ignore_fields_left, impl_header};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;

extern crate alloc;
        
pub fn next_variant(item: 
      ItemEnum) -> TokenStream
{
    let impl_header = impl_header(&item);
    let mut next_variant_match_lines = TokenStream::new();

    let mut iter = item.variants.iter().peekable();
    while let Some(variant) = iter.next() {
        let next = match iter.peek() {
            Some(&next) => next,
            None => &item.variants[0],
        };

        let left_side = ignore_fields_left(&variant.fields);
        let right_side = default_fields_right(&next.fields);

        let left_ident = &variant.ident;
        let right_ident = &next.ident;

        next_variant_match_lines.extend(quote! {
            Self::#left_ident #left_side => Self::#right_ident #right_side,
        });
    }

    quote! {
        #item

        impl #impl_header {
            pub fn next_variant(&self) -> Self {
                match self {
                    #next_variant_match_lines
                    _ => unreachable!(),
                }
            }
        }
    }
}
