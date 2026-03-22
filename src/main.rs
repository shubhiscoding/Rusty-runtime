mod lexer;
mod parser;
mod ast;
use lexer::tokenize;
use parser::Parser;


fn main() {
    let input = "let x = 5; print x;";
    
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    println!("{:#?}", ast);
    println!("Rusty Runtime 🦀");
}