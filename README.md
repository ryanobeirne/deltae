# deltae

## Library

A rust library for converting colors and calculating
[DeltaE](http://www.colorwiki.com/wiki/Delta_E:_The_Color_Difference) (color
difference).

Check out the documentation here:
[Rust API Documentation](https://ryanobeirne.github.io/deltae)

...or compile it yourself:

```sh
cargo doc --open
```

### Examples

```rust
use std::error::Error;
use std::str::FromStr;
use deltae::*;

fn main() -> Result<(), Box<dyn Error>>{
    let lab0 = LabValue::from_str("89.73, 1.88, -6.96").unwrap();
    let lab1 = LabValue {
        l: 95.08,
        a: -0.17,
        b: -10.81,
    }.validate()?;

    println!("{}", &lab0); // [L:89.73, a:1.88, b:-6.96]

    let de0 = DeltaE::new(&lab0, &lab1, DE2000).round_to(4);

    println!("{}: {}", de0.method, de0.value); // DE2000: 5.3169

    let lch0 = LchValue::from(lab0);
    println!("{}", lch0.round_to(4)); // [L:89.73, c:7.2094, h:285.1157]
    let lab2 = LabValue::from(lch0);
    println!("{}", lab0.round_to(4)); // [L:89.73, c:1.88, h:-6.96]
    assert_eq!(lch0, lab2);

    let de1 = lch0.delta(lab1, DE2000);
    assert_eq!(de0, de1.round_to(4));


    assert_eq!(lab0.round_to(4), lab2.round_to(4));

    Ok(())
}
```

---

## Binary

The binary included with this library is a command line application that
calculates Delta E between to Lab colors.

### Usage

```txt
deltae 0.2.0
Ryan O'Beirne <ryanobeirne@gmail.com>
Calculate Delta E between two colors in CIE Lab space.

USAGE:
    deltae [OPTIONS] <COLOR0> <COLOR1>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --color-type <COLORTYPE>    Set color type [default: lab]  [possible values: lab, lch, xyz]
    -m, --method <METHOD>           Set DeltaE method [default: 2000]  [possible values: 2000, 1994, 1994T, CMC1, CMC2,
                                    1976]

ARGS:
    <COLOR0>    Reference color values
    <COLOR1>    Sample color values
```

### Example

```sh
deltae --method=de1976 "89.73, 1.88, -6.96" "95.08, -0.17, -10.81"
```

### Install

```sh
git clone https://github.com/ryanobeirne/deltae
cd deltae
cargo install --example=deltae --path=. --force
```

### Notes

Calculates DE2000, DE1994 (Graphic Arts and Textiles), DECMC (with a tolerance
for lightness and chroma), and DE1976. The Default is DE2000.
