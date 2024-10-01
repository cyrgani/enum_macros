use enum_macros::custom_discriminant;

#[custom_discriminant(W<'_>)]
enum F {
    A = VALUE,
}

const VALUE: W<'_> = W { a: &1 };

#[derive(PartialEq)]
struct W<'a> {
    a: &'a u32,
}

fn main() {}