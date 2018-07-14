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

    if h < 0.0 {
        h = 360.0 - h.abs();
    };

    LchValue {
        l: color.l,
        c: ( color.a.powi(2) + color.b.powi(2) ).sqrt(),
        h: h % 360.0,
    }
}

fn delta_h_calc(h0: &f64, h1: &f64) -> f64 {
    if h0 - h1 <= 180.0 {
        h1 - h0
    } else {
        if h1 <= h0 {
            h1 - h0 + 360.0
        } else {
            h1 - h0 - 360.0
        }
    }
}

fn avg_h_calc(h0: &f64, h1:&f64) -> f64 {
    if h0 - h1 > 180.0 {
        (h0 + h1 + 360.0) / 2.0
    } else {
        (h0 + h1) / 2.0
    }
}

//This is pretty solid, I think
fn delta_e_1976(c0: &LabValue, c1: &LabValue) -> f64 {
    ( (c0.l - c1.l).powi(2) + (c0.a - c1.a).powi(2) + (c0.b - c1.b).powi(2) ).sqrt()
}

fn delta_e_2000(c0: &LabValue, c1:&LabValue) -> f64 {
    let lch_0 = lab_to_lch(&c0);
    let lch_1 = lab_to_lch(&c1);
    let l_avg = (lch_0.l + lch_1.l) / 2.0;
    let c_avg = (lch_0.c + lch_1.c) / 2.0;
    let h_avg = avg_h_calc(&lch_0.h, &lch_1.h);
    let t = 1.0 - 0.17*((    h_avg - 30.0)).to_radians().cos()
                + 0.24*((2.0*h_avg         )).to_radians().cos()
                + 0.32*((3.0*h_avg +  6.0)).to_radians().cos()
                - 0.20*((4.0*h_avg - 63.0)).to_radians().cos();
    let s_l = 1.0 + (
                        (0.015 * (l_avg - 50.0).powi(2))
                      / ( 20.0 + (l_avg - 50.0).powi(2)).sqrt()
                      );
    let s_c = 1.0 + 0.045*c_avg;
    let s_h = 1.0 + 0.015*c_avg*t;
    let r_t = -2.0*((c_avg.powi(7)/(c_avg.powi(7) + 25_f64.powi(7))).sqrt());
    let delta_l = lch_1.l - lch_0.l;
    let delta_c = lch_1.c - lch_0.c;
    let delta_h = 2.0*(lch_0.c*lch_1.c).sqrt()*(delta_h_calc(&lch_0.h, &lch_1.h)/2.0).sin();

    println!("r_t:\t{}", r_t);
    println!("t:\t{}", t);
    println!("s_l:\t{}", s_l);
    println!("s_c:\t{}", s_c);
    println!("s_h:\t{}", s_h);

    //Return de2000
    ( (delta_l/s_l).powi(2)
    + (delta_c/s_c).powi(2)
    + (delta_h/s_h).powi(2)
    + r_t*(delta_c/s_c)*(delta_h/s_h)
    ).sqrt()
}

//fn delta_e_CMC1(c1: &LabValue, c2:&LabValue) -> f64 {
    //math
//}

//fn delta_e_CMC2(c1: &LabValue, c2:&LabValue) -> f64 {
    //math
//}

//Decide which delta e method to use and use it
fn de_by_method(c0: &LabValue, c1: &LabValue, method: &str) -> f64 {
    match method {
        "de1976" | "de76" | "DE1976" | "DE76" | "1976" | "76" => delta_e_1976(&c0, &c1),
        "de2000" | "de00" | "DE2000" | "DE00" | "2000" | "00" => delta_e_2000(&c0, &c1),
        //"deCMC1" => delta_e_CMC1(&color0, &color1),
        //"deCMC2" => delta_e_CMC2(&color0, &color1),
        _ => {
          eprintln!("'{}' is not a valid Delta E method. Using de1976.", method);
          delta_e_1976(&c0, &c1)
        },
    }
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
    if  split[0] < 0.0    || split[0] > 100.0 ||
        split[1] < -128.0 || split[1] > 128.0 ||
        split[2] < -128.0 || split[2] > 128.0 {
            eprintln!("Bad Lab values: {}", lab_string);
            eprintln!("\tL: 0..100\n\ta: -128..128\n\tb: -128..128");
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

    //Parse the arguments into LabValues
    let color0 = string_to_lab( &String::from( matches.value_of("COLOR0").unwrap() ) );
    let color1 = string_to_lab( &String::from( matches.value_of("COLOR1").unwrap() ) );

    //Calculate and format dE to 2 decimal places
    let delta_e = format!("{:.*}", 4, de_by_method(&color0, &color1, &arg_method));
    println!("{}", delta_e);

}

