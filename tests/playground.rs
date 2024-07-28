//! Experimental testing for `enum_macros`.
#![allow(unused_imports)]
use enum_macros::*;

#[variant_amount]
#[next_variant]
#[custom_discriminant(&'static str)]
#[allow(dead_code)]
#[marker_type]
pub(crate) enum X<'a, T: Clone>
where
    T: Copy + Default,
    &'a T: Default,
{
    A(T) = "f",
    B(&'a str) = "gsdf",
    C = "dgml",
    D {} = "kgs",
    E { a: &'a str, b: &'static str } = "13",
    F((&'a T)) = "ffffffffff",
}

#[variant_amount]
#[custom_discriminant(&'static str)]
#[next_variant]
#[allow(dead_code)]
#[marker_type]
pub(crate) enum Void {}

#[marker_type]
#[allow(dead_code)]
enum Y {
    A(u32),
    B,
    C { d: (), _e: u8 },
}

#[cfg(not_working_yet)]
mod custom_discriminant_with_generic_type {
    #[custom_discriminant(W<'a, T>)]
    enum F {
        A = W::new(),
    }

    struct W<'a, T: Copy>
    where
        T: Default,
    {
        a: &'a T,
    }

    impl<'a, T: Copy> W<'a, T>
    where
        T: Default,
    {
        const fn new() -> Self {
            loop {}
        }
    }

    struct WW<'a, T: Copy>
    where
        T: Default,
    {
        w: W<'a, T>,
    }
}
