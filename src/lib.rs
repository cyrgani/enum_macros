//! A collection of useful macros that make working with enums easier.
#![no_std]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo, missing_docs)]
#![allow(clippy::needless_pass_by_value)]

extern crate alloc;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, ItemEnum};

mod utils;

#[cfg(feature = "custom_discriminant")]
mod custom_discriminant;

#[cfg(feature = "marker_type")]
mod marker_type;

#[cfg(feature = "next_variant")]
mod next_variant;

#[cfg(feature = "unwrap_variant")]
mod unwrap_variant;

#[cfg(feature = "variant_amount")]
mod variant_amount;

/// Adds support for custom enum "discriminants" of other types than `isize`.
///
/// Every enum variant must be given a discriminant.
/// This macro takes the type of the discriminant as argument.
///
/// This macro adds:
/// * `impl From<ENUM_TYPE> for DISCRIMINANT_TYPE`
/// * `impl TryFrom<DISCRIMINANT_TYPE> for ENUM_TYPE`
///
/// The discriminant may be a `const` item, but not a `static`.
/// It can also be the result of a `const fn` call, which will be evaluated
/// only once during compilation and then stored.
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
    custom_discriminant::custom_discriminant(attr.into(), parse_macro_input!(item as ItemEnum))
        .into()
}

/// Adds a new enum that has the same variants as this enum, but holds no data.
/// The new enum will be called `{OLD_NAME}Marker` and always implement
/// [`Debug`], [`Clone`], [`Copy`], [`PartialEq`], [`Eq`], [`Hash`].
///
/// Any other macros applied to the enum will not be invoked on the marker type.
/// Such atttributes can be given as an argument to `#[marker_type]` instead.
///
/// # Examples
/// Simple case:
/// ```
/// use enum_macros::marker_type;
///
/// #[marker_type]
/// enum Type<'a, T> {
///     A(&'a u32),
///     B(T),
///     C(&'a T),
/// }
///
/// /* generates:
/// #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
/// enum TypeMarker {
///     A,
///     B,
///     C,
/// }
/// */
/// ```
///
/// With arguments:
/// ```
/// use enum_macros::marker_type;
///
/// #[marker_type(derive(Ord), derive(PartialOrd))]
/// enum Example {
///     Int(u8),
///     Float(f32),
/// }
///
/// let int_marker = ExampleMarker::Int;
/// let float_marker = ExampleMarker::Float;
///
/// assert!(int_marker < float_marker);
/// assert!(float_marker > int_marker);
///
#[cfg(feature = "marker_type")]
#[proc_macro_attribute]
pub fn marker_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    marker_type::marker_type(
        parse_macro_input!(attr as marker_type::Arguments),
        parse_macro_input!(item as ItemEnum),
    )
    .into()
}

/// Adds a method for advancing to the next enum variant.
/// Wraps around at the last element.
///
/// For enum variants with data, every field must implement [`Default`].
///
/// # Examples
/// ```
/// use enum_macros::NextVariant;
///
/// #[derive(PartialEq, Debug, NextVariant)]
/// enum Example {
///     A,
///     B,
/// }
///
/// assert_eq!(Example::B, Example::A.next_variant());
/// assert_eq!(Example::A, Example::B.next_variant());
/// ```
#[cfg(feature = "next_variant")]
#[proc_macro_derive(NextVariant)]
pub fn next_variant(item: TokenStream) -> TokenStream {
    next_variant::next_variant(parse_macro_input!(item as ItemEnum)).into()
}

/// Adds methods for unwrapping variants of the enum.
///
/// Any variant that contains exactly one unnamed field can be annotated with the `unwrap` attribute.
/// There are two possible forms for it, which may be combined: `unwrap(ref)` and `unwrap(mut)`.
///
/// The generated method will require `&self` or `&mut self` and return `&T` or `&mut T`,
/// where `T` is the type of the single unnamed field.
///
/// Like `Option::unwrap`, `Result::unwrap` or `Result::unwrap_err`, this panics if the contained
/// variant is not existent.
///
/// # Examples
/// ```
/// use enum_macros::UnwrapVariant;
///
/// #[derive(UnwrapVariant)]
/// enum Test {
///     #[unwrap(ref, mut)]
///     A(String),
///     B(usize),
/// }
///
/// let mut test = Test::A(String::from("hello"));
/// assert_eq!(test.unwrap_a_ref(), "hello");
///     
/// test.unwrap_a_mut().push_str(" world");
/// assert_eq!(test.unwrap_a_ref(), "hello world");
/// ```
#[cfg(feature = "unwrap_variant")]
#[proc_macro_derive(UnwrapVariant, attributes(unwrap))]
pub fn unwrap_variant(item: TokenStream) -> TokenStream {
    unwrap_variant::unwrap_variant(parse_macro_input!(item as ItemEnum))
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Adds a constant storing the amount of variants the enum has to the enum.
///
/// # Examples
/// ```
/// use enum_macros::VariantAmount;
///
/// #[derive(VariantAmount)]
/// enum Example {
///     A,
///     B,
///     C,
/// }
///
/// assert_eq!(Example::VARIANT_AMOUNT, 3);
/// ```
#[cfg(feature = "variant_amount")]
#[proc_macro_derive(VariantAmount)]
pub fn variant_amount(item: TokenStream) -> TokenStream {
    variant_amount::variant_amount(parse_macro_input!(item as ItemEnum)).into()
}
