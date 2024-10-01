use enum_macros::custom_discriminant;

#[custom_discriminant(fn(u8) -> i32)]
enum Foo1 {
    Bar = f,
    Baz = g,
}

fn f(x: u8) -> i32 {
    x as i32
}

fn g(x: u8) -> i32 {
    (x as i32) * 2
}

fn main() {}
