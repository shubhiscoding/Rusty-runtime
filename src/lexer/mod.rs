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
    Division,
    LeftParentheses,
    RightParentheses,
    Less,
    Greater,
    EqualEqual,
    NotEqual,
    If,
    LeftBrace,
    RightBrace,
    Else,
    While,
    Break,
    Continue,
    And,
    Or
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
                chars.next();
                if let Some(&ch) = chars.peek() {
                    if ch == '=' {
                        tokens.push(Token::EqualEqual);
                        chars.next();
                    } else {
                        tokens.push(Token::Equals);
                    }
                } else {
                    panic!("Unexpected character: {}", ch);
                }
            }

            '<' => {
                tokens.push(Token::Less);
                chars.next();
            }

            '>' => {
                tokens.push(Token::Greater);
                chars.next();
            }

            '!' => {
                chars.next();
                if let Some(&ch) = chars.peek() {
                    if ch == '=' {
                        tokens.push(Token::NotEqual);
                        chars.next();
                    } else {
                        panic!("Unexpected character: {}", ch);
                    }
                } else {
                    panic!("Unexpected character: {}", ch);
                }
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

            '(' => {
                tokens.push(Token::LeftParentheses);
                chars.next();
            }

            ')' => {
                tokens.push(Token::RightParentheses);
                chars.next();
            }

            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();
            }

            '{' => {
                tokens.push(Token::LeftBrace);
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
            },

            '&' => {
                chars.next();
                if let Some(&ch) = chars.peek() {
                    if ch == '&' {
                        tokens.push(Token::And);
                        chars.next();
                    } else {
                        panic!("Unexpected character: {}", ch);
                    }
                } else {
                    panic!("Unexpected character: {}", ch);
                }
            },

            '|' => {
                chars.next();
                if let Some(&ch) = chars.peek() {
                    if ch == '|' {
                        tokens.push(Token::Or);
                        chars.next();
                    } else {
                        panic!("Unexpected character: {}", ch);
                    }
                } else {
                    panic!("Unexpected character: {}", ch);
                }
            },
            
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
                    "if" => tokens.push(Token::If),
                    "else" => tokens.push(Token::Else),
                    "while" => tokens.push(Token::While),
                    "break" => tokens.push(Token::Break),
                    "continue" => tokens.push(Token::Continue),
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