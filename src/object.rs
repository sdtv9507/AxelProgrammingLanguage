use std::collections::HashMap;
use std::fmt;

use crate::parser;
#[derive(Debug, Clone)]
pub enum Objects {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Function(Function),
    BuiltIn(BuiltinFunction),
    Array(Vec<Objects>),
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
        match &self.name[..] {
            "len" => {
                if args.len() != 1 {
                    return Err("wrong number of arguments for len function");
                }
                match &args[0] {
                    Objects::String(s) => return Ok(Objects::Integer(s.len() as i32)),
                    Objects::Array(s) => return Ok(Objects::Integer(s.len() as i32)),
                    _ => return Err("unsupported argument for len"),
                }
            }
            "first" => {
                if args.len() != 1 {
                    return Err("wrong number of arguments for first function");
                }
                match &args[0] {
                    Objects::String(s) => {
                        let chr: Vec<char> = s.chars().collect();
                        return Ok(Objects::String(String::from(chr[0])));
                    }
                    Objects::Array(s) => return Ok(s[0].clone()),
                    _ => return Err("unsupported argument for first"),
                }
            }
            "last" => {
                if args.len() != 1 {
                    return Err("wrong number of arguments for last function");
                }
                match &args[0] {
                    Objects::String(s) => {
                        let chr: Vec<char> = s.chars().collect();
                        let len = chr.len() - 1;
                        return Ok(Objects::String(String::from(chr[len])));
                    }
                    Objects::Array(s) => {
                        let len = s.len() - 1;
                        return Ok(s[len].clone());
                    }
                    _ => return Err("unsupported argument for last"),
                }
            }
            "push" => {
                if args.len() != 2 {
                    return Err("wrong number of arguments for push function");
                }
                match &args[0] {
                    Objects::Array(s) => {
                        let mut f: Vec<Objects> = s.clone();
                        match &args[1] {
                            Objects::Integer(t) => f.push(Objects::Integer(*t)),
                            Objects::String(t) => f.push(Objects::String(t.clone())),
                            Objects::Boolean(t) => f.push(Objects::Boolean(t.clone())),
                            Objects::Array(t) => f.push(Objects::Array(t.clone())),
                            _ => {
                                return Err(
                                    "push function only supports objects in the second argument",
                                )
                            }
                        }
                        return Ok(Objects::Array(f));
                    }
                    _ => return Err("push function only supports arrays for the first argument"),
                }
            }
            "print" => {
                if args.len() != 1 {
                    return Err("wrong number of arguments for print function");
                }
                match &args[0] {
                    Objects::String(s) => {
                        println!("{}", s);
                        return Ok(Objects::String(String::from("")));
                    }
                    Objects::Integer(s) => {
                        println!("{}", s);
                        return Ok(Objects::String(String::from("")));
                    }
                    Objects::Float(s) => {
                        println!("{}", s);
                        return Ok(Objects::String(String::from("")));
                    }
                    Objects::Boolean(s) => {
                        println!("{}", s);
                        return Ok(Objects::String(String::from("")));
                    }
                    Objects::Array(s) => {
                        for i in s {
                            println!("{}", i);
                        }
                        return Ok(Objects::String(String::from("")));
                    }
                    _ => return Err("unsupported argument for print"),
                }
            }
            _ => return Err("builtin function not found"),
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
            Objects::Float(i) => write!(f, "Float: {}", i),
            Objects::String(s) => write!(f, "String: {}", s),
            Objects::Boolean(b) => write!(f, "Boolean: {}", b),
            Objects::Function(_s) => write!(f, "Function"),
            Objects::BuiltIn(_s) => write!(f, "Builtin Function"),
            Objects::Array(s) => {
                for i in s {
                    write!(f, "Array object {0} ", i)?;
                }
                return Ok(());
            }
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
