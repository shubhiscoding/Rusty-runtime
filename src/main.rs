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
    let input = "
    let x = (2 + (3 * (4+2)));
    let b = -2-2;
    print x;
    print (2 + 3) * 4;
    print -(2 + 3);
    print (2 + 3) * (4 + 1);
";
    
    let tokens = tokenize(input);

    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    let instructions = compile_statements(ast);

    execute(instructions);

    println!("Rusty Runtime 🦀");
}