use enum_macros::VariantAmount;

#[derive(VariantAmount)]
enum Void {}

fn main() {
    assert_eq!(Void::VARIANT_AMOUNT, 0);
}
