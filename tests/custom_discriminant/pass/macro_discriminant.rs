use enum_macros::custom_discriminant;

macro_rules! get_type {
    () => {
        u8
    };
}

#[custom_discriminant(get_type! {})]
enum Foo {
    Bar = 0,
    Baz = 1,
}

fn main() {
    let foo = Foo::Baz;
    assert_eq!(foo.custom_discriminant(), 1);
}
