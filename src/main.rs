use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::{dfa::Dfa, lexer::Lexer, parser::Parser, semantic_analyzer::SemanticAnalyzer};

mod ast;
mod dfa;
mod lexer;
mod node;
mod parser;
mod semantic_analyzer;
mod semantic_error;
mod symbol_table;
mod token;
mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <path_to_pascal_file> <pathtooutput>", args[0]);
        return;
    }

    let filepath = &args[1];
    let pathtooutput = &args[2];

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
    let mut tokens = Vec::new();

    while let Some(token) = lexer.get_next_token() {
        tokens.push(token);
    }

    println!("---TOKENS---");
    for token in &tokens {
        println!("{}", token);
    }
    println!("------------");

    let file = match File::create(pathtooutput) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error output file {}: {}", pathtooutput, e);
            return;
        }
    };
    let mut writer = BufWriter::new(file);

    writeln!(writer, "---TOKENS---").unwrap();
    for token in &tokens {
        writeln!(writer, "{}", token).unwrap();
    }
    writeln!(writer, "------------").unwrap();

    println!("\nParsing...");

    let mut parser = Parser::new(tokens);

    let parse_tree_result = parser.parse();

    match parse_tree_result {
        Ok(node) => {
            println!("\n---PARSE TREE---");
            println!("{}", node);
            println!("--------------");

            writeln!(writer, "\n---PARSE TREE---").unwrap();
            writeln!(writer, "{}", node).unwrap();
            writeln!(writer, "--------------").unwrap();

            // Semantic Analysis
            println!("\nPerforming semantic analysis...");
            let mut analyzer = SemanticAnalyzer::new();
            
            match analyzer.analyze(&node) {
                Ok(ast) => {
                    println!("\n---SEMANTIC ANALYSIS---");
                    println!("{}", analyzer.symbol_table);
                    println!("\n---DECORATED AST---");
                    println!("{}", ast);
                    println!("--------------");

                    writeln!(writer, "\n---SEMANTIC ANALYSIS---").unwrap();
                    writeln!(writer, "{}", analyzer.symbol_table).unwrap();
                    writeln!(writer, "\n---DECORATED AST---").unwrap();
                    writeln!(writer, "{}", ast).unwrap();
                    writeln!(writer, "--------------").unwrap();

                    println!("\nSuccessfully analyzed and wrote to {}", pathtooutput);
                }
                Err(errors) => {
                    eprintln!("\n---SEMANTIC ERRORS---");
                    for error in &errors {
                        eprintln!("{}", error);
                    }
                    eprintln!("------------------");

                    writeln!(writer, "\n---SEMANTIC ERRORS---").unwrap();
                    for error in &errors {
                        writeln!(writer, "{}", error).unwrap();
                    }
                    writeln!(writer, "------------------").unwrap();

                    println!("\nSemantic analysis completed with {} error(s). Output written to {}", 
                             errors.len(), pathtooutput);
                }
            }
        }
        Err(e) => {
            eprintln!("\n---PARSER ERROR---");
            eprintln!("{}", e);
            eprintln!("------------------");

            writeln!(writer, "\n---PARSER ERROR---").unwrap();
            writeln!(writer, "{}", e).unwrap();
            writeln!(writer, "------------------").unwrap();
        }
    }

    writer.flush().unwrap();
}
