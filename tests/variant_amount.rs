use enum_macros::variant_amount;

#[test]
fn empty() {
    #[variant_amount]
    enum Void {}

    assert_eq!(Void::VARIANT_AMOUNT, 0);
}

#[test]
fn nonempty() {
    #[variant_amount]
    #[allow(dead_code)]
    enum Complex<T, S: Default, const N: usize>
    where
        T: Default,
        [T; N]: Default,
    {
        A { val: S },
        B([T; N]),
    }

    assert_eq!(Complex::<bool, Vec<i32>, 3>::VARIANT_AMOUNT, 2);
}
