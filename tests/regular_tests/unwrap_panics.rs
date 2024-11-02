use enum_macros::UnwrapVariant;

#[derive(UnwrapVariant)]
#[allow(dead_code)]
enum Test {
    #[unwrap(ref, mut)]
    A(String),
    #[unwrap(mut, ref)]
    B(usize),
}

#[test]
#[should_panic(expected = "called `Test::unwrap_b_ref()` on a value of a different variant")]
fn unwrap_1() {
    let x = Test::A(String::from("x"));
    x.unwrap_b_ref();
}

#[test]
#[should_panic(expected = "called `Test::unwrap_a_mut()` on a value of a different variant")]
fn unwrap_2() {
    let mut x = Test::B(0);
    x.unwrap_a_mut();
}
