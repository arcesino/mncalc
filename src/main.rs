use std::process;
use structopt::StructOpt;

fn main() {
    let config = mncalc::Config::from_args();

    if let Err(e) = mncalc::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
