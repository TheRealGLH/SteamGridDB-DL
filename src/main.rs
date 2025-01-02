use std::env;
use std::process;
fn main() {
    let config = steamgriddb_dl::Configuration::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        steamgriddb_dl::print_help();
        process::exit(10);
    });
    match steamgriddb_dl::run(config) {
        Ok(_) => (),
        Err(status_code) => {
            process::exit(status_code);
        },
    }
}
