use structopt::StructOpt;

/// This struct models the command line options
#[derive(Debug, StructOpt)]
#[structopt(name = "mncalc", about = "Simple Mixed Numbers Calculator")]
pub struct Config {
    #[structopt(short = "e", long = "eval", help = "The expression to evaluate")]
    expression: Option<String>
}

/// The program can run in 2 modes: single evaluation & repl
/// Single mode is run if an expression is provided through command line option
/// REPL mode is run if no options are provided
fn main() {
    let config = Config::from_args();

    match config.expression {
        Some(expression) => mncalc::run_single_evaluation(&expression),
        None => mncalc::run_repl_evaluation()
    }
}
