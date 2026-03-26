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
            Some(Token::Identifier(name)) => {
                if name == "print"{ 
                    self.parse_print()
                }
                else {
                    self.parse_assignment(name.clone())
                }
            }
            Some(Token::If) => self.parse_if(),
            Some(Token::While) => self.parse_while(),
            Some(Token::For) => self.parse_for(),
            Some(Token::Break) => {
                self.advance();
                if self.advance() != Some(&Token::Semicolon) {
                    panic!("Expected ';' after 'break'");
                }
                Statement::Break
            },
            Some(Token::Continue) => {
                self.advance();
                if self.advance() != Some(&Token::Semicolon) {
                    panic!("Expected ';' after 'continue'");
                }
                Statement::Continue
            },
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
    }

    fn parse_update(&mut self) -> Statement {
        // parse update
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => panic!("Expected identifier in for update"),
        };
        match self.advance() {
            Some(Token::Increment) => Statement::Assignment {
                name: name.clone(),
                value: Expression::Binary {
                    left: Box::new(Expression::Identifier(name)),
                    op: BinaryOperation::Addition,
                    right: Box::new(Expression::Number(1)),
                },
            },
            Some(Token::Decrement) => Statement::Assignment {
                name: name.clone(),
                value: Expression::Binary {
                    left: Box::new(Expression::Identifier(name)),
                    op: BinaryOperation::Subtraction,
                    right: Box::new(Expression::Number(1)),
                },
            },
            _ => panic!("Expected '++' or '--' in for update"),
        }
     }

    fn parse_assignment(&mut self, name: String) -> Statement {
        self.advance(); // consume identifier
        match self.advance() {
            Some(Token::Equals) => {}
            Some(Token::Increment) => {
                let value = Expression::Binary {
                    left: Box::new(Expression::Identifier(name.clone())),
                    op: BinaryOperation::Addition,
                    right: Box::new(Expression::Number(1)),
                };
                match self.advance() {
                    Some(Token::Semicolon) => {}
                    _ => panic!("Expected ';'"),
                }
                return Statement::Assignment { name, value }
            },
            Some(Token::Decrement) => {
                let value = Expression::Binary {
                    left: Box::new(Expression::Identifier(name.clone())),
                    op: BinaryOperation::Subtraction,
                    right: Box::new(Expression::Number(1)),
                };
                match self.advance() {
                    Some(Token::Semicolon) => {}
                    _ => panic!("Expected ';'"),
                }
                return Statement::Assignment { name, value }
            },
            _ => panic!("Expected '='"),
        }

        let value = self.parse_first();

        match self.advance() {
            Some(Token::Semicolon) => {}
            _ => panic!("Expected ';'"),
        }

        Statement::Assignment { name, value }
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

        let value = self.parse_first();

        match self.advance() {
            Some(Token::Semicolon) => {}
            _ => panic!("Expected ';'"),
        }

        Statement::VarDecl { name, value }
    }

    fn parse_print(&mut self) -> Statement {
        self.advance(); // consume 'print'

        let value = self.parse_first();

        match self.advance() {
            Some(Token::Semicolon) => {}
            _ => panic!("Expected ';'"),
        }

        Statement::Print { value }
    }

    fn parse_and(&mut self) -> Expression {
        let mut left = self.parse_comparison();

        while let Some(Token::And) = self.peek() {
            self.advance();
            let right = self.parse_comparison();
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOperation::And,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_or(&mut self) -> Expression {
        let mut left = self.parse_and();

        while let Some(Token::Or) = self.peek() {
            self.advance();
            let right = self.parse_and();
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOperation::Or,
                right: Box::new(right),
            };
        }
        left
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
                let expr = self.parse_first();
                match self.advance() {
                    Some(Token::RightParentheses) => {}
                    _ => panic!("Expected ')'"),
                }
                expr
            }
            Some(Token::Number(n)) => Expression::Number(*n),
            Some(Token::Identifier(name)) => Expression::Identifier(name.clone()),
            Some(Token::String(s)) => Expression::String(s.clone()),
            _ => panic!("Invalid expression"),
        }
    }

    fn parse_if(&mut self) -> Statement {
        self.advance(); // consume 'if'

        match self.advance() {
            Some(Token::LeftParentheses) => {}
            _ => panic!("Expected '(' after 'if'"),
        }

        let condition = self.parse_first();

        match self.advance() {
            Some(Token::RightParentheses) => {}
            _ => panic!("Expected ')' after if condition"),
        }

        match self.advance() {
            Some(Token::LeftBrace) => {}
            _ => panic!("Expected '{{'"),
        }

        let mut body = Vec::new();
        while let Some(token) = self.peek() {
            if *token == Token::RightBrace {
                break;
            }
            body.push(self.parse_statement());
        }

        match self.advance() {
            Some(Token::RightBrace) => {}
            _ => panic!("Expected '}}'"),
        }

        let mut else_body = Vec::new();
        if let Some(Token::Else) = self.peek() {
            self.advance();
            match self.advance() {
                Some(Token::LeftBrace) => {}_ => panic!("Expected '{{' after 'else'"),
            }

            while let Some(token) = self.peek() {
                if *token == Token::RightBrace {
                    break;
                }
                else_body.push(self.parse_statement());
            }

            match self.advance() {
                Some(Token::RightBrace) => {}
                _ => panic!("Expected '}}' after else block"),
            }
            return Statement::If { condition, body, else_body: Some(else_body)};
        }

        Statement::If { condition, body, else_body: None }
    }

    fn parse_while(&mut self) -> Statement {
        self.advance(); // consume 'while'

        match self.advance() {
            Some(Token::LeftParentheses) => {}
            _ => panic!("Expected '(' after 'while'"),
        }

        let condition = self.parse_first();

        match self.advance() {
            Some(Token::RightParentheses) => {}
            _ => panic!("Expected ')' after while condition"),
        }

        match self.advance() {
            Some(Token::LeftBrace) => {}
            _ => panic!("Expected '{{'"),
        }

        let mut body = Vec::new();
        while let Some(token) = self.peek() {
            if *token == Token::RightBrace {
                break;
            }
            body.push(self.parse_statement());
        }

        match self.advance() {
            Some(Token::RightBrace) => {}
            _ => panic!("Expected '}}' after while block"),
        }

        Statement::While { condition, body }
     }

    fn parse_for(&mut self) -> Statement {
        self.advance(); // consume 'for'

        match self.advance() {
            Some(Token::LeftParentheses) => {}
            _ => panic!("Expected '(' after 'for'"),
        }

        let init = self.parse_statement();

        let condition = self.parse_first();

        match self.advance() {
            Some(Token::Semicolon) => {}
            _ => panic!("Expected ';' after for condition"),
        }

        let update = self.parse_update();

        match self.advance() {
            Some(Token::RightParentheses) => {}
            _ => panic!("Expected ')' after for condition"),
        }

        match self.advance() {
            Some(Token::LeftBrace) => {}
            _ => panic!("Expected '{{'"),
        }

        let mut body = Vec::new();
        while let Some(token) = self.peek() {
            if *token == Token::RightBrace {
                break;
            }
            body.push(self.parse_statement());
        }

        match self.advance() {
            Some(Token::RightBrace) => {}
            _ => panic!("Expected '}}' after for block"),
        }

        Statement::For { init: Box::new(init), condition, update: Box::new(update), body }
    }

    fn parse_first(&mut self) -> Expression {
        self.parse_or()
    }
}
