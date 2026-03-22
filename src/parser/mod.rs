use crate::lexer::Token;
use crate::ast::{Statement, Expression};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }
}

impl Parser {
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while self.pos < self.tokens.len() {
            statements.push(self.parse_statement());
        }

        statements
    }
}

impl Parser {
    fn parse_statement(&mut self) -> Statement {
        match self.peek() {
            Some(Token::Let) => self.parse_var_decl(),
            Some(Token::Identifier(name)) if name == "print" => self.parse_print(),
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
    }
}

impl Parser {
    fn parse_var_decl(&mut self) -> Statement {
        self.advance(); // consume 'let'

        let name = match self.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => panic!("Expected identifier"),
        };

        match self.advance() {
            Some(Token::Equals) => {}
            _ => panic!("Expected '='"),
        }

        let value = self.parse_expression();

        match self.advance() {
            Some(Token::Semicolon) => {}
            _ => panic!("Expected ';'"),
        }

        Statement::VarDecl { name, value }
    }
}

impl Parser {
    fn parse_print(&mut self) -> Statement {
        self.advance(); // consume 'print'

        let value = self.parse_expression();

        match self.advance() {
            Some(Token::Semicolon) => {}
            _ => panic!("Expected ';'"),
        }

        Statement::Print { value }
    }
}

impl Parser {
    fn parse_expression(&mut self) -> Expression {
        match self.advance() {
            Some(Token::Number(n)) => Expression::Number(*n),
            Some(Token::Identifier(name)) => Expression::Identifier(name.clone()),
            _ => panic!("Invalid expression"),
        }
    }
}