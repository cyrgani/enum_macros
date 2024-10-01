#![allow(unused_parens)]
use enum_macros::custom_discriminant;

#[custom_discriminant((((((u8))))))]
enum Foo {
    Bar = (((((0))))),
    Baz = (1),
}

fn main() {
    let foo = Foo::Baz;
    assert_eq!(foo.custom_discriminant(), 1);
}
