## 0.0.3
- changed `UnwrapVariant`-generated method names to `snake_case`
- stop applying macro invocations after `#[marker_type]` to the generated marker type
- derive `Debug`, `PartialEq`, `Eq` and `Hash` for generated marker types
- allow passing arguments in `#[marker_type]` invocations that will be applied to the marker type
- removed `#[variant_amount]` and `next_variant` in favor of the new `VariantAmount` and `NextVariant` derive macros
- removed `std` feature, the entire crate is `no_std` compatible anyway

## 0.0.2
- added `UnwrapVariant` and improved error output

## 0.0.1
- added `custom_discriminant`, `marker_type`, `next_variant` and `variant_amount`
