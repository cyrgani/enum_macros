use enum_macros::marker_type;

fn require_copy<T: Copy>() {}

#[test]
fn test() {
    #[marker_type]
    #[derive(PartialEq, Debug)]
    #[allow(dead_code)]
    enum Complex<T, S: Default, const N: usize>
    where
        T: Default,
        [T; N]: Default,
    {
        A { val: S },
        B([T; N]),
    }

    require_copy::<ComplexMarker>();
    assert_eq!(ComplexMarker::A, ComplexMarker::A);
}
