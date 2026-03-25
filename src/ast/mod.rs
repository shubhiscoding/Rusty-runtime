#[derive(Debug)]
pub enum Statement {
    VarDecl {
        name: String,
        value: Expression,
    },
    Print {
        value: Expression,
    },
    If {
        condition: Expression,
        body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>
    },
    Assignment {
        name: String,
        value: Expression,
    },
    Break,
    Continue
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Greater,
    Less,
    Equal,
    NotEqual,
    And,
    Or
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(i32),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        op: BinaryOperation,
        right: Box<Expression>,
    },
    Unary {
        op: BinaryOperation,
        expr: Box<Expression>
    }
}