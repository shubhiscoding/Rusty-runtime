use crate::ast::{BinaryOperation, Expression, Statement};


#[derive(Debug)]
pub enum Instruction {
    LoadConst(i32),
    StoreVar(String),
    LoadVar(String),
    Print,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Greater,
    Less,
    Equal,
    NotEqual,
}

fn compile_expr(instructions: &mut Vec<Instruction>, expr: &Expression) {
    match expr {
        Expression::Number(x) => instructions.push(Instruction::LoadConst(*x)),
        Expression::Identifier(val) => instructions.push(Instruction::LoadVar(val.to_string())),
        Expression::Binary { left, op, right } => {
            compile_expr(instructions, left);
            compile_expr(instructions, right);
            match op {
                BinaryOperation::Addition => instructions.push(Instruction::Add),
                BinaryOperation::Subtraction => instructions.push(Instruction::Subtract),
                BinaryOperation::Multiplication => instructions.push(Instruction::Multiply),
                BinaryOperation::Division => instructions.push(Instruction::Divide),
                BinaryOperation::Greater => instructions.push(Instruction::Greater),
                BinaryOperation::Less => instructions.push(Instruction::Less),
                BinaryOperation::Equal => instructions.push(Instruction::Equal),
                BinaryOperation::NotEqual => instructions.push(Instruction::NotEqual),
            }
        }
        Expression::Unary { op, expr } => {
            compile_expr(instructions, expr);
            match op {
                BinaryOperation::Subtraction => instructions.push(Instruction::Negate),
                _ => {}
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