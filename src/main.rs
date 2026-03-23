use std::env::{args};

mod lexer;
mod parser;
mod ast;
mod compiler;
mod vm;
use lexer::tokenize;
use parser::Parser;
use compiler::compile_statements;
use vm::execute;
fn main() {
    let run_args = args().collect::<Vec<String>>();
    if run_args.len() < 2 {
        println!("Usage: {} <source_file>", run_args[0]);
        return;
    }
    let command = run_args[1].as_str();
    if command == "--version" || command == "-v" {
        println!("Rusty Runtime 🦀 v{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if command == "run" {
        if run_args.len() < 3 {
            println!("Usage: {} run <source_file>", run_args[0]);
            return;
        }
        let filename = &run_args[2];
        let source = std::fs::read_to_string(filename).expect("Failed to read source file");
        let tokens = tokenize(&source);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        let instructions = compile_statements(ast);
        let mut runtime = vm::Runtime::new();
        execute(instructions, &mut runtime);
    } else {
        println!("Unknown command: {}", command);
        println!("Usage: {} run <source_file>", run_args[0]);
    }
}