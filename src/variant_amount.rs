use crate::utils::impl_header;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;

pub fn variant_amount(item: ItemEnum) -> TokenStream {
    let variant_amount = item.variants.len();
    let impl_header = impl_header(&item);

    quote! {
        #item

        impl #impl_header {
            pub const VARIANT_AMOUNT: ::core::primitive::usize = #variant_amount;
        }
    }
}
