use enum_macros::variant_amount;

#[variant_amount]
struct S {
    a: u32,
    b: u8,
}

#[variant_amount]
union U {
    a: u32,
    b: f32,
}

fn main() {}