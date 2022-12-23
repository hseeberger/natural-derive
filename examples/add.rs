use natural_derive::Add;

#[derive(Debug, PartialEq, Eq, Add)]
struct Kelvin(u32);

fn main() {
    let kelvin = Kelvin(42) + Kelvin(1);
    assert_eq!(kelvin, Kelvin(43));
}
