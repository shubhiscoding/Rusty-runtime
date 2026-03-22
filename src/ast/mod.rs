#[derive(Debug)]
pub enum Statement {
    VarDecl {
        name: String,
        value: Expression,
    },
    Print {
        value: Expression,
    },
}

#[derive(Debug, Clone)]
pub enum Arithmetic {
    Addition,
    Subtraction,
    Multiplication,
    Division
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(i32),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        op: Arithmetic,
        right: Box<Expression>,
    }
}
