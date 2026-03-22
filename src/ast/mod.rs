#[derive(Debug)]
pub enum Statement {
    VarDecl {
        name: String,
        value: Value,
    },
    Print {
        value: Value,
    },
}

#[derive(Debug)]
pub enum Arithmetic {
    Addition
}

#[derive(Debug)]
pub enum Expression {
    Number(i32),
    Identifier(String)
}

#[derive(Debug)]
pub struct  BinaryExpression {
    pub value1: Expression,
    pub identifier: Arithmetic,
    pub value2: Expression
}

#[derive(Debug)]
pub enum Value {
    Expression(Expression),         // Can print an Expression
    BinaryExpression(BinaryExpression), // Can also print a BinaryExpression
}