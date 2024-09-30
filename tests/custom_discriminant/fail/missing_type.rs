use enum_macros::custom_discriminant;

#[custom_discriminant]
enum Foo {
    Bar,
    Baz(u8),
    Thing { foo: u8 },
}

fn main() {}
