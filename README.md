# deltae

## Library

A rust library for interacting with and manipulating Lab and Lch colors and calculating DeltaE (color difference).

---

[Rust API Documentation](https://robeirne.github.io/deltae)

### Examples

```rust
extern crate deltae;
use deltae::*;

fn main() {
    let de = DeltaE::parse("50,0,0", "50,0,0", "DE2000").unwrap();
    println!("Delta E 2000: {}", de.round_to(4).value );
}
```

## Binary

The binary included with this library is a command line application that calculates Delta E between to Lab colors.

---

## Usage

```sh
deltae [--method=<DE Method>] <L,a,b,> <L,a,b>
```

### Example

```sh
deltae --method=de1976 89.73,1.88,-6.96 95.08,-0.17,-10.81
```

## Install

### * Requires rustc and cargo to build

```sh
git clone https://github.com/robeirne/deltae
cd deltae
cargo install
```

## Notes

Calculates DE2000, DE1994 (Graphic Arts only), and DE1976. The Default is DE2000.

### TODO

- Add DE1994 (textiles), DECMC1, DECMC2 DEMethods
