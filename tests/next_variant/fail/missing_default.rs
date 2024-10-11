use enum_macros::NextVariant;

#[derive(NextVariant)]
enum Complex<T> {
    A { val: T },
    B(u8),
}

fn main() {}
