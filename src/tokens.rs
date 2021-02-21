use std::fmt;
#[derive(Debug, PartialEq, Clone)]
pub enum TokenTypes {
    Operator(char),
    Identifier(String),
    Keywords(Keywords),
    NumbersInt(i32),
    Strings(String),
    Semicolon,
    Comment,
	EndOfLine,
    Illegal,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keywords {
    Var,
    Const,
    If,
}

impl fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TokenTypes::Operator(s) => write!(f, "Operator {}", s),
            TokenTypes::Identifier(s) => write!(f, "Identifier {}", s),
            TokenTypes::Keywords(s) => write!(f, "Keyword {}", s),
            TokenTypes::NumbersInt(s) => write!(f, "Number {}", s),
            TokenTypes::Strings(s) => write!(f, "String {}", s),
            TokenTypes::Semicolon => write!(f, "Semicolon"),
            TokenTypes::Comment => write!(f, "Comment"),
            TokenTypes::EndOfLine => write!(f, "End of line"),
            TokenTypes::Illegal => write!(f, "Illegal"),
        }
    }
}

impl fmt::Display for Keywords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Keywords::Var => write!(f, "Variable declaration"),
            Keywords::Const => write!(f, "Constant declaration"),
            Keywords::If => write!(f, "If statement"),
        }
    }
}