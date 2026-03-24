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
        body: Vec<Statement>
    }
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
