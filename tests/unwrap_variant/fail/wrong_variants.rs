use enum_macros::UnwrapVariant;

#[derive(UnwrapVariant)]
#[allow(dead_code)]
enum Test {
    #[unwrap(ref)]
    A { v: u8 },
    #[unwrap(mut)]
    B,
    #[unwrap(ref, mut)]
    C(u8, u8),
    #[unwrap(ref)]
    D { },
}

fn main() {}