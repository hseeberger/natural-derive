use natural_derive::*;

#[derive(
    Debug,
    PartialEq,
    Eq,
    New,
    Inner,
    From,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    MulScalar,
    MulAssignScalar,
    Div,
    DivAssign,
    DivScalar,
    DivAssignScalar,
    Rem,
    RemAssign,
    RemScalar,
    RemAssignScalar,
)]
struct SomeUnit(u32);

#[test]
fn test_new() {
    assert_eq!(SomeUnit::new(42), SomeUnit(42));
}

#[test]
fn test_inner() {
    assert_eq!(*SomeUnit::new(42).inner(), 42);
}

#[test]
fn test_from() {
    let kelvin: SomeUnit = 42.into();
    assert_eq!(kelvin, SomeUnit(42));
}

#[test]
fn test_add() {
    let kelvin = SomeUnit(42) + SomeUnit(1);
    assert_eq!(kelvin, SomeUnit(43));
}

#[test]
fn test_add_assign() {
    let mut kelvin = SomeUnit(42);
    kelvin += SomeUnit(1);
    assert_eq!(kelvin, SomeUnit(43));
}

#[test]
fn test_sub() {
    let kelvin = SomeUnit(42) - SomeUnit(42);
    assert_eq!(kelvin, SomeUnit(0));
}

#[test]
fn test_sub_assign() {
    let mut kelvin = SomeUnit(42);
    kelvin -= SomeUnit(42);
    assert_eq!(kelvin, SomeUnit(0));
}

#[test]
fn test_mul() {
    let kelvin = SomeUnit(42) * SomeUnit(2);
    assert_eq!(kelvin, SomeUnit(84));
}

#[test]
fn test_mul_assign() {
    let mut kelvin = SomeUnit(42);
    kelvin *= SomeUnit(2);
    assert_eq!(kelvin, SomeUnit(84));
}

#[test]
fn test_mul_scalar() {
    let kelvin = SomeUnit(42) * 2;
    assert_eq!(kelvin, SomeUnit(84));
}

#[test]
fn test_mul_assign_scalar() {
    let mut kelvin = SomeUnit(42);
    kelvin *= 2;
    assert_eq!(kelvin, SomeUnit(84));
}

#[test]
fn test_div() {
    let kelvin = SomeUnit(42) / SomeUnit(2);
    assert_eq!(kelvin, SomeUnit(21));
}

#[test]
fn test_div_assign() {
    let mut kelvin = SomeUnit(42);
    kelvin /= SomeUnit(2);
    assert_eq!(kelvin, SomeUnit(21));
}

#[test]
fn test_div_scalar() {
    let kelvin = SomeUnit(42) / 2;
    assert_eq!(kelvin, SomeUnit(21));
}

#[test]
fn test_div_assign_scalar() {
    let mut kelvin = SomeUnit(42);
    kelvin /= 2;
    assert_eq!(kelvin, SomeUnit(21));
}

#[test]
fn test_rem() {
    let kelvin = SomeUnit(42) % SomeUnit(2);
    assert_eq!(kelvin, SomeUnit(0));
}

#[test]
fn test_rem_assign() {
    let mut kelvin = SomeUnit(42);
    kelvin %= SomeUnit(2);
    assert_eq!(kelvin, SomeUnit(0));
}

#[test]
fn test_rem_scalar() {
    let kelvin = SomeUnit(42) % 2;
    assert_eq!(kelvin, SomeUnit(0));
}

#[test]
fn test_rem_assign_scalar() {
    let mut kelvin = SomeUnit(42);
    kelvin %= 2;
    assert_eq!(kelvin, SomeUnit(0));
}
