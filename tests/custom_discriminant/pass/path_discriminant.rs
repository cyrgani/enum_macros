use enum_macros::custom_discriminant;

#[custom_discriminant(core::primitive::usize)]
enum Foo {
    Bar = 1usize,
    Baz = usize::MAX,
}

fn main() {}
