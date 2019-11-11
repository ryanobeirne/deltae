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
    // Lab from a string
    let lab0 = LabValue::from_str("89.73, 1.88, -6.96")?;
    // Lab directly from values
    let lab1 = LabValue {
        l: 95.08,
        a: -0.17,
        b: -10.81,
    }.validate()?; // Validate that the values are in range

    // Calculate DeltaE between two lab values
    let de0 = DeltaE::new(&lab0, &lab1, DE2000);
    // Use the Delta trait
    let de1 = lab0.delta(lab1, DE2000);
    assert_eq!(de0, de1);

    // Convert to other color types
    let lch0 = LchValue::from(lab0);
    let xyz0 = XyzValue::from(lab1);
    // If DE2000 is less than 1.0, the colors are considered equivalent
    assert_eq!(lch0, lab0);
    assert_eq!(xyz0, lab1);

    // Calculate DeltaE between different color types
    let de2 = lch0.delta(xyz0, DE2000);
    assert_eq!(de2.round_to(4), de0.round_to(4));
    // There is some loss of accuracy in the conversion.
    // Usually rounding to 4 decimal places is more than enough.

    println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        lab0, // [L:89.73, a:1.88, b:-6.96]
        lab1, // [L:95.08, a:-0.17, b:-10.81]
        lch0, // [L:89.73, c:7.2094383, h:285.11572]
        xyz0, // [X:0.84574246, Y:0.8780792, Z:0.8542397]
        de0,  // 5.316941
        de1,  // 5.316941
        de2,  // 5.316937
    );

    Ok(())
}
```

---

## Binary

The binary included with this library is a command line application that
calculates Delta E between to Lab colors.

### Usage

```txt
deltae 0.2.1
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
