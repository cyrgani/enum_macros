use crate::utils::{default_fields_right, ignore_fields_left, impl_header};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, Type, Variant};

pub fn custom_discriminant(disc_ty: Type, item: ItemEnum) -> TokenStream {
    let impl_header = impl_header(&item);
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let enum_ident = &item.ident;
    let enum_type = quote! {
        #enum_ident #ty_generics
    };

    let mut variant_to_discriminant_match_lines = TokenStream::new();
    let mut try_discriminant_to_variant_match_lines = TokenStream::new();

    for variant in &item.variants {
        let (_, discriminant) = variant
            .discriminant
            .as_ref()
            .expect("Every variant must have a discriminant!");

        let variant_ident = &variant.ident;

        let left_side = ignore_fields_left(&variant.fields);
        let right_side = default_fields_right(&variant.fields);

        variant_to_discriminant_match_lines.extend(quote! {
            Self::#variant_ident #left_side => #discriminant,
        });

        try_discriminant_to_variant_match_lines.extend(quote! {
            #discriminant => ::core::result::Result::Ok(Self::#variant_ident #right_side),
        });
    }

    let enum_without_discriminants = ItemEnum {
        variants: item
            .variants
            .iter()
            .map(|variant| Variant {
                discriminant: None,
                ..variant.clone()
            })
            .collect(),
        ..item.clone()
    };

    quote! {
        #enum_without_discriminants

        impl #impl_header {
            pub fn custom_discriminant(&self) -> #disc_ty {
                match self {
                    #variant_to_discriminant_match_lines
                    _ => unreachable!(),
                }
            }
        }

        impl #impl_generics From<#enum_type> for #disc_ty #where_clause {
            fn from(value: #enum_type) -> Self {
                value.custom_discriminant()
            }
        }

        impl #impl_generics TryFrom<#disc_ty> for #enum_type #where_clause {
            type Error = ();

            fn try_from(value: #disc_ty) -> Result<Self, Self::Error> {
                match value {
                    #try_discriminant_to_variant_match_lines
                    _ => ::core::result::Result::Err(()),
                }
            }
        }
    }
}
