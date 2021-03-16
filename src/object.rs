use std::collections::HashMap;
use std::fmt;

use crate::parser;
#[derive(Debug, Clone)]
pub enum Objects {
    Integer(i32),
    Boolean(bool),
    Function(Function),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<String>,
    pub body: Box<parser::Statement>,
    pub environment: Environment,
}

impl Function {
    pub fn new(parameters: Vec<String>, body: Box<parser::Statement>) -> Self {
        let environment = Environment::new();
        Function {
            parameters,
            body,
            environment,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Environment {
    value: HashMap<String, Objects>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: String, value: Objects) {
        self.value.insert(name, value);
    }

    pub fn remove(&mut self, name: String) {
        self.value.remove_entry(&name);
    }

    pub fn update(&mut self, name: String, value: Objects) {
        self.value.entry(name).or_insert(value);
    }

    pub fn search(&mut self, name: String) -> Option<&Objects> {
        return self.value.get(&name);
    }
}

impl fmt::Display for Objects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Objects::Integer(i) => write!(f, "Integer: {}", i),
            Objects::Boolean(b) => write!(f, "Boolean: {}", b),
            Objects::Function(_s) => write!(f, "Function"),
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, j) in &self.value {
            write!(f, "{0}: {1}", i, j);
        }
        return Ok(());
    }
}
