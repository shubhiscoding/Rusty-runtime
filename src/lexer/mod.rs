#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Identifier(String),
    Number(i32),
    Equals,
    Semicolon,
    Addition,
    Subtraction,
    Multiplication,
    Division
}


pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\n' | '\t' => {
                chars.next();
            }

            '=' => {
                tokens.push(Token::Equals);
                chars.next();
            }

            '+' => {
                tokens.push(Token::Addition);
                chars.next();
            }

            '-' => {
                tokens.push(Token::Subtraction);
                chars.next();
            }

            '*' => {
                tokens.push(Token::Multiplication);
                chars.next();
            }

            '/' => {
                tokens.push(Token::Division);
                chars.next();
            }

            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }

            '0'..='9' => {
                let mut num = String::new();

                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        num.push(digit);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Number(num.parse().unwrap()));
            }

            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();

                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match ident.as_str() {
                    "let" => tokens.push(Token::Let),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }

            _ => {
                panic!("Unexpected character: {}", ch);
            }
        }
    }

    tokens
}