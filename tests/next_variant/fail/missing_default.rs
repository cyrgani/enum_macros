use enum_macros::next_variant;

#[next_variant]
enum Complex<T> {
    A { val: T },
    B(u8),
}

fn main() {}
