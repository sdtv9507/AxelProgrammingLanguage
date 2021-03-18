use std::collections::HashMap;
use std::fmt;

use crate::parser;
#[derive(Debug, Clone)]
pub enum Objects {
    Integer(i32),
    String(String),
    Boolean(bool),
    Function(Function),
    BuiltIn(BuiltinFunction),
}

#[derive(Debug, Clone)]
pub struct BuiltinFunction {
    pub name: String,
}

impl BuiltinFunction {
    pub fn new(name: String) -> Self {
        BuiltinFunction { name }
    }
    pub fn call<'a>(&mut self, args: Vec<Objects>) -> Result<Objects, &'a str> {
        match &self.name {
            _len => {
                if args.len() != 1 {
                    return Err("wrong number of arguments for len function");
                }
                match &args[0] {
                    Objects::String(s) => return Ok(Objects::Integer(s.len() as i32)),
                    _ => return Err("unsupported argument for len"),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<String>,
    pub body: Box<Vec<parser::Statement>>,
    pub environment: Environment,
}

impl Function {
    pub fn new(parameters: Vec<String>, body: Box<Vec<parser::Statement>>) -> Self {
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

    pub fn search(&mut self, name: String) -> Option<&Objects> {
        return self.value.get(&name);
    }
}

impl fmt::Display for Objects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Objects::Integer(i) => write!(f, "Integer: {}", i),
            Objects::String(s) => write!(f, "String: {}", s),
            Objects::Boolean(b) => write!(f, "Boolean: {}", b),
            Objects::Function(_s) => write!(f, "Function"),
            Objects::BuiltIn(_s) => write!(f, "Builtin Function"),
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, j) in &self.value {
            write!(f, "{0}: {1}", i, j)?;
        }
        return Ok(());
    }
}
