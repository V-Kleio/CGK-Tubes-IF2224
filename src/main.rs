use std::env;

use crate::{dfa::Dfa, lexer::Lexer};

mod token;
mod dfa;
mod lexer;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_pascal_file", args[0]);
        return;
    }

    let filepath = &args[1];

    let dfa = match Dfa::from_file("dfa_rules.json") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error loading dfa_rules.json: {}", e);
            return;
        }
    };

    let source_code = match std::fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filepath, e);
            return;
        }
    };

    let mut lexer = Lexer::new(source_code, dfa);

    println!("---TOKENS---");
    while let Some(token) = lexer.get_next_token() {
        println!("{}", token);
    }
    println!("------------");
}
