use crate::lexer::Token;
use crate::ast::{BinaryOperation, Expression, Statement};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

fn is_arithmetic_operator(tkn: &Token) -> Option<BinaryOperation> {
    match tkn {
        Token::Addition => Some(BinaryOperation::Addition),
        Token::Subtraction => Some(BinaryOperation::Subtraction),
        Token::Multiplication => Some(BinaryOperation::Multiplication),
        Token::Division => Some(BinaryOperation::Division),
        _ => None
    }
}

fn is_compare_operator(tkn: &Token) -> Option<BinaryOperation> {
    match tkn {
        Token::Greater => Some(BinaryOperation::Greater),
        Token::Less => Some(BinaryOperation::Less),
        Token::EqualEqual => Some(BinaryOperation::Equal),
        Token::NotEqual => Some(BinaryOperation::NotEqual),
        _ => None
    }
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

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while self.pos < self.tokens.len() {
            statements.push(self.parse_statement());
        }

        statements
    }

    fn parse_statement(&mut self) -> Statement {
        match self.peek() {
            Some(Token::Let) => self.parse_var_decl(),
            Some(Token::Identifier(name)) if name == "print" => self.parse_print(),
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
    }

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

        let value = self.parse_comparison();

        match self.advance() {
            Some(Token::Semicolon) => {}
            _ => panic!("Expected ';'"),
        }

        Statement::VarDecl { name, value }
    }

    fn parse_print(&mut self) -> Statement {
        self.advance(); // consume 'print'

        let value = self.parse_comparison();

        match self.advance() {
            Some(Token::Semicolon) => {}
            _ => panic!("Expected ';'"),
        }

        Statement::Print { value }
    }

    fn parse_comparison(&mut self) -> Expression {
        let mut left = self.parse_expression();

        while let Some(token) = self.peek() {
            if let Some(opr) = is_compare_operator(token){
                self.advance();
                let right = self.parse_expression();
                left = Expression::Binary {
                    left: Box::new(left),
                    op: opr,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        left
    }

    fn parse_expression(&mut self) -> Expression {
        let mut left = self.parse_term();

        while let Some(token) = self.peek() {
            if let Some(opr) = is_arithmetic_operator(token){
                if opr != BinaryOperation::Addition && opr != BinaryOperation::Subtraction {
                    break;
                }
                self.advance();
                let right = self.parse_term();
                left = Expression::Binary {
                    left: Box::new(left),
                    op: opr,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        left
    }

    fn parse_term(&mut self) -> Expression {
        let mut left = self.parse_unary();

        while let Some(token) = self.peek() {
            if let Some(opr) = is_arithmetic_operator(token){
                if opr != BinaryOperation::Multiplication && opr != BinaryOperation::Division {
                    break;
                }
                self.advance();
                let right = self.parse_primary();
                left = Expression::Binary {
                    left: Box::new(left),
                    op: opr,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        left
    }

    fn parse_unary(&mut self) -> Expression {
        if let Some(token) = self.peek() {
            if let Some(opr) = is_arithmetic_operator(token){
                if opr == BinaryOperation::Subtraction {
                    self.advance();
                    let expr = self.parse_unary();
                    return Expression::Unary {
                        op: opr,
                        expr: Box::new(expr)
                    }
                }
            }
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expression {
        match self.advance() {
            Some(Token::LeftParentheses) => {
                let expr = self.parse_comparison();
                match self.advance() {
                    Some(Token::RightParentheses) => {}
                    _ => panic!("Expected ')'"),
                }
                expr
            }
            Some(Token::Number(n)) => Expression::Number(*n),
            Some(Token::Identifier(name)) => Expression::Identifier(name.clone()),
            _ => panic!("Invalid expression"),
        }
    }
}
