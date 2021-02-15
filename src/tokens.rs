pub enum TokenTypes {
    Operator(char),
    Identifier(String),
    Keywords(Keywords),
    NumbersInt(i32),
    Comment,
    Illegal,
}

pub enum Keywords {
    Var,
    Const,
    If,
}
