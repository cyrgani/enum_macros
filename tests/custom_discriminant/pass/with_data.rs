use enum_macros::custom_discriminant;

#[custom_discriminant(&'static str)]
#[derive(Debug, PartialEq)]
enum WithData {
    A(u8) = "A",
    B([bool; 2]) = "B",
}

fn main() {
    let a = WithData::try_from("A");
    assert_eq!(a, Ok(WithData::A(u8::default())));

    let b_disc1 = WithData::B([true, false]).custom_discriminant();
    let b_disc2 = <&'static str>::from(WithData::B([false, false]));
    assert_eq!(b_disc1, b_disc2);
    assert_eq!(b_disc1, "B");

    let err = WithData::try_from("b");
    assert_eq!(err, Err(()));
}
