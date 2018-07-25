# deltae
Calculates Delta E between to Lab colors

## Usage
```sh
deltae [--method=<DE Method>] <L,a,b,> <L,a,b>
```
#### Example
```sh
deltae --method=de1976 89.73,1.88,-6.96 95.08,-0.17,-10.81
```

## Install
#### * Requires rustc and cargo to build
```sh
git clone https://github.com/robeirne/deltae
cd deltae
cargo build && cargo install
```

## Notes:
Calculates de2000, de1994 (Graphic Arts only), and de1976. The default is de2000.

### TODO:
- Add de1994, deCMC1, deCMC2 delta E methods
