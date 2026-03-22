use crate::ast::{Arithmetic, Expression, Statement, Value};


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
    }
}

fn compile_val(instructions: &mut Vec<Instruction>, value: &Value){
    match value {
        Value::Expression(expression) => compile_expr(instructions, expression),
        
        Value::BinaryExpression(expression) => {
            let identifier = &expression.identifier;
            let value1 = &expression.value1;
            let value2 = &expression.value2;
            
            compile_expr(instructions, value1);
            compile_expr(instructions, value2);

            match identifier {
                Arithmetic::Addition => {
                    instructions.push(Instruction::Add);
                }
            }
        }
    }
}

pub fn compile_statements(statements: Vec<Statement>) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();
    for statement in statements {
        match statement {
            Statement::VarDecl { name, value } => {
                compile_val(&mut instructions, &value);
                instructions.push(Instruction::StoreVar(name));
            },
            Statement::Print { value } => {
                compile_val(&mut instructions, &value);
                instructions.push(Instruction::Print);
            }
        }
    }
    instructions
}