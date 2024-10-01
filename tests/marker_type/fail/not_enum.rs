use enum_macros::marker_type;

#[marker_type]
struct S {
    a: u32,
    b: u8,
}

#[marker_type]
union U {
    a: u32,
    b: f32,
}

fn main() {}