use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;

pub fn variant_amount(item: ItemEnum) -> TokenStream {
    let enum_ident = &item.ident;
    let variant_amount = item.variants.len();
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote! {
        #item

        impl #impl_generics #enum_ident #ty_generics #where_clause {
            pub const VARIANT_AMOUNT: ::core::primitive::usize = #variant_amount;
        }
    }
}
