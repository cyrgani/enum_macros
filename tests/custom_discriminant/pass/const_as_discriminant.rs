use enum_macros::custom_discriminant;

const BAR: u8 = 0;
const BAZ: u8 = 1;

#[custom_discriminant(u8)]
#[derive(Debug, PartialEq)]
enum Foo {
    Bar = BAR,
    Baz = BAZ,
}

fn main() {
    assert_eq!(Foo::Bar.custom_discriminant(), BAR);
    assert_eq!(Foo::Baz.custom_discriminant(), BAZ);
    assert_eq!(Foo::try_from(BAR), Ok(Foo::Bar));
    assert_eq!(Foo::try_from(BAZ), Ok(Foo::Baz));
    assert_eq!(Foo::try_from(0), Ok(Foo::Bar));
    assert_eq!(Foo::try_from(2), Err(()));
}
