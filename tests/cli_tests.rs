use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions

#[test]
fn command_help_shows_usage_and_options() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::main_binary()?;

    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("Simple Mixed Numbers Calculator"))
        .stdout(predicate::str::contains("-e, --eval <expression>    The expression to evaluate"));

    Ok(())
}

#[test]
fn run_with_short_eval_arg() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    cmd.arg("-e")
        .arg("1/2 * 3_3/4");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("= 1_7/8"));

    Ok(())
}

#[test]
fn run_with_long_eval_arg() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    cmd.arg("--eval")
        .arg("2_3/8 + 9/8");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("= 3_1/2"));

    Ok(())
}

#[test]
fn run_with_unparseable_expression_prints_error() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    cmd.arg("-e")
        .arg("1 / 2 - 3_3/4");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Unparseable operation!"));

    Ok(())
}

#[test]
fn run_with_fraction_with_zero_denominator_prints_error() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    cmd.arg("-e")
        .arg("1/0 / 3_3/4");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Fraction with zero denominator!"));

    Ok(())
}

#[test]
fn run_with_no_args_start_repl_mode() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::main_binary()?;

    cmd.with_stdin()
        .buffer("1/2 * 3_3/4\n2_3/8 + 9/8\nq")
        .assert()
        .success()
        .stdout(predicate::str::contains("Starting repl mode. Type 'q' to quit"))
        .stdout(predicate::str::contains("= 1_7/8"))
        .stdout(predicate::str::contains("= 3_1/2"));

    Ok(())
}

#[test]
fn run_repl_mode_with_errors() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::main_binary()?;

    cmd.with_stdin()
        .buffer("5/2\n1/2 * 3_3/4\n2_1/2 + 3_2/0\n2_3/8 + 9/8\nq")
        .assert()
        .success()
        .stdout(predicate::str::contains("Starting repl mode. Type 'q' to quit"))
        .stderr(predicate::str::contains("Error: Unparseable operation!"))
        .stdout(predicate::str::contains("= 1_7/8"))
        .stderr(predicate::str::contains("Error: Fraction with zero denominator!"))
        .stdout(predicate::str::contains("= 3_1/2"));

    Ok(())
}
