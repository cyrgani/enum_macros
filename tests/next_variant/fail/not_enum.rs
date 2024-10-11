use enum_macros::NextVariant;

#[derive(NextVariant)]
struct S {
    a: u32,
    b: u8,
}

#[derive(NextVariant)]
union U {
    a: u32,
    b: f32,
}

fn main() {}