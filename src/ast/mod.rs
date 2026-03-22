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

#[derive(Debug)]
pub enum Expression {
    Number(i32),
    Identifier(String),
}
