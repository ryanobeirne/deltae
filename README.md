# deltae
Calculates Delta E between to Lab colors

## Usage
```sh
deltae 89.73,1.88,-6.96 95.08,-0.17,-10.81
```

## Install
#### * Requires rustc and cargo to build
```sh
git clone https://github.com/robeirne/deltae
cd deltae
cargo build && cargo install
```

## Notes:
Only calculates de2000 and de1976 at the moment. The default is de2000.

### TODO:
- Add de1994, deCMC1, deCMC2 delta E methods
