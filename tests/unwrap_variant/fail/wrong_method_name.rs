use enum_macros::UnwrapVariant;

#[derive(UnwrapVariant)]
enum Test {
    #[unwrap(mut)]
    A(String),
    #[unwrap(ref)]
    B(usize),
}

fn main() {
    let _a: &String = Test::unwrap_A_ref();
    let _b: &mut usize = Test::unwrap_B_mut();
}
