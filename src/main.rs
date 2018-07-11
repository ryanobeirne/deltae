
#[derive(Debug)]
struct LabValue {
    l: f64,
    a: f64,
    b: f64,
}

#[derive(Debug)]
struct LchValue {
    l: f64,
    c: f64,
    h: f64,
}

fn lab_to_lch(color: &LabValue) -> LchValue {
    LchValue {
        l: color.l,
        c: ( color.a.powi(2) + color.b.powi(2) ).sqrt(),
        h: color.b,        
    }
}

fn delta_e_1976(c1: &LabValue, c2: &LabValue) -> f64 {
    ( (c1.l - c2.l).powi(2) + (c1.a - c2.a).powi(2) + (c1.b - c2.b).powi(2) ).sqrt()
}

fn main() {

    let color0 = LabValue {
        l: 95.08,
        a: -0.17,
        b: -10.81,
    };

    let color1 = LabValue {
        l: 89.73,
        a: 1.88,
        b: -6.96,
    };

    let color2 = LabValue {
        l: 50.0,
        a: 100.0,
        b: 100.0,
    };

    let delta_e = delta_e_1976(&color0, &color1);
    println!("{:?}", delta_e);

    let lch = lab_to_lch(&color2);
    println!("{:?}", lch);

}

