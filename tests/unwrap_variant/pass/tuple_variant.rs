use enum_macros::UnwrapVariant;

#[derive(UnwrapVariant)]
#[allow(dead_code)]
enum Test {
    #[unwrap(ref)]
    A(u8),
    #[unwrap(mut)]
    B((u8, String)),
    #[unwrap(ref, mut)]
    C(()),
}

fn main() {}
