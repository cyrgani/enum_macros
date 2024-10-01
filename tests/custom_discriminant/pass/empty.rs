use enum_macros::custom_discriminant;

#[custom_discriminant(())]
#[derive(Debug, PartialEq)]
enum Empty {}

fn main() {
    let empty = Empty::try_from(());
    assert_eq!(empty, Err(()));
}
