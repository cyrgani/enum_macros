use enum_macros::custom_discriminant;

#[custom_discriminant(str)]
#[derive(Debug, PartialEq)]
enum Foo {
    A = "a",
    B = "b",
}

fn main() {
    let s = String::from("a");
    let f = Foo::try_from(s.as_str()).unwrap();
    assert_eq!(f, Foo::A);

    assert_eq!(f.custom_discriminant(), "a");
}
