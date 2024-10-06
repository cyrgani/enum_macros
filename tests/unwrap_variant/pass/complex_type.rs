use enum_macros::UnwrapVariant;

#[derive(UnwrapVariant)]
#[allow(dead_code)]
enum Test<'a, T> {
    #[unwrap(ref, mut)]
    A(&'a mut T),
    B,
}

fn main() {}
