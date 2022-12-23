use natural_derive::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Add, AddAssign, Sub, SubAssign)]
struct Kelvin(u32);

#[test]
fn test_add() {
    let kelvin = Kelvin(42) + Kelvin(1);
    assert_eq!(kelvin, Kelvin(43));
}

#[test]
fn test_add_assign() {
    let mut kelvin = Kelvin(42);
    kelvin += Kelvin(1);
    assert_eq!(kelvin, Kelvin(43));
}

#[test]
fn test_sub() {
    let kelvin = Kelvin(42) - Kelvin(42);
    assert_eq!(kelvin, Kelvin(0));
}

#[test]
fn test_sub_assign() {
    let mut kelvin = Kelvin(42);
    kelvin -= Kelvin(42);
    assert_eq!(kelvin, Kelvin(0));
}