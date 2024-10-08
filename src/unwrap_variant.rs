use crate::utils::impl_header;
use alloc::string::ToString;
use alloc::vec;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Fields, FieldsUnnamed, ItemEnum};

const ERROR: &str = "UnwrapVariant can only be derived for single unnamed variants";

pub fn unwrap_variant(item: ItemEnum) -> Result<TokenStream, Error> {
    let mut output = TokenStream::new();

    let impl_header = impl_header(&item);

    for variant in &item.variants {
        let mut processed_attrs = vec![];

        for attr in &variant.attrs {
            attr.parse_nested_meta(|meta| {
                let ident = meta
                    .path
                    .get_ident()
                    .ok_or_else(|| meta.error("could not get ident of attribute path"))?
                    .to_string();

                if processed_attrs.contains(&ident) {
                    return Err(meta.error("duplicate attribute"));
                }

                match ident.as_str() {
                    "ref" | "mut" => {
                        processed_attrs.push(ident);
                        Ok(())
                    }
                    _ => Err(meta.error("unrecognized unwrap attribute")),
                }
            })?;
        }

        if processed_attrs.is_empty() {
            continue;
        }

        let Fields::Unnamed(FieldsUnnamed {
            paren_token: _,
            unnamed,
        }) = &variant.fields
        else {
            output.extend(Error::new_spanned(variant, ERROR).to_compile_error());
            continue;
        };

        if unnamed.len() != 1 {
            output.extend(Error::new_spanned(variant, ERROR).to_compile_error());
            continue;
        }

        let return_ty = &unnamed[0].ty;

        let variant_ident = &variant.ident;

        for kind in processed_attrs {
            let lowercase_variant = variant_ident.to_string().to_lowercase();
            let method_ident = format_ident!("unwrap_{lowercase_variant}_{kind}");
            let ref_ty = if kind == "ref" {
                quote! {&}
            } else {
                quote! {&mut}
            };

            output.extend(quote! {
                impl #impl_header {
                    pub fn #method_ident(#ref_ty self) -> #ref_ty #return_ty {
                        match self {
                            Self::#variant_ident(inner) => inner,
                            _ => panic!("tried to unwrap the wrong field"),
                        }
                    }
                }
            });
        }
    }

    Ok(output)
}
