use enum_macros::marker_type;

#[marker_type(derive(Clone, Copy, Debug, Hash, PartialEq, Eq))]
enum Complex {
    A(bool),
    B { val: u8 },
}

fn main() {}
