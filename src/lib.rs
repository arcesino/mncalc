use std::io;
use std::io::prelude::*;
use std::process;

mod operation;
mod mixed_number;
mod fraction;
mod math;

/// Single evaluation mode evaluates the given expression and terminates
pub fn run_single_evaluation(expression: &str) {
    let result = evaluate_expression(&expression);
    if result.is_err() {
        process::exit(1);
    }
}

/// REPL evaluation runs in a loop than terminates only when the user enters 'q'
pub fn run_repl_evaluation() {
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

fn evaluate_expression(expression: &str) -> Result<(), &str> {
    let result = operation::Operation::parse_operation(expression);
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
