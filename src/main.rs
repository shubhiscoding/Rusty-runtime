mod lexer;
mod parser;
mod ast;
use lexer::tokenize;


fn main() {
    let input = "let x     = 523;";
    let tokens = tokenize(input);

    println!("{:#?}", tokens);
    println!("Rusty Runtime 🦀");
}