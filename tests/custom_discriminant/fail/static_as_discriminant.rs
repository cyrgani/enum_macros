use enum_macros::custom_discriminant;

static BAR: u8 = 0;
static BAZ: u8 = 1;

#[custom_discriminant(u8)]
enum Foo {
    Bar = BAR,
    Baz = BAZ,
}

fn main() {
    let foo = Foo::Bar;
    assert_eq!(foo.custom_discriminant(), BAR);
}
