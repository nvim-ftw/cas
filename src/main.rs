#![warn(clippy::todo)]
use cas;
use untwine::{parser, parser_repl};

fn main() {
    println!("Welcome to a (simple) Computer Algebra System.");
    cas::start_repl();
}
