use enum_macros::UnwrapVariant;

#[derive(UnwrapVariant)]
#[allow(dead_code)]
enum Test {
    #[unwrap(xyz)]
    A(String),
    B(usize),
}

fn main() {}