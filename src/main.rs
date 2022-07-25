mod lexer;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => println!("Usage: [script]"),
    }

    process::exit(0);
}

pub fn run_file(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap();
    let tokens = lexer::logos_lexer::lex(filename, &file_contents).unwrap();
    for t in tokens {
        println!("{:?}", t.token_type);
    }
}

pub fn run_prompt() {}
