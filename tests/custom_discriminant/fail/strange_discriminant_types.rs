use enum_macros::custom_discriminant;

#[custom_discriminant(impl Default)]
enum Foo1 {
    Bar,
    Baz,
}

#[custom_discriminant(dyn Default)]
enum Foo2 {
    Bar,
    Baz,
}

#[custom_discriminant(_)]
enum Foo3 {
    Bar,
    Baz,
}

#[custom_discriminant(!)]
enum Foo3 {
    Bar,
    Baz,
}


fn main() {}
