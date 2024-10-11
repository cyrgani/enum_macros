use enum_macros::NextVariant;

#[derive(PartialEq, Debug, NextVariant)]
enum Complex<T, S: Default, const N: usize>
where
    T: Default,
    [T; N]: Default,
{
    A { val: S },
    B([T; N]),
}

type C = Complex<bool, Vec<i32>, 3>;

fn main() {
    let a = C::A { val: vec![3] };
    let next = a.next_variant();
    let nextnext = next.next_variant();
    assert_eq!(next, C::B([false, false, false]));
    assert_eq!(nextnext, Complex::A { val: vec![] });
    assert_ne!(a, nextnext);
}
