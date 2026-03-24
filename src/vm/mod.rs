use std::{collections::HashMap, i32};

use crate::{ast::BinaryOperation, compiler::Instruction};

pub struct Runtime {
    stack: Vec<i32>,
    variables: HashMap<String, i32>,
}

impl Runtime {

    pub fn new() -> Self {
        Self { stack: Vec::new(), variables: HashMap::<String, i32>::new() }
    }

    fn load_variable(&mut self, var: &str) {
        if let Some(value) = self.variables.get(var) {
            self.stack.push(*value);
        } else {
            panic!("{} is not defined", var);
        }
    }

    fn store_variable(&mut self, var: String) {
        if let Some(value) = self.stack.pop(){
            self.variables.insert(var, value);
        } else {
            panic!("No defined value to store in {}", var);
        }
    }

    fn load_const(&mut self, value: i32) {
        self.stack.push(value);
    }

    fn print(&mut self) {
        if let Some(value) = self.stack.pop(){
            println!("{}", value);
        } else {
            panic!("No defined value to print");
        }
    }
    fn pop_or_panic_stack(&mut self) -> i32 {
        if let Some(value) = self.stack.pop(){
            return value;
        } else {
            panic!("Invalid expression");
        }
    }
    fn arithmetic_opr(&mut self, opr_type: BinaryOperation) {
        let value1 = self.pop_or_panic_stack();
        let value2= self.pop_or_panic_stack();

        match opr_type {
            BinaryOperation::Addition => {
                let  final_value = value1 + value2;
                self.stack.push(final_value);
            },
            BinaryOperation::Subtraction => {
                let  final_value = value2 - value1;
                self.stack.push(final_value);
            },
            BinaryOperation::Multiplication => {
                let  final_value = value1 * value2;
                self.stack.push(final_value);
            },
            BinaryOperation::Division => {
                if value1 == 0 {
                    panic!("Division by zero");
                }
                let final_value = value2 / value1;
                self.stack.push(final_value);
            }
            _ => panic!("Invalid arithmetic operator")
        }
    }


    fn compare_opr(&mut self, opr_type: BinaryOperation) {
        let value1 = self.pop_or_panic_stack();
        let value2= self.pop_or_panic_stack();

        let result = match opr_type {
            BinaryOperation::Greater => value2 > value1,
            BinaryOperation::Less => value2 < value1,
            BinaryOperation::Equal => value2 == value1,
            BinaryOperation::NotEqual => value2 != value1,
            _ => panic!("Invalid comparison operator")
        };
        self.stack.push(if result { 1 } else { 0 });
    }
}

pub fn execute(instructions: Vec<Instruction>, runtime: &mut Runtime) {
    let mut i = 0; // Initialize index to 0
    while i < instructions.len() {
        let instruction = &instructions[i];
        match instruction {
            Instruction::LoadConst(val) => {
                runtime.load_const(*val);
            },
            Instruction::StoreVar(val)  => {
                runtime.store_variable(val.to_string());
            },
            Instruction::LoadVar(val) => {
                runtime.load_variable(val);
            },
            Instruction::Print => {
                runtime.print();
            },
            Instruction::Add => {
                runtime.arithmetic_opr(BinaryOperation::Addition);
            },
            Instruction::Subtract => {
                runtime.arithmetic_opr(BinaryOperation::Subtraction);
            },
            Instruction::Multiply => {
                runtime.arithmetic_opr(BinaryOperation::Multiplication);
            },
            Instruction::Divide => {
                runtime.arithmetic_opr(BinaryOperation::Division);
            },
            Instruction::Negate => {
                let value = runtime.pop_or_panic_stack();
                runtime.stack.push(-value);
            },
            Instruction::Greater => {
                runtime.compare_opr(BinaryOperation::Greater);
            },
            Instruction::Less => {
                runtime.compare_opr(BinaryOperation::Less);
            },
            Instruction::Equal => {
                runtime.compare_opr(BinaryOperation::Equal);
            },
            Instruction::NotEqual => {
                runtime.compare_opr(BinaryOperation::NotEqual);
            },
            Instruction::JumpIfFalse(idx) => {
                let condition = runtime.pop_or_panic_stack();
                if condition == 0 {
                    i = *idx;
                    continue;
                }
            }
        }
        i += 1;
    }
}