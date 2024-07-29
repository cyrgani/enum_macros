use enum_macros::next_variant;

#[test]
fn once() {
    #[next_variant]
    #[derive(PartialEq, Debug)]
    enum One {
        A(u8),
    }

    let one = One::A(2);
    let next = one.next_variant();
    let nextnext = next.next_variant();
    assert_eq!(next, nextnext);
    assert_eq!(next, One::A(0));
    assert_ne!(one, next);
}

#[test]
fn multiple() {
    #[next_variant]
    #[derive(PartialEq, Debug)]
    enum Complex<T, S: Default, const N: usize>
    where
        T: Default,
        [T; N]: Default,
    {
        A { val: S },
        B([T; N]),
    }
    type C = Complex<bool, Vec<i32>, 3>;

    let a = C::A { val: vec![3] };
    let next = a.next_variant();
    let nextnext = next.next_variant();
    assert_eq!(next, C::B([false, false, false]));
    assert_eq!(nextnext, Complex::A { val: vec![] });
    assert_ne!(a, nextnext);
}
