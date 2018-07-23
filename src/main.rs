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

//Convert Lab to Lch. Not in use now, but may be helpful soon
fn lab_to_lch(color: &LabValue) -> LchValue {
    let mut h: f64 = color.b.atan2(color.a).to_degrees();

    if h < 0.0 {
        h += 360.0;
    };

    LchValue {
        l: color.l,
        c: ( color.a.powi(2) + color.b.powi(2) ).sqrt(),
        h: h % 360.0,
    }
}

//This is pretty solid, I think
fn delta_e_1976(c0: &LabValue, c1: &LabValue) -> f64 {
    ( (c0.l - c1.l).powi(2) + (c0.a - c1.a).powi(2) + (c0.b - c1.b).powi(2) ).sqrt()
}

//This is ridiculously complicated
fn delta_e_2000(c0: &LabValue, c1:&LabValue) -> f64 {
    let l_bar_prime = (c0.l + c1.l)/2.0;
    let c_0 = (c0.a.powi(2) + c0.b.powi(2)).sqrt();
    let c_1 = (c1.a.powi(2) + c1.b.powi(2)).sqrt();
    let c_bar = (c_0 + c_1) / 2.0;
    let g = 0.5 * (1.0 - ( c_bar.powi(7) / (c_bar.powi(7) + 25_f64.powi(7)) ).sqrt());
    let a_prime_0 = c0.a * (1.0 + g);
    let a_prime_1 = c1.a * (1.0 + g);
    let c_prime_0 = (a_prime_0.powi(2) + c0.b.powi(2)).sqrt();
    let c_prime_1 = (a_prime_1.powi(2) + c1.b.powi(2)).sqrt();
    let c_bar_prime = (c_prime_0 + c_prime_1) / 2.0;
    
    //Hue calculations have to account for degrees: 360 == 0
    let mut h_prime_0 = c0.b.atan2(a_prime_0).to_degrees();
    if h_prime_0 < 0.0 {
        h_prime_0 += 360.0;
    };

    let mut h_prime_1 = c1.b.atan2(a_prime_1).to_degrees();
    if h_prime_1 < 0.0 {
        h_prime_1 += 360.0;
    };

    let mut h_bar_prime = h_prime_0 - h_prime_1;
    if h_bar_prime > 180.0 {
        h_bar_prime = (h_prime_0 + h_prime_1 + 360.0) / 2.0;
    } else {
        h_bar_prime = (h_prime_0 + h_prime_1) / 2.0;
    };

    let t = 1.0 - 0.17*(    h_bar_prime - 30.0).to_radians().cos()
                + 0.24*(2.0*h_bar_prime       ).to_radians().cos()
                + 0.32*(3.0*h_bar_prime +  6.0).to_radians().cos()
                - 0.20*(4.0*h_bar_prime - 63.0).to_radians().cos();
    
    let mut delta_h = (h_prime_1 - h_prime_0).abs();
    if delta_h > 180.0 && h_prime_1 <= h_prime_0 {
        delta_h += 360.0;
    } else if delta_h > 180.0 {
        delta_h -= 360.0;
    };

    let delta_l_prime = c1.l - c0.l;
    let delta_c_prime = c_prime_1 - c_prime_0;
    let delta_h_prime = 2.0 * (c_prime_0 * c_prime_1).sqrt() * (delta_h.to_radians() / 2.0).sin();

    let s_l = 1.0 + (
              (0.015 * (l_bar_prime - 50.0).powi(2))
            / (20.00 + (l_bar_prime - 50.0).powi(2)).sqrt()
        );
    let s_c = 1.0 + 0.045 * c_bar_prime;
    let s_h = 1.0 + 0.015 * c_bar_prime * t;
    
    let delta_theta = 30.0 * (-((h_bar_prime - 275.0)/25.0).powi(2)).exp();
    let r_c =  2.0 * (c_bar_prime.powi(7)/(c_bar_prime.powi(7) + 25_f64.powi(7))).sqrt();
    let r_t = -(r_c * (2.0 * delta_theta.to_radians()).sin());

    let k_l = 1.0;
    let k_c = 1.0;
    let k_h = 1.0;

    //Return the Delta E 2000
    (
        (delta_l_prime/(k_l*s_l)).powi(2)
      + (delta_c_prime/(k_c*s_c)).powi(2)
      + (delta_h_prime/(k_h*s_h)).powi(2)
      + (r_t * (delta_c_prime/(k_c*s_c)) * (delta_h_prime/(k_h*s_h)))
    ).sqrt()
}

fn delta_e_1994(c0: &LabValue, c1: &LabValue) -> f64 {
    let delta_l = c0.l - c1.l;
    let chroma_0 = (c0.a.powi(2) + c0.b.powi(2)).sqrt();
    let chroma_1 = (c1.a.powi(2) + c1.b.powi(2)).sqrt();
    let delta_chroma = chroma_0 - chroma_1;
    let delta_a = c0.a - c1.a;
    let delta_b = c0.b - c1.b;
    let delta_hue = (delta_a.powi(2) + delta_b.powi(2) - delta_chroma.powi(2)).sqrt();
    let s_l = 1.0;
    let s_c = 1.0 + 0.045 * chroma_0;
    let s_h = 1.0 + 0.015 * chroma_0;

      ( (delta_l / s_l).powi(2)
      + (delta_chroma / s_c).powi(2)
      + (delta_hue / s_h).powi(2)
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
        "de1994" | "de94" | "DE1994" | "DE94" | "1994" | "94" => delta_e_1994(&c0, &c1),
        //"deCMC1" => delta_e_CMC1(&color0, &color1),
        //"deCMC2" => delta_e_CMC2(&color0, &color1),
        _ => {
          eprintln!("'{}' is not a valid Delta E method. Using de2000.", method);
          delta_e_2000(&c0, &c1)
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
        (@arg METHOD: -m --method +takes_value "Sets delta E method (1976, 2000, CMC1, CMC2). Default is dE2000")
        (@arg COLOR0: +required "Lab values for reference color: (98.08,-0.17,-10.81)")
        (@arg COLOR1: +required "Lab values for comparison color: (89.73,1.88,-6.96)")
    ).get_matches();

    //Select the desired dE method or use de2000 by default
    let arg_method = matches.value_of("METHOD").unwrap_or("de2000");

    //Parse the arguments into LabValues
    let color0 = string_to_lab( &String::from( matches.value_of("COLOR0").unwrap() ) );
    let color1 = string_to_lab( &String::from( matches.value_of("COLOR1").unwrap() ) );

    //Calculate and format dE to 2 decimal places
    let delta_e = format!("{:.*}", 2, de_by_method(&color0, &color1, &arg_method));
    println!("{}", delta_e);

}

