use enum_macros::marker_type;

#[marker_type]
enum Complex<T> {
    A(T),
    B { val: u8 },
}

struct F<T>(ComplexMarker<T>);

fn main() {}
