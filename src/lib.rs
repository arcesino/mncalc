use std::io;
use std::io::prelude::*;
use std::process;

use structopt::StructOpt;

mod operation;
mod mixed_number;
mod fraction;
mod math;

#[derive(Debug, StructOpt)]
#[structopt(name = "mncalc", about = "Simple Mixed Numbers Calculator")]
pub struct Config {
    #[structopt(short = "e", long = "eval", help = "The expression to evaluate")]
    expression: Option<String>
}

pub fn run(config: Config) {
    match config.expression {
        Some(expression) => run_single_evaluation(&expression),
        None => run_repl_evaluation()
    }
}

fn run_single_evaluation(expression: &str) {
    let result = evaluate_expression(&expression);
    if result.is_err() {
        process::exit(1);
    }
}

fn evaluate_expression(expression: &str) -> Result<(), &str> {
    let result = operation::parse_operation(expression);
    match result {
        Ok(operation) => process_operation(operation),
        Err(e) => log_and_propagate_error(e)
    }
}

fn process_operation(operation: operation::Operation) -> Result<(), &'static str> {
    match operation.compute() {
        Ok(result) => println!("= {}", result),
        Err(e) => log_and_propagate_error(e)?
    }

    Ok(())
}

fn log_and_propagate_error(error: &str) -> Result<(), &'static str> {
    eprintln!("Error: {}", error);
    
    Err("Application error!")
}

fn run_repl_evaluation() {
    println!("Starting repl mode. Type 'q' to quit\n");

    loop {
        print!("? ");
        io::stdout().flush().ok().expect("Unable to flush stdout!");

        let mut expression = String::new();
        io::stdin()
            .read_line(&mut expression)
            .expect("Failed to read expression");

        if expression.trim() == "q" {
            break;
        }

        evaluate_expression(&expression).ok();
    }
}
