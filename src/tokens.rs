pub enum TokenTypes {
    Operator(char),
    Identifier(String),
    Keywords(Keywords),
    Numbers(i32),
    Comment,
    Break,
    EndOfLine,
    Unknown,
    Illegal,
}

pub enum Keywords {
    Let,
    Const,
    If,
}
