use enum_macros::custom_discriminant;

#[custom_discriminant(&'static str)]
#[derive(Debug, PartialEq)]
enum NoData {
    A = "A",
    B = "B",
}

fn main() {
    let a = NoData::try_from("A");
    assert_eq!(a, Ok(NoData::A));

    let b_disc1 = NoData::B.custom_discriminant();
    let b_disc2 = <&'static str>::from(NoData::B);
    assert_eq!(b_disc1, b_disc2);
    assert_eq!(b_disc1, "B");

    let err = NoData::try_from("b");
    assert_eq!(err, Err(()));
}
