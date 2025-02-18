use enum_macros::custom_discriminant;

#[custom_discriminant(SelfRef)]
enum SelfRef {
    A = SelfRef::A,
    B = SelfRef::A,
}

fn main() {}
