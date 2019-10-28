use clap::{App, Arg, crate_version, crate_description, crate_authors};
use deltae::color::LabValue;
use std::str::FromStr;

pub fn app() -> App<'static, 'static> {
    App::new("deltae")
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("METHOD")
            .help("Set DeltaE method")
            .long("method")
            .short("m")
            .default_value("DE2000")
            .possible_values(&["DE2000", "DECMC1", "DECMC2", "DE1994", "DE1994T", "DE1976"])
            .takes_value(true))
        .arg(Arg::with_name("COLOR0")
            .help("Lab values for reference color")
            .required(true)
            .validator(validate_lab))
        .arg(Arg::with_name("COLOR1")
            .help("Lab values for reference color")
            .required(true)
            .validator(validate_lab))
}

fn validate_lab(s: String) -> Result<(), String> {
    match LabValue::from_str(&s) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}
