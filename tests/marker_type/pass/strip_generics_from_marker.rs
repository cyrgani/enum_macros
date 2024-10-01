use enum_macros::marker_type;

#[marker_type]
enum Complex<T> {
    A(T),
    B { val: u8 },
}

struct F(ComplexMarker);

fn main() {}
