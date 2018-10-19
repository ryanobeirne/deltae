# deltae

## Library

A rust library for interacting with and manipulating Lab and Lch colors and calculating DeltaE (color difference).

Check out the documentation here:
[Rust API Documentation](https://robeirne.github.io/deltae)

...or compile it yourself:

```sh
cargo doc --open
```

### Examples

```rust
extern crate deltae;
use deltae::{DeltaE, DEMethod::DE2000};
use deltae::color::LabValue;

fn main() {
    let lab0 = LabValue::from("89.73, 1.88, -6.96").unwrap();
    let lab1 = LabValue {
        l: 95.08,
        a: -0.17,
        b: -10.81,
    };

    println!("{}", lab0); // [L:89.73, a:1.88, b:-6.96]

    let de0 = DeltaE::new(&lab0, &lab1, DE2000).round_to(4);

    println!("{}: {}", de0.method, de0.value); // DE2000: 5.3169

    let de1 = DeltaE::from(
        "89.73, 1.88, -6.96",
        "95.08, -0.17, -10.81",
        "DE2000"
    ).unwrap();

    assert_eq!(de0, de1.round_to(4));

    let lch0 = lab0.to_lch();
    let lab2 = lch0.to_lab();

    println!("{}", lch0); // [L:89.73, c:7.2094, h:285.1157]

    assert_eq!(lab0.round_to(4), lab2.round_to(4));
}
```

---

## Binary

The binary included with this library is a command line application that calculates Delta E between to Lab colors.

### Usage

```sh
deltae [--method=<DE Method>] <L,a,b,> <L,a,b>
```

### Example

```sh
deltae --method=de1976 "89.73, 1.88, -6.96" "95.08, -0.17, -10.81"
```

### Install

#### Requires rustc and cargo to build

```sh
git clone https://github.com/robeirne/deltae
cd deltae
cargo install
```

### Notes

Calculates DE2000, DE1994 (Graphic Arts only), and DE1976. The Default is DE2000.

### TODO

- Add DE1994 (textiles), DECMC1, DECMC2 DEMethods
