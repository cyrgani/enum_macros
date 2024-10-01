use enum_macros::UnwrapVariant;

#[derive(UnwrapVariant)]
struct S {
    #[unwrap(ref)]
    a: u32,
    b: u8,
}

#[derive(UnwrapVariant)]
union U {
    #[unwrap(ref, mut)]
    a: u32,
    b: f32,
}

fn main() {}