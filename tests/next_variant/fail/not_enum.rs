use enum_macros::next_variant;

#[next_variant]
struct S {
    a: u32,
    b: u8,
}

#[next_variant]
union U {
    a: u32,
    b: f32,
}

fn main() {}