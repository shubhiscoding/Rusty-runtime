use crate::ast::{BinaryOperation, Expression, Statement};

pub struct LoopContext {
    start: usize,
    break_placeholders: Vec<usize>,
    continue_placeholders: Vec<usize>,
}


#[derive(Debug)]
pub enum Instruction {
    LoadConst(i32),
    DeclareVar(String),
    AssignVar(String),
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
    JumpIfFalse(usize),
    Jump(usize),
    JumpIfTrue(usize),
    DuplicateTop,
    PopTop
}

fn compile_expr(instructions: &mut Vec<Instruction>, expr: &Expression) {
    match expr {
        Expression::Number(x) => instructions.push(Instruction::LoadConst(*x)),
        Expression::Identifier(val) => instructions.push(Instruction::LoadVar(val.to_string())),
        Expression::Binary { left, op, right } => {
            if *op != BinaryOperation::And && *op != BinaryOperation::Or {
                compile_expr(instructions, left);
                compile_expr(instructions, right);
            } else {
                compile_expr(instructions, left);
            }
            match op {
                BinaryOperation::Addition => instructions.push(Instruction::Add),
                BinaryOperation::Subtraction => instructions.push(Instruction::Subtract),
                BinaryOperation::Multiplication => instructions.push(Instruction::Multiply),
                BinaryOperation::Division => instructions.push(Instruction::Divide),
                BinaryOperation::Greater => instructions.push(Instruction::Greater),
                BinaryOperation::Less => instructions.push(Instruction::Less),
                BinaryOperation::Equal => instructions.push(Instruction::Equal),
                BinaryOperation::NotEqual => instructions.push(Instruction::NotEqual),
                BinaryOperation::And => {
                    instructions.push(Instruction::DuplicateTop);
                    instructions.push(Instruction::JumpIfFalse(0));
                    let jump_instructions_index = instructions.len() - 1;
                    instructions.push(Instruction::PopTop);
                    compile_expr(instructions, right);
                    instructions[jump_instructions_index] = Instruction::JumpIfFalse(instructions.len());
                }
                BinaryOperation::Or => {
                    instructions.push(Instruction::DuplicateTop);
                    instructions.push(Instruction::JumpIfTrue(0));
                    let jump_instructions_index = instructions.len() - 1;
                    instructions.push(Instruction::PopTop);
                    compile_expr(instructions, right);
                    instructions[jump_instructions_index] = Instruction::JumpIfTrue(instructions.len());
                }
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

pub fn compile_statements(
    statements: Vec<Statement>,
    instructions: &mut Vec<Instruction>,
    mut loop_ctx: Option<&mut LoopContext>,
) {
    for statement in statements {
        match statement {
            Statement::VarDecl { name, value } => {
                compile_expr(instructions, &value);
                instructions.push(Instruction::DeclareVar(name));
            },
            Statement::Print { value } => {
                compile_expr(instructions, &value);
                instructions.push(Instruction::Print);
            }
            Statement::If { condition, body, else_body } => {
                compile_expr(instructions, &condition);

                let jump_if_false_index = instructions.len();
                instructions.push(Instruction::JumpIfFalse(0));

                compile_statements(body, instructions, loop_ctx.as_deref_mut());

                if let Some(else_body) = else_body {
                    let jump_to_end_index = instructions.len();
                    instructions.push(Instruction::Jump(0));
                    instructions[jump_if_false_index] = Instruction::JumpIfFalse(instructions.len());
                    compile_statements(else_body, instructions, loop_ctx.as_deref_mut());
                    instructions[jump_to_end_index] = Instruction::Jump(instructions.len());
                } else {
                    instructions[jump_if_false_index] = Instruction::JumpIfFalse(instructions.len());
                }
            }
            Statement::While { condition, body } => {
                let loop_start_index = instructions.len();
                compile_expr(instructions, &condition);

                let jump_if_false_index = instructions.len();
                instructions.push(Instruction::JumpIfFalse(0));

                let mut ctx = LoopContext {
                    start: loop_start_index,
                    break_placeholders: vec![],
                    continue_placeholders: vec![],
                };
                compile_statements(body, instructions, Some(&mut ctx));

                for idx in ctx.continue_placeholders {
                    instructions[idx] = Instruction::Jump(ctx.start);
                }

                instructions.push(Instruction::Jump(loop_start_index));
                instructions[jump_if_false_index] = Instruction::JumpIfFalse(instructions.len());

                for idx in ctx.break_placeholders {
                    instructions[idx] = Instruction::Jump(instructions.len());
                }
            },
            Statement::For { init, condition, update, body } => {
                compile_statements(vec![*init], instructions, loop_ctx.as_deref_mut());

                let loop_start_index = instructions.len();
                compile_expr(instructions, &condition);

                let jump_if_false_index = instructions.len();
                instructions.push(Instruction::JumpIfFalse(0));

                let mut ctx = LoopContext {
                    start: 0,
                    break_placeholders: vec![],
                    continue_placeholders: vec![],
                };
                compile_statements(body, instructions, Some(&mut ctx));

                let update_start = instructions.len();
                for idx in ctx.continue_placeholders {
                    instructions[idx] = Instruction::Jump(update_start);
                }

                compile_statements(vec![*update], instructions, None);

                instructions.push(Instruction::Jump(loop_start_index));
                instructions[jump_if_false_index] = Instruction::JumpIfFalse(instructions.len());

                for idx in ctx.break_placeholders {
                    instructions[idx] = Instruction::Jump(instructions.len());
                }
            },
            Statement::Assignment { name, value } => {
                compile_expr(instructions, &value);
                instructions.push(Instruction::AssignVar(name));
            },
            Statement::Break => {
                if let Some(ctx) = loop_ctx.as_deref_mut() {
                    instructions.push(Instruction::Jump(0));
                    ctx.break_placeholders.push(instructions.len() - 1);
                } else {
                    panic!("'break' used outside of a loop");
                }
            },
            Statement::Continue => {
                if let Some(ctx) = loop_ctx.as_deref_mut() {
                    instructions.push(Instruction::Jump(0));
                    ctx.continue_placeholders.push(instructions.len() - 1);
                } else {
                    panic!("'continue' used outside of a loop");
                }
            }
        }
    }
}
