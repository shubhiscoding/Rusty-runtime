use std::{collections::HashMap, fmt};

use crate::{ast::{BinaryOperation, Value}, compiler::Instruction};


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

pub struct Runtime {
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
}

impl Runtime {

    pub fn new() -> Self {
        Self { stack: Vec::new(), variables: HashMap::<String, Value>::new() }
    }

    fn load_variable(&mut self, var: &str) {
        if let Some(value) = self.variables.get(var) {
            self.stack.push((*value).clone());
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

    fn assign_variable(&mut self, var: String) {
        if self.variables.contains_key(&var) {
            if let Some(value) = self.stack.pop(){
                self.variables.insert(var, value);
            } else {
                panic!("No defined value to store in {}", var);
            }
        } else {
            panic!("{} is not defined", var);
        }
    }

    fn load_const(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn print(&mut self) {
        if let Some(value) = self.stack.pop(){
            println!("{}", value);
        } else {
            panic!("No defined value to print");
        }
    }
    fn pop_or_panic_stack(&mut self) -> Value {
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
                if let (Value::Number(v1), Value::Number(v2)) = (&value1, &value2) {
                    let final_value = v1 + v2;
                    self.stack.push(Value::Number(final_value));
                } else {
                    let final_value = format!("{}{}", value2, value1);
                    self.stack.push(Value::String(final_value));
                }
            },
            BinaryOperation::Subtraction => {
                if let (Value::Number(v1), Value::Number(v2)) = (value1, value2) {
                    let  final_value = v2 - v1;
                    self.stack.push(Value::Number(final_value));
                }else {
                    panic!("Subtraction is only supported for numbers");
                }
            },
            BinaryOperation::Multiplication => {
                if let (Value::Number(v1), Value::Number(v2)) = (value1, value2) {
                    let final_value = v1 * v2;
                    self.stack.push(Value::Number(final_value));
                } else {
                    panic!("Multiplication is only supported for numbers");
                }
            },
            BinaryOperation::Division => {
                if let (Value::Number(v1), Value::Number(v2)) = (value1, value2) {
                    if v1 == 0 {
                        panic!("Division by zero");
                    }
                    let final_value = v2 / v1;
                    self.stack.push(Value::Number(final_value));
                } else {
                    panic!("Division is only supported for numbers");
                }
            }
            _ => panic!("Invalid arithmetic operator")
        }
    }


    fn compare_opr(&mut self, opr_type: BinaryOperation) {
        let value1 = self.pop_or_panic_stack();
        let value2= self.pop_or_panic_stack();

        let result = match opr_type {
            BinaryOperation::Greater => {
                if matches!((&value1, &value2),
                    (Value::Number(_), Value::Number(_)) |
                    (Value::String(_), Value::String(_))
                ) {
                    value2 > value1
                } else {
                    panic!("Greater comparison is only supported between values of the same type");
                }
            },
            BinaryOperation::Less => {
                if matches!((&value1, &value2),
                    (Value::Number(_), Value::Number(_)) |
                    (Value::String(_), Value::String(_))
                ) {
                    value2 < value1
                } else {
                    panic!("Less comparison is only supported between values of the same type");
                }
            },
            BinaryOperation::Equal => value2 == value1,
            BinaryOperation::NotEqual => value2 != value1,
            _ => panic!("Invalid comparison operator")
        };
        self.stack.push(if result { Value::Number(1) } else { Value::Number(0) });
    }
}

pub fn execute(instructions: Vec<Instruction>, runtime: &mut Runtime) {
    let mut i = 0; // Initialize index to 0
    while i < instructions.len() {
        let instruction = &instructions[i];
        match instruction {
            Instruction::LoadConst(val) => {
                runtime.load_const((*val).clone());
            },
            Instruction::DeclareVar(val)  => {
                runtime.store_variable(val.to_string());
            },
            Instruction::AssignVar(val) => {
                runtime.assign_variable(val.to_string());
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
                if let Value::Number(num) = value {
                    runtime.stack.push(Value::Number(-num));
                } else {
                    panic!("Negation is only supported for numbers");
                }
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
                if matches!(condition, Value::Number(0)) ||
                   matches!(condition, Value::String(s) if s.is_empty())
                {
                    i = *idx;
                    continue;
                }
            },
            Instruction::Jump(idx) => {
                i = *idx;
                continue;
            },
            Instruction::JumpIfTrue(idx) => {
                let condition = runtime.pop_or_panic_stack();
                if matches!(condition, Value::Number(n) if n != 0) ||
                   matches!(condition, Value::String(s) if !s.is_empty())
                {
                    i = *idx;
                    continue;
                }  
            },
            Instruction::DuplicateTop => {
                if let Some(value) = runtime.stack.last() {
                    runtime.stack.push((*value).clone());
                } else {
                    panic!("No defined value to duplicate");
                }
            },
            Instruction::PopTop => {
                runtime.pop_or_panic_stack();
            }
        }
        i += 1;
    }
}