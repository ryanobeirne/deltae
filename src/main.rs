#[macro_use]
extern crate clap;
extern crate deltae;

use deltae::*;
use deltae::color::LabValue;

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

    // Select the desired dE method or use de2000 by default
    let arg_method = matches.value_of("METHOD").unwrap_or("de2000");
    let arg_color0 = matches.value_of("COLOR0").unwrap();
    let arg_color1 = matches.value_of("COLOR1").unwrap();

    //Parse the arguments into LabValues
    let lab_0 = LabValue::from(arg_color0).unwrap();
    let lab_1 = LabValue::from(arg_color1).unwrap();

    // Calculate and format dE to 4 decimal places
    let de_method = DEMethod::from(&arg_method);
    let delta_e = DeltaE::new(&lab_0, &lab_1, de_method);

    // Round to 4 places and print
    println!("{}", delta_e.round_to(4).value);
}
