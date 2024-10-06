use enum_macros::VariantAmount;

#[derive(VariantAmount)]
#[allow(dead_code)]
enum Complex<T, S: Default, const N: usize>
where
    T: Default,
    [T; N]: Default,
{
    A { val: S },
    B([T; N]),
}

fn main() {
    assert_eq!(Complex::<bool, Vec<i32>, 3>::VARIANT_AMOUNT, 2);
}
