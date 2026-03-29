#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
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
        body: Vec<Statement>,
    },
    For {
        init: Box<Statement>,
        condition: Expression,
        update: Box<Statement>,
        body: Vec<Statement>,
    },
    Assignment {
        name: String,
        value: Expression,
    },
    Break,
    Continue,
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    Return {
        value: Expression,
    },
    AssignmentIndex {
        array: String,
        index: Expression,
        value: Expression,
    },
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
    Or,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(i32),
    Identifier(String),
    String(String),
    Binary {
        left: Box<Expression>,
        op: BinaryOperation,
        right: Box<Expression>,
    },
    Unary {
        op: BinaryOperation,
        expr: Box<Expression>,
    },
    Call {
        name: String,
        args: Vec<Expression>,
    },
    ArrayLiteral(Vec<Expression>),
    Index {
        array: Box<Expression>,
        index: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Number(i32),
    String(String),
    Array(Vec<Value>),
}
