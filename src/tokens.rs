pub enum TokenTypes {
    Operator(String),
    Keywords(Keywords),
    Strings(String),
    Numbers(i32),
    Unknown,
    Illegal,
}

pub enum Keywords {
    Let(String),
    If(String),
}
