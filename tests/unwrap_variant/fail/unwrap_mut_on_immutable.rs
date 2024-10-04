use enum_macros::UnwrapVariant;

#[derive(UnwrapVariant)]
#[allow(dead_code)]
enum Test {
    #[unwrap(ref, mut)]
    A(String),
    #[unwrap(mut, ref)]
    B(usize),
}

fn main() {
    let x = Test::A(String::from("x"));
    x.unwrap_a_mut().push('r');
    assert_eq!(x.unwrap_a_ref(), "rx");
}
