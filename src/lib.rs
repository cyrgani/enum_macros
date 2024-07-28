#![cfg_attr(not(feature = "std"), no_std)]
//! A collection of useful macros that make working with enums easier.
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo, missing_docs)]
#![allow(clippy::needless_pass_by_value)]
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum, Type};

mod utils;

#[cfg(feature = "custom_discriminant")]
mod custom_discriminant;

#[cfg(feature = "marker_type")]
mod marker_type;

#[cfg(feature = "next_variant")]
mod next_variant;

#[cfg(feature = "variant_amount")]
mod variant_amount;

/// Adds a constant storing the amount of variants the enum has to the enum.
///
/// # Examples
/// ```
/// use enum_macros::variant_amount;
///
/// #[variant_amount]
/// enum Example {
///     A,
///     B,
///     C,
/// }
///
/// assert_eq!(Example::VARIANT_AMOUNT, 3);
/// ```
#[cfg(feature = "variant_amount")]
#[proc_macro_attribute]
pub fn variant_amount(_attr: TokenStream, item: TokenStream) -> TokenStream {
    variant_amount::variant_amount(parse_macro_input!(item as ItemEnum)).into()
}

/// Adds support for custom enum "discriminants" of other types than `isize`.
///
/// Every enum variant must be given a discriminant.
/// This macro takes the type of the discriminant as argument.
///
/// This macro adds:
/// * `impl From<ENUM_TYPE> for DISCRIMINANT_TYPE`
/// * `impl TryFrom<DISCRIMINANT_TYPE> for ENUM_TYPE`
///
/// # Panics
/// Panics if there is a variant without a custom discriminant.
///
/// # Examples
/// ```
/// use enum_macros::custom_discriminant;
///
/// #[custom_discriminant(&'static str)]
/// enum Example {
///     Data = "Data",
///     Thing = "OtherThing",
/// }
///
/// assert_eq!("OtherThing", Example::Thing.custom_discriminant());
/// ```
#[cfg(feature = "custom_discriminant")]
#[proc_macro_attribute]
pub fn custom_discriminant(attr: TokenStream, item: TokenStream) -> TokenStream {
    custom_discriminant::custom_discriminant(
        parse_macro_input!(attr as Type),
        parse_macro_input!(item as ItemEnum),
    )
    .into()
}

/// Adds a new enum that has the same variants as this enum, but holds no data.
/// The new enum will be called `{OLD_NAME}Marker` and always implement [`Clone`] and [`Copy`].
///
/// Any macros applied to the enum preceding this one will be invoked on the marker type as well.
///
/// # Examples
/// ```
/// use enum_macros::marker_type;
///
/// enum Type<'a, T: Clone> {
///     A(&'a u32),
///     B(T),
///     C(&'a T),
/// }
///
/// /* generates:
/// #[derive(Clone, Copy)]
/// enum TypeMarker {
///     A,
///     B,
///     C,
/// }
/// */
/// ```
#[cfg(feature = "marker_type")]
#[proc_macro_attribute]
pub fn marker_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    marker_type::marker_type(parse_macro_input!(item as ItemEnum)).into()
}

/// Adds a method for advancing to the next enum variant.
/// Wraps around at the last element.
///
/// For enum variants with data, every field must implement [Default](Default).
///
/// # Examples
/// ```
/// use enum_macros::next_variant;
///
/// #[next_variant]
/// #[derive(PartialEq, Debug)]
/// enum Example {
///     A,
///     B,
/// }
///
/// assert_eq!(Example::B, Example::A.next_variant());
/// assert_eq!(Example::A, Example::B.next_variant());
/// ```
#[cfg(feature = "next_variant")]
#[proc_macro_attribute]
pub fn next_variant(_attr: TokenStream, item: TokenStream) -> TokenStream {
    next_variant::next_variant(parse_macro_input!(item as ItemEnum)).into()
}
