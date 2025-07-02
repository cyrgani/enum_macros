use crate::utils::{default_fields_right, ignore_fields_left, impl_header};
use alloc::string::ToString;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_str, Error, ItemEnum, Type, Variant};

fn validate_discriminant_type(ty: &Type) -> Result<(), &'static str> {
    #[allow(clippy::match_same_arms)]
    match ty {
        Type::Array(_) => Ok(()),
        Type::BareFn(_) => Err("cannot use function pointer as discriminant type, see https://github.com/rust-lang/rust/issues/62411"),
        // TODO: find an example and add a test for this
        Type::Group(_) => Ok(()),
        Type::ImplTrait(_) => Err("cannot use `impl Trait` as discriminant type"),
        Type::Infer(_) => Err("cannot infer discriminant type"),
        Type::Macro(_) => Ok(()),
        // NOTE: this cannot be allowed because it would cause conflicting `From` / `TryFrom` impls
        // with the blanket implementations from `core` to allow never type coercion
        Type::Never(_) => Err("cannot use the never type as discriminant type"),
        Type::Paren(_) => Ok(()),
        Type::Path(_) => Ok(()),
        Type::Ptr(_) => Ok(()),
        Type::Reference(_) => Ok(()),
        Type::Slice(_) => Ok(()),
        Type::TraitObject(_) => Err("cannot use trait object as discriminant type"),
        Type::Tuple(_) => Ok(()),
        // TODO: find an example and add a test for this
        Type::Verbatim(_) => Ok(()),
        _ => unreachable!("a new kind of type was added to syn and not handled here yet"),
    }
}

/// TODO:
///  it is not reasonably possible to design something right now that has a string discriminant:
///  if we use `#[custom_discriminant(&str)]`, `impl From<Enum> for &str` does not compile
///  if we use `#[custom_discriminant(&'static str)]`, `impl TryFrom<&'static str> for Enum` is too strict
pub fn custom_discriminant(attr: TokenStream, item: ItemEnum) -> TokenStream {
    let attr_span = attr.span();

    let Ok(disc_ty) = syn::parse2::<Type>(attr) else {
        return Error::new(attr_span, "missing type of custom discriminant").to_compile_error();
    };
    if let Err(msg) = validate_discriminant_type(&disc_ty) {
        return Error::new(attr_span, msg).to_compile_error();
    }

    let mut errors = TokenStream::new();

    let impl_header = impl_header(&item);
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let enum_ident = &item.ident;
    let enum_type = quote! {
        #enum_ident #ty_generics
    };

    let mut disc_arg_ty = disc_ty.clone();
    let mut disc_ret_ty = disc_ty.clone();
    if disc_ty.to_token_stream().to_string() == "str" {
        disc_ret_ty = parse_str("&'static str").unwrap();
        disc_arg_ty = parse_str("&str").unwrap();
    }

    let mut variant_to_discriminant_match_lines = TokenStream::new();
    let mut try_discriminant_to_variant_match_lines = TokenStream::new();

    let mut consts = TokenStream::new();

    for variant in &item.variants {
        let Some((_, discriminant)) = variant.discriminant.as_ref() else {
            errors.extend(
                Error::new_spanned(variant, "missing variant discriminant").to_compile_error(),
            );
            continue;
        };

        let variant_ident = &variant.ident;

        let const_ident = format_ident!("__DISCRIMINANT_{variant_ident}");
        consts.extend(quote! {
            #[doc(hidden)]
            const #const_ident: #disc_ret_ty = #discriminant;
        });

        let left_side = ignore_fields_left(&variant.fields);
        let right_side = default_fields_right(&variant.fields);

        variant_to_discriminant_match_lines.extend(quote! {
            Self::#variant_ident #left_side => Self::#const_ident,
        });

        try_discriminant_to_variant_match_lines.extend(quote! {
            Self::#const_ident => ::core::result::Result::Ok(Self::#variant_ident #right_side),
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
        #errors

        #enum_without_discriminants

        impl #impl_header {
            #consts
        }

        impl #impl_header {
            pub fn custom_discriminant(&self) -> #disc_ret_ty {
                match self {
                    #variant_to_discriminant_match_lines
                    _ => unreachable!(),
                }
            }
        }

        impl #impl_generics From<#enum_type> for #disc_ret_ty #where_clause {
            fn from(value: #enum_type) -> Self {
                value.custom_discriminant()
            }
        }

        impl #impl_generics TryFrom<#disc_arg_ty> for #enum_type #where_clause {
            type Error = ();

            fn try_from(value: #disc_arg_ty) -> Result<Self, Self::Error> {
                match value {
                    #try_discriminant_to_variant_match_lines
                    _ => ::core::result::Result::Err(()),
                }
            }
        }
    }
}
