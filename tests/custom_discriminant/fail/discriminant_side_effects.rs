static mut X: u8 = 0;
const fn f() -> u8 {
    unsafe {
        let old = X;
        X += 1;
        old
    }
}

#[enum_macros::custom_discriminant(u8)]
#[derive(Debug, PartialEq)]
enum F {
    A = f(),
    B = f(),
}

fn main() {
    assert_eq!(F::A.custom_discriminant(), 0);
    assert_eq!(F::B.custom_discriminant(), 1);
    assert_eq!(F::A.custom_discriminant(), 0);
    assert_eq!(F::B.custom_discriminant(), 1);
    assert_eq!(F::try_from(2), Err(()));
    assert_eq!(F::try_from(0), Ok(F::A));
    assert_eq!(F::try_from(0), Ok(F::A));
    assert_eq!(F::try_from(0), Ok(F::A));
}