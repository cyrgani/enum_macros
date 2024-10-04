use enum_macros::UnwrapVariant;

#[derive(UnwrapVariant)]
enum Test {
    #[unwrap(ref)]
    A(String),
    B(usize),
}

fn main() {
    let _a = Test::unwrap_a_ref(1);
}
