use enum_macros::variant_amount;

#[variant_amount]
enum Void {}

fn main() {
    assert_eq!(Void::VARIANT_AMOUNT, 0);
}
