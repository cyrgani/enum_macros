use enum_macros::marker_type;

#[marker_type]
#[derive(Clone, Copy)]
enum Complex {
    A(bool),
    B { val: u8 },
}

fn main() {}
