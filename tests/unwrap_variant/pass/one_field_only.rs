use enum_macros::UnwrapVariant;

#[derive(UnwrapVariant)]
#[allow(dead_code)]
enum Test {
    #[unwrap(ref, mut)]
    A(String),
    B,
}

fn main() {
    let mut test = Test::A(String::from("hello"));
    assert_eq!(test.unwrap_A_ref(), "hello");

    test.unwrap_A_mut().push_str(" world");
    assert_eq!(test.unwrap_A_ref(), "hello world");
}
