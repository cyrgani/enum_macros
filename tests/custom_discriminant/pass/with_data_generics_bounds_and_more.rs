use enum_macros::custom_discriminant;

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

fn main() {
    let a = C::try_from("A");
    assert_eq!(a, Ok(C::A { val: vec![] }));

    let b_disc1 = C::B([true, false, true]).custom_discriminant();
    let b_disc2 = <&'static str>::from(C::B([false, false, true]));
    assert_eq!(b_disc1, b_disc2);
    assert_eq!(b_disc1, "B");

    let err = C::try_from("b");
    assert_eq!(err, Err(()));
}
