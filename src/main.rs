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
    let input = "let x = 5; print x;";
    
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    let instructions = compile_statements(ast);

    execute(instructions);

    println!("Rusty Runtime 🦀");
}