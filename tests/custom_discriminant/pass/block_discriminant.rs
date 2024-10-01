use enum_macros::custom_discriminant;

#[custom_discriminant(u8)]
enum Foo {
    Bar = {
        let x = 0;
        let y = x + 1;
        y
    },
    Baz = 4,
}

fn main() {}
