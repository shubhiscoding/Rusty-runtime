use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::ast::Value;

#[derive(Debug, Clone)]
pub struct Environment {
    pub variables: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(
        parent: Option<Rc<RefCell<Environment>>>
    ) -> Self {
        Environment {
            variables: HashMap::new(),
            parent,
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.variables.get(name) {
            Some(value.clone())
        } else if let Some(parent) = self.parent.as_ref() {
            parent.borrow().get(name)
        } else {
            None
        }
    }

    pub fn assign(
        &mut self,
        name: &str,
        value: Value
    ) -> bool {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            true
        } else if let Some(parent) = self.parent.as_mut() {
            parent.borrow_mut().assign(name, value)
        }else{
            false
        }
    }
}