use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::{
    ast::{BinaryOperation, Value},
    compiler::{FunctionBytecode, Instruction, Program},
};

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                let vec = arr.borrow();
                let elements = vec
                    .iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{}]", elements)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    variables: HashMap<String, Value>,
    instructions: Vec<Instruction>,
    ip: usize,
}

pub struct Runtime {
    stack: Vec<Value>,
    frames: Vec<Frame>,
    functions: HashMap<String, FunctionBytecode>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            frames: Vec::new(),
            functions: HashMap::new(),
        }
    }

    fn load_variable(&mut self, var: &str) {
        if let Some(value) = self
            .frames
            .iter()
            .rev()
            .find_map(|frame| frame.variables.get(var))
        {
            self.stack.push((*value).clone());
        } else {
            panic!("{} is not defined", var);
        }
    }

    fn store_variable(&mut self, var: String) {
        if let Some(value) = self.stack.pop() {
            self.frames.last_mut().unwrap().variables.insert(var, value);
        } else {
            panic!("No defined value to store in {}", var);
        }
    }

    fn assign_variable(&mut self, var: String) {
        if let Some(value) = self
            .frames
            .iter_mut()
            .rev()
            .find_map(|frame| frame.variables.get_mut(&var))
        {
            if let Some(val) = self.stack.pop() {
                *value = val;
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
        if let Some(value) = self.stack.pop() {
            println!("{}", value);
        } else {
            panic!("No defined value to print");
        }
    }
    fn pop_or_panic_stack(&mut self) -> Value {
        if let Some(value) = self.stack.pop() {
            value
        } else {
            panic!("Invalid expression");
        }
    }
    fn arithmetic_opr(&mut self, opr_type: BinaryOperation) {
        let value1 = self.pop_or_panic_stack();
        let value2 = self.pop_or_panic_stack();

        match opr_type {
            BinaryOperation::Addition => {
                if let (Value::Number(v1), Value::Number(v2)) = (&value1, &value2) {
                    let final_value = v1 + v2;
                    self.stack.push(Value::Number(final_value));
                } else {
                    let final_value = format!("{}{}", value2, value1);
                    self.stack.push(Value::String(final_value));
                }
            }
            BinaryOperation::Subtraction => {
                if let (Value::Number(v1), Value::Number(v2)) = (value1, value2) {
                    let final_value = v2 - v1;
                    self.stack.push(Value::Number(final_value));
                } else {
                    panic!("Subtraction is only supported for numbers");
                }
            }
            BinaryOperation::Multiplication => {
                if let (Value::Number(v1), Value::Number(v2)) = (value1, value2) {
                    let final_value = v1 * v2;
                    self.stack.push(Value::Number(final_value));
                } else {
                    panic!("Multiplication is only supported for numbers");
                }
            }
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
            _ => panic!("Invalid arithmetic operator"),
        }
    }

    fn compare_opr(&mut self, opr_type: BinaryOperation) {
        let value1 = self.pop_or_panic_stack();
        let value2 = self.pop_or_panic_stack();

        let result = match opr_type {
            BinaryOperation::Greater => {
                if matches!(
                    (&value1, &value2),
                    (Value::Number(_), Value::Number(_)) | (Value::String(_), Value::String(_))
                ) {
                    value2 > value1
                } else {
                    panic!("Greater comparison is only supported between values of the same type");
                }
            }
            BinaryOperation::Less => {
                if matches!(
                    (&value1, &value2),
                    (Value::Number(_), Value::Number(_)) | (Value::String(_), Value::String(_))
                ) {
                    value2 < value1
                } else {
                    panic!("Less comparison is only supported between values of the same type");
                }
            }
            BinaryOperation::Equal => value2 == value1,
            BinaryOperation::NotEqual => value2 != value1,
            _ => panic!("Invalid comparison operator"),
        };
        self.stack.push(if result {
            Value::Number(1)
        } else {
            Value::Number(0)
        });
    }
}

pub fn execute(program: Program, runtime: &mut Runtime) {
    runtime.functions = program.functions;
    runtime.frames.push(Frame {
        variables: HashMap::new(),
        instructions: program.main,
        ip: 0,
    });

    while let Some(frame) = runtime.frames.last() {
        if frame.ip >= frame.instructions.len() {
            runtime.frames.pop(); // function ended without explicit return
            continue;
        }

        let instruction = frame.instructions[frame.ip].clone();
        match instruction {
            Instruction::LoadConst(val) => {
                runtime.load_const((val).clone());
            }
            Instruction::DeclareVar(val) => {
                runtime.store_variable(val.to_string());
            }
            Instruction::AssignVar(val) => {
                runtime.assign_variable(val.to_string());
            }
            Instruction::LoadVar(val) => {
                runtime.load_variable(&val);
            }
            Instruction::Print => {
                runtime.print();
            }
            Instruction::Add => {
                runtime.arithmetic_opr(BinaryOperation::Addition);
            }
            Instruction::Subtract => {
                runtime.arithmetic_opr(BinaryOperation::Subtraction);
            }
            Instruction::Multiply => {
                runtime.arithmetic_opr(BinaryOperation::Multiplication);
            }
            Instruction::Divide => {
                runtime.arithmetic_opr(BinaryOperation::Division);
            }
            Instruction::Negate => {
                let value = runtime.pop_or_panic_stack();
                if let Value::Number(num) = value {
                    runtime.stack.push(Value::Number(-num));
                } else {
                    panic!("Negation is only supported for numbers");
                }
            }
            Instruction::Greater => {
                runtime.compare_opr(BinaryOperation::Greater);
            }
            Instruction::Less => {
                runtime.compare_opr(BinaryOperation::Less);
            }
            Instruction::Equal => {
                runtime.compare_opr(BinaryOperation::Equal);
            }
            Instruction::NotEqual => {
                runtime.compare_opr(BinaryOperation::NotEqual);
            }
            Instruction::JumpIfFalse(idx) => {
                let condition = runtime.pop_or_panic_stack();
                if matches!(condition, Value::Number(0))
                    || matches!(condition, Value::String(s) if s.is_empty())
                {
                    runtime.frames.last_mut().unwrap().ip = idx;
                    continue;
                }
            }
            Instruction::Jump(idx) => {
                runtime.frames.last_mut().unwrap().ip = idx;
                continue;
            }
            Instruction::JumpIfTrue(idx) => {
                let condition = runtime.pop_or_panic_stack();
                if matches!(condition, Value::Number(n) if n != 0)
                    || matches!(condition, Value::String(s) if !s.is_empty())
                {
                    runtime.frames.last_mut().unwrap().ip = idx;
                    continue;
                }
            }
            Instruction::DuplicateTop => {
                if let Some(value) = runtime.stack.last() {
                    runtime.stack.push((*value).clone());
                } else {
                    panic!("No defined value to duplicate");
                }
            }
            Instruction::PopTop => {
                runtime.pop_or_panic_stack();
            }
            Instruction::CallFunction(name, arg_count) => {
                let func = runtime.functions.get(&name).expect("undefined function");
                let mut locals = HashMap::new();
                // Pop args in reverse (last arg was pushed last)
                if arg_count != func.params.len() {
                    panic!(
                        "Expected {} arguments but got {}",
                        func.params.len(),
                        arg_count
                    );
                }
                for param in func.params.iter().rev() {
                    locals.insert(param.clone(), runtime.stack.pop().unwrap());
                }
                // Save caller's position (move past the CallFunction instruction)
                runtime.frames.last_mut().unwrap().ip += 1;
                // Push new frame
                runtime.frames.push(Frame {
                    instructions: func.instructions.clone(),
                    ip: 0,
                    variables: locals,
                });
                continue; // don't increment ip again
            }
            Instruction::Return => {
                let return_value = runtime.stack.pop().unwrap();
                runtime.frames.pop();
                runtime.stack.push(return_value);
                continue;
            }
            Instruction::CreateArray(len) => {
                let elements = Rc::new(RefCell::new(Vec::new()));
                for _ in 0..len {
                    if let Some(value) = runtime.stack.pop() {
                        elements.borrow_mut().push(value);
                    } else {
                        panic!("Not enough values on stack to create array");
                    }
                }
                elements.borrow_mut().reverse();
                runtime.stack.push(Value::Array(elements));
            }
            Instruction::LoadIndex => {
                let index = runtime.pop_or_panic_stack();
                let array = runtime.pop_or_panic_stack();
                if let (Value::Array(arr), Value::Number(idx)) = (array, index) {
                    let vec = arr.borrow();
                    if idx < 0 || (idx as usize) >= vec.len() {
                        panic!("Array index out of bounds");
                    } // immutable borrow
                    runtime.stack.push(vec[idx as usize].clone());
                } else {
                    panic!("LoadIndex requires an array and a number index");
                }
            }
            Instruction::StoreIndex => {
                let value = runtime.pop_or_panic_stack();
                let index = runtime.pop_or_panic_stack();
                let array = runtime.pop_or_panic_stack();

                if let (Value::Array(arr), Value::Number(idx)) = (&array, index) {
                    let mut vec = arr.borrow_mut();
                    if idx < 0 || (idx as usize) >= vec.len() {
                        panic!("Array index out of bounds");
                    }
                    vec[idx as usize] = value;
                } else {
                    panic!("StoreIndex requires an array and a number index");
                }
            }
        }
        runtime.frames.last_mut().unwrap().ip += 1; // Move to next instruction
    }
}
