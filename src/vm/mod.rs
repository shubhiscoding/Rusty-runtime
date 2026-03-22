use std::{collections::HashMap, i32};

use crate::{ast::Arithmetic, compiler::Instruction};

struct Runtime {
    stack: Vec<i32>,
    variables: HashMap<String, i32>,
}

impl Runtime {
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
    fn arithmetic_opr(&mut self, opr_type: Arithmetic) {
        let value1 = self.pop_or_panic_stack();
        let value2= self.pop_or_panic_stack();

        match opr_type {
            Arithmetic::Addition => {
                let  final_value = value1 + value2;
                self.stack.push(final_value);
            }
        }
    }
}

pub fn execute(instructions: Vec<Instruction>)  {
    let mut runtime = Runtime{stack: Vec::new(), variables: HashMap::<String, i32>::new()};
    for instruction in instructions {
        match instruction {
            Instruction::LoadConst(val) => {
                runtime.load_const(val);
            },
            Instruction::StoreVar(val)  => {
                runtime.store_variable(val);
            },
            Instruction::LoadVar(val) => {
                runtime.load_variable(&val);
            },
            Instruction::Print => {
                runtime.print();
            },
            Instruction::Add => {
                runtime.arithmetic_opr(Arithmetic::Addition);
            }
        }
    }
}