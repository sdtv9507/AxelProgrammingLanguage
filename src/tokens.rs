use std::fmt;
#[derive(Debug, PartialEq, Clone)]
pub enum TokenTypes {
    Operator(char),
    Identifier(String),
    Keywords(Keywords),
    NumbersInt(i32),
    Strings(String),
    Delim(char),
    Compare(Comparison),
    Bang,
    Comma,
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
    Else,
    Return,
    Function,
    True,
    False,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Comparison {
    Less,
    Greater,
    LessE,
    GreaterE,
    Equal,
    NotEqual,
}

impl fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TokenTypes::Operator(s) => write!(f, "Operator {}", s),
            TokenTypes::Identifier(s) => write!(f, "Identifier {}", s),
            TokenTypes::Keywords(s) => write!(f, "Keyword {}", s),
            TokenTypes::NumbersInt(s) => write!(f, "Number {}", s),
            TokenTypes::Strings(s) => write!(f, "String {}", s),
            TokenTypes::Delim(s) => write!(f, "Delim {}", s),
            TokenTypes::Compare(s) => write!(f, "Compare {}", s),
            TokenTypes::Bang => write!(f, "Bang"),
            TokenTypes::Comma => write!(f, "Comma"),
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
            Keywords::Else => write!(f, "Else statement"),
            Keywords::Return => write!(f, "Return statement"),
            Keywords::Function => write!(f, "Function statement"),
            Keywords::True => write!(f, "True statement"),
            Keywords::False => write!(f, "False statement"),
        }
    }
}

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Comparison::Less => write!(f, "<"),
            Comparison::Greater => write!(f, ">"),
            Comparison::LessE => write!(f, "<="),
            Comparison::GreaterE => write!(f, ">="),
            Comparison::Equal => write!(f, "=="),
            Comparison::NotEqual => write!(f, "=="),
        }
    }
}