use enum_macros::UnwrapVariant;

#[derive(UnwrapVariant)]
#[allow(dead_code)]
enum Test {
    #[unwrap(ref, mut)]
    A(u8),
}

fn main() {
    let mut a = Test::A(9);
    let x: &mut u8 = a.unwrap_a_mut();
    *x = 16;
    let y: &u8 = a.unwrap_a_ref();
    assert_eq!(*y, 16);
}
