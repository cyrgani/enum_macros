use enum_macros::custom_discriminant;

#[custom_discriminant(u8)]
enum Foo {
    Bar = f(),
    Baz = f(),
}

const fn f() -> u8 {
    0
}

fn main() {}
