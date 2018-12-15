use std::process;
use structopt::StructOpt;

#[macro_use] 
extern crate lazy_static;

fn main() {
    let config = mncalc::Config::from_args();

    if let Err(e) = mncalc::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
