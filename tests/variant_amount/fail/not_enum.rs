use enum_macros::VariantAmount;

#[derive(VariantAmount)]
struct S {
    a: u32,
    b: u8,
}

#[derive(VariantAmount)]
union U {
    a: u32,
    b: f32,
}

fn main() {}