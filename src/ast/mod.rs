use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

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
        index: Vec<Expression>,
        value: Expression,
    },
    AssignmentProperty {
        object: String,
        property: Vec<String>,
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
    ObjectLiteral(Vec<(String, Expression)>),
    PropertyAccess {
        object: Box<Expression>,
        property: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i32),
    String(String),
    Array(Rc<RefCell<Vec<Value>>>),
    Object(Rc<RefCell<HashMap<String, Value>>>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}
