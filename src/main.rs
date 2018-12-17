use structopt::StructOpt;

fn main() {
    let config = mncalc::Config::from_args();

    mncalc::run(config)
}
