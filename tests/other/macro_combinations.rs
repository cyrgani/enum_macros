#![allow(unused_imports)]
use enum_macros::*;

#[variant_amount]
#[next_variant]
#[derive(UnwrapVariant)]
#[custom_discriminant(&'static str)]
#[allow(dead_code)]
#[marker_type]
pub(crate) enum X<'a, T: Clone>
where
    T: Copy + Default,
    &'a T: Default,
{
    #[unwrap(mut)]
    A(T) = "f",
    #[unwrap(ref, mut)]
    B(&'a str) = "gsdf",
    C = "dgml",
    D {} = "kgs",
    E { a: &'a str, b: &'static str } = "13",
    #[unwrap(ref)]
    F(&'a T) = "ffffffffff",
}

#[variant_amount]
#[custom_discriminant(&'static str)]
#[next_variant]
#[derive(UnwrapVariant)]
#[allow(dead_code)]
#[marker_type]
pub(crate) enum Void {}

fn main() {}
