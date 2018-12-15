use std::error::Error;
use structopt::StructOpt;

mod operation;

#[derive(Debug, StructOpt)]
#[structopt(name = "mncalc", about = "Simple Mixed Numbers Calculator")]
pub struct Config {
    #[structopt(short = "e", long = "eval", help = "The expression to evaluate")]
    expression: Option<String>
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.expression {
        Some(expression) => evaluate_expression(expression.trim()),
        None => println!("Starting repl mode. Type 'q' to quit\n")
    }
    Ok(())
}

pub fn evaluate_expression(expression: &str) {
    let operation = operation::parse_operation(expression);
    let result = operation.compute();

    println!("= {}", result);
}
