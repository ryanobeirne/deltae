#[macro_use]
extern crate clap;
extern crate deltae;

use deltae::*;


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
    let arg_color0 = matches.value_of("COLOR0").unwrap();
    let arg_color1 = matches.value_of("COLOR1").unwrap();

    //Parse the arguments into LabValues
    let color0 = string_to_lab(arg_color0).unwrap();
    let color1 = string_to_lab(arg_color1).unwrap();

    //Calculate and format dE to 4 decimal places
    let delta_e = format!("{:.*}", 4, de_by_method(&color0, &color1, &arg_method));
    println!("{}", delta_e);

}
