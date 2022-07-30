mod common;
mod gc;
mod interpreter;
mod lexer;
mod parser;

use rustyline::Editor;
use std::env;
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
    run(filename, &file_contents);
}

pub fn run(filename: &str, contents: &str) {
    let tokens = lexer::logos_lexer::lex(filename, contents).unwrap();
    for t in tokens {
        println!("{:?}", t.token_type);
    }
}

pub fn run_prompt() {
    let mut rl = Editor::<()>::new().unwrap();

    let mut unit = String::new();
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let line = line.trim_end();
                if let Some(line) = line.strip_suffix("\\") {
                    unit.push_str(line);
                } else {
                    unit.clear();
                    run("Input", line);
                }
            }
            Err(err) => {
                println!("console error: {}", err);
                std::process::exit(0);
            }
        }
    }
}
