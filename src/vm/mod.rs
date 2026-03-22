use std::collections::{HashMap};

use crate::compiler::Instruction;

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
            }
        }
    }
}