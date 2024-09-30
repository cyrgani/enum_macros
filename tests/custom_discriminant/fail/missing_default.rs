use enum_macros::custom_discriminant;

#[custom_discriminant(i64)]
enum Foo<T> {
    Bar(T) = 0,
    Baz([u8; 32]) = 1,
}

fn main() {}
