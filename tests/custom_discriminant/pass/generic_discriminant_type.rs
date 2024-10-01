use enum_macros::custom_discriminant;

#[custom_discriminant(Option<u8>)]
enum Foo {
    Bar = Some(0),
    Baz = None,
}

fn main() {
    let foo = Foo::Bar;
    assert_eq!(foo.custom_discriminant(), Some(0));
}
