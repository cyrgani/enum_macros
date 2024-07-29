use enum_macros::custom_discriminant;

#[test]
fn no_data() {
    #[custom_discriminant(&'static str)]
    #[derive(Debug, PartialEq)]
    enum NoData {
        A = "A",
        B = "B",
    }

    let a = NoData::try_from("A");
    assert_eq!(a, Ok(NoData::A));

    let b_disc1 = NoData::B.custom_discriminant();
    let b_disc2 = <&'static str>::from(NoData::B);
    assert_eq!(b_disc1, b_disc2);
    assert_eq!(b_disc1, "B");

    let err = NoData::try_from("b");
    assert_eq!(err, Err(()));
}

#[test]
fn with_data() {
    #[custom_discriminant(&'static str)]
    #[derive(Debug, PartialEq)]
    enum WithData {
        A(u8) = "A",
        B([bool; 2]) = "B",
    }

    let a = WithData::try_from("A");
    assert_eq!(a, Ok(WithData::A(u8::default())));

    let b_disc1 = WithData::B([true, false]).custom_discriminant();
    let b_disc2 = <&'static str>::from(WithData::B([false, false]));
    assert_eq!(b_disc1, b_disc2);
    assert_eq!(b_disc1, "B");

    let err = WithData::try_from("b");
    assert_eq!(err, Err(()));
}

#[test]
fn with_data_generics_bounds_and_more() {
    #[custom_discriminant(&'static str)]
    #[derive(Debug, PartialEq)]
    enum Complex<T, S: Default, const N: usize>
    where
        T: Default,
        [T; N]: Default,
    {
        A { val: S } = "A",
        B([T; N]) = "B",
    }

    type C = Complex<bool, Vec<i32>, 3>;

    let a = C::try_from("A");
    assert_eq!(a, Ok(C::A { val: vec![] }));

    let b_disc1 = C::B([true, false, true]).custom_discriminant();
    let b_disc2 = <&'static str>::from(C::B([false, false, true]));
    assert_eq!(b_disc1, b_disc2);
    assert_eq!(b_disc1, "B");

    let err = C::try_from("b");
    assert_eq!(err, Err(()));
}
