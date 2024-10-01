use enum_macros::next_variant;

#[next_variant]
#[derive(PartialEq, Debug)]
enum One {
    A(u8),
}

fn main() {
    let one = One::A(2);
    let next = one.next_variant();
    let nextnext = next.next_variant();
    assert_eq!(next, nextnext);
    assert_eq!(next, One::A(0));
    assert_ne!(one, next);
}
