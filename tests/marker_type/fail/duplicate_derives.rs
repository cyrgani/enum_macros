// The error output of this test will improve once https://github.com/rust-lang/rust/issues/131083 is resolved.
use enum_macros::marker_type;

#[marker_type(derive(Clone, Copy, Debug, Hash, PartialEq, Eq))]
enum Complex {
    A(bool),
    B { val: u8 },
}

fn main() {}
