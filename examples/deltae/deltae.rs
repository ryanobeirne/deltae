use deltae::DeltaE;

mod cli;

fn main() {
    //Parse command line arguments with clap
    let matches = cli::app().get_matches();

    let method = matches.value_of("METHOD").unwrap();
    let color0 = matches.value_of("COLOR0").unwrap();
    let color1 = matches.value_of("COLOR1").unwrap();

    match DeltaE::from(color0, color1, method) {
        Ok(de) => println!("{}: {}", de.method, de),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    };
}
