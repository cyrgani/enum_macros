use enum_macros::UnwrapVariant;

#[derive(UnwrapVariant)]
#[allow(dead_code)]
enum Test<T> {
    #[unwrap(ref, mut)]
    A(T),
    B,
}

fn main() {
    let mut test = Test::A(String::from("hello"));
    assert_eq!(test.unwrap_a_ref(), "hello");

    test.unwrap_a_mut().push_str(" world");
    assert_eq!(test.unwrap_a_ref(), "hello world");
}
