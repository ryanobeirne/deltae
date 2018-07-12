#[macro_use]
extern crate clap;

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

//Not in use yet, but will be necessary for de2000
fn lab_to_lch(color: &LabValue) -> LchValue {
    let mut h: f64 = color.b.atan2(color.a).to_degrees();

    if h < 0_f64 {
        h = 360_f64 - h.abs();
    };

    LchValue {
        l: color.l,
        c: ( color.a.powi(2) + color.b.powi(2) ).sqrt(),
        h: h % 360_f64,
    }
}

//This is pretty solid, I think
fn delta_e_1976(c1: &LabValue, c2: &LabValue) -> f64 {
    ( (c1.l - c2.l).powi(2) + (c1.a - c2.a).powi(2) + (c1.b - c2.b).powi(2) ).sqrt()
}

//Convert and validate strings to LabValue
fn string_to_lab(lab_string: &String) -> LabValue {
    //Split string into Vec by comma (,)
    let split = lab_string.split(",").filter_map(|s| s.parse::<f64>().ok()).collect::<Vec<_>>();
    //Validate that there are only 3 values
    if split.len() != 3 {
        eprintln!("Bad Lab values format: '{}'.\n\tUse 'L,a,b'", lab_string);
        std::process::exit(1);
    };
    //Check that the Lab values are in the proper range or exit
    if  split[0] < 0_f64    || split[0] > 100_f64 ||
        split[1] < -128_f64 || split[1] > 128_f64 ||
        split[2] < -128_f64 || split[2] > 128_f64 {
            eprintln!("Bad Lab values: {}\n\tL: 0..100\n\ta: -128..128\n\tb: -128..128", lab_string);
            std::process::exit(1);
        }
    //Return the LabValue
    LabValue {
        l: split[0],
        a: split[1],
        b: split[2],
    }
}

fn main() {
    //Parse command line arguments with clap
    let matches = clap_app!(deltae =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg METHOD: -m --method +takes_value "Sets delta E method (1976, 2000, CMC1, CMC2). Default is dE1976")
        (@arg COLOR0: +required "Lab values for reference color: (98.08,-0.17,-10.81)")
        (@arg COLOR1: +required "Lab values for comparison color: (89.73,1.88,-6.96)")
    ).get_matches();

    //Select the desired dE method or use de1976 by default
    let arg_method = matches.value_of("METHOD").unwrap_or("de1976");
    eprintln!("Delta E Method: {}", arg_method);

    //Decide which delta e method to use
    fn de_by_method(color0: &LabValue, color1: &LabValue, method: &str) -> f64 {
        match method {
            "de1976" => delta_e_1976(&color0, &color1),
            //"de2000",
            //"deCMC1",
            //"deCMC2",
            _ => {
              eprintln!("'{}' is not a valid Delta E method. Using de1976.", method);
              delta_e_1976(&color0, &color1)
            },
        }
    }

    //Parse the arguments into LabValues
    let color0 = string_to_lab( &String::from( matches.value_of("COLOR0").unwrap() ) );
    let color1 = string_to_lab( &String::from( matches.value_of("COLOR1").unwrap() ) );

    //Calculate and format dE to 2 decimal places
    let delta_e = format!("{:.*}", 2, de_by_method(&color0, &color1, &arg_method));
    println!("{}", delta_e);

}

