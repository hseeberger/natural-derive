# natural-derive

Proc macros for naturally deriving basic trait impls for new types. Natural means that the impl for the outer type just delegates to an impl of the inner type.

## Example

```rust
use natural_derive::Add;

#[derive(Debug, PartialEq, Eq, Add)]
struct Kelvin(u32);

fn main() {
    let kelvin = Kelvin(42) + Kelvin(1);
    assert_eq!(kelvin, Kelvin(43));
}
```

## Contribution policy ##

Contributions via GitHub pull requests are gladly accepted from their original author. Along with
any pull requests, please state that the contribution is your original work and that you license the
work to the project under the project's open source license. Whether or not you state this
explicitly, by submitting any copyrighted material via pull request, email, or other means you agree
to license the material under the project's open source license and warrant that you have the legal
authority to do so.

## License ##

This code is open source software licensed under the
[Apache 2.0 License]("http://www.apache.org/licenses/LICENSE-2.0.html").
