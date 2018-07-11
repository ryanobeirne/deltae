
#[derive(Debug)]
struct LabValue {
    L: f64,
    a: f64,
    b: f64,
}

fn delta_e_1976(c1: LabValue, c2: LabValue) -> f64 {
    ( (c1.L - c2.L).exp2() + (c1.a - c2.a).exp2() + (c1.b - c2.b).exp2() ).sqrt()
}

fn main() {

    let color0 = LabValue {
        L: 95.08,
        a: -0.17,
        b: -10.81,
    };

    let color1 = LabValue {
        L: 89.73,
        a: 1.88,
        b: -6.96,
    };

    let delta_e = delta_e_1976(color0, color1);

    println!("{:?}", delta_e);

}

