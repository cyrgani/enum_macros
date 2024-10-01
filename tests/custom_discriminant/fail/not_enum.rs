use enum_macros::custom_discriminant;

#[custom_discriminant(u8)]
struct S {
    a: u32,
    b: u8,
}

#[custom_discriminant(u8)]
union U {
    a: u32,
    b: f32,
}

fn main() {}
