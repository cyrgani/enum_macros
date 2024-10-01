use enum_macros::custom_discriminant;
use std::ptr::null;

static A: u8 = 0;
static B: u8 = 1;

#[custom_discriminant(*const u8)]
enum Foo {
    Bar = {
        let x = 4u8;
        x as *const u8
    },
    Baz = null(),
}

fn main() {}
