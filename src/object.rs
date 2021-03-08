use std::fmt;
#[derive(Debug, Clone)]
pub enum Objects {
    Integer(i32),
    Boolean(bool),
    Null,
}

impl fmt::Display for Objects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Objects::Integer(i) => write!(f, "Integer: {}", i),
            Objects::Boolean(b) => write!(f, "Boolean: {}", b),
            Objects::Null => write!(f, "Null"),
        }
    }
}