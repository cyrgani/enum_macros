use enum_macros::custom_discriminant;

#[custom_discriminant([u8; 3])]
enum Foo {
    Bar = [0, 43, 12],
    Baz = [55; 3],
}

fn main() {}
