use enum_macros::marker_type;

#[marker_type(derive(Ord), derive(PartialOrd))]
enum Example {
    Int(u8),
    Float(f32),
}

fn main() {
    let int_marker = ExampleMarker::Int;
    let float_marker = ExampleMarker::Float;

    assert!(int_marker < float_marker);
    assert!(float_marker > int_marker);
}
