use enum_macros::custom_discriminant;

#[custom_discriminant(())]
enum Foo {
    Bar = 0,
    Baz = 1,
}

fn main() {}
