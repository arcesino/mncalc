# mncalc
Command line mixed numbers calculator written in Rust

## How to build
You need to have Rust installed in order to build the utility. Instalation instructions are available at: https://www.rust-lang.org/learn/get-started
Once installed, you only need to navigate to the root of the repo and run the following command:

```bash
$ cargo run --release
```

The command above will build the project with release optimizations & run the utility in REPL mode. Cargo also generates the program 
executable under `target/release/mncalc`.

## Run modes
This utility can work in 2 different modes:

### REPL mode
This is the default mode and makes the utility run in an interactive mode and allows the user to enter several expressions to be evaluated. 
The program will keep running until the user enters **'q'**. To run the utility in REPL mode you just need to execute the program without
specifying any options:

```bash
$ cargo run --release
```
or


```bash
# from the repo root
$ ./target/release/mncalc
```

Find below an example of the utility running in REPL mode:

```
$ target/release/mncalc
Starting repl mode. Type 'q' to quit

? 1/2 * 3_3/4
= 1_7/8
? 1/2
Error: Unparseable operation!
? 2_3/8 + 9/8
= 3_1/2
? 1_3/7 - 5_2/4
= -4_1/14
? 3/0 + 2_2/5
Error: Fraction with zero denominator!
? 7 / 3_1/4
= 2_2/13
? 25/13 / 0/11
Error: Division by zero!
? q
```

### Single evaluation mode
In this mode, the utility will evaluate an expression passed as a command line option. The program will terminate right after the given
expression gets evaluated. To run the utility in single evaluation mode you just need to execute the program specifying the expression 
to evaluate:


```bash
$ cargo run --release -- -e "1/2 * 3_3/4"
```
or


```bash
# from the repo root
$ ./target/release/mncalc --eval "1/2 * 3_3/4"
```

As you may've noticed, you can specify the expression to evaluate using short option (`-e`) or long option (`--eval`). For more details,
you can ask the utility for help:

```
$ target/release/mncalc -h
mncalc 0.1.0
Ignacio Arces <arcesino@gmail.com>
Simple Mixed Numbers Calculator

USAGE:
    mncalc [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --eval <expression>    The expression to evaluate
```
