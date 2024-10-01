use enum_macros::custom_discriminant;

#[custom_discriminant(u8)]
enum Foo {
    Bar = f(),
    Baz = g(),
}

const fn f() -> u8 {
    0
}

const fn g() -> u8 {
    1
}

fn main() {}
