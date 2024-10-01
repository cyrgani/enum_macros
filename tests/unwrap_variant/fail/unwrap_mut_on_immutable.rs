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
    x.unwrap_A_mut().push('r');
    assert_eq!(x.unwrap_A_ref(), "rx");
}
