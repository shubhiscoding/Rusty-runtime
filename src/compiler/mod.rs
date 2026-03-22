use crate::ast::{Arithmetic, Expression, Statement};


#[derive(Debug)]
pub enum Instruction {
    LoadConst(i32),
    StoreVar(String),
    LoadVar(String),
    Print,
    Add
}

fn compile_expr(instructions: &mut Vec<Instruction>, expr: &Expression) {
    match expr {
        Expression::Number(x) => instructions.push(Instruction::LoadConst(*x)),
        Expression::Identifier(val) => instructions.push(Instruction::LoadVar(val.to_string())),
        Expression::Binary { left, op, right } => {
            compile_expr(instructions, left);
            compile_expr(instructions, right);
            match op {
                Arithmetic::Addition => instructions.push(Instruction::Add),
            }
        }
    }
}

pub fn compile_statements(statements: Vec<Statement>) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();
    for statement in statements {
        match statement {
            Statement::VarDecl { name, value } => {
                compile_expr(&mut instructions, &value);
                instructions.push(Instruction::StoreVar(name));
            },
            Statement::Print { value } => {
                compile_expr(&mut instructions, &value);
                instructions.push(Instruction::Print);
            }
        }
    }
    instructions
}