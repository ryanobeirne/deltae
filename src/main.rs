#[macro_use]
extern crate clap;
extern crate deltae;

use deltae::*;

fn main() {
    //Parse command line arguments with clap
    let matches = clap_app!(deltae =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg METHOD: -m --method +takes_value "Sets delta E method (1976, 1994, 2000). Default is DE2000")
        (@arg COLOR0: +required "Lab values for reference color: (98.08,-0.17,-10.81)")
        (@arg COLOR1: +required "Lab values for comparison color: (89.73,1.88,-6.96)")
    ).get_matches();

    let method = matches.value_of("METHOD").unwrap_or("DE2000");
    let color0 = matches.value_of("COLOR0").unwrap();
    let color1 = matches.value_of("COLOR1").unwrap();

    let delta_e = DeltaE::from(color0, color1, method);

    match delta_e {
        Ok(de) => println!("{}", de.round_to(4).value),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    };
}
