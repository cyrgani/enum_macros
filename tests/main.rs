#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/*/pass/*.rs");
    t.compile_fail("tests/*/fail/*.rs");
    t.pass("tests/other/macro_combinations.rs");
}
