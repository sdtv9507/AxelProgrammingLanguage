use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub enum TokenTypes {
    Operator(char),
    CompoundOperator(char),
    Identifier(String),
    Keywords(Keywords),
    NumbersInt(i32),
    NumbersFloat(f32),
    Strings(String),
    Delim(char),
    Compare(Comparison),
    Bang,
    Comma,
    Colon,
    Semicolon,
    Comment,
    EndOfLine,
    Illegal,
}

impl Eq for TokenTypes {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keywords {
    Var,
    Class,
    Const,
    If,
    Else,
    While,
    Return,
    Function,
    True,
    False,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            TokenTypes::CompoundOperator(s) => write!(f, "Compound Operator {}", s),
            TokenTypes::Identifier(s) => write!(f, "Identifier {}", s),
            TokenTypes::Keywords(s) => write!(f, "Keyword {}", s),
            TokenTypes::NumbersInt(s) => write!(f, "Number {}", s),
            TokenTypes::NumbersFloat(s) => write!(f, "Float Number {}", s),
            TokenTypes::Strings(s) => write!(f, "String {}", s),
            TokenTypes::Delim(s) => write!(f, "Delim {}", s),
            TokenTypes::Compare(s) => write!(f, "Compare {}", s),
            TokenTypes::Bang => write!(f, "Bang"),
            TokenTypes::Comma => write!(f, "Comma"),
            TokenTypes::Colon => write!(f, "Colon"),
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
            Keywords::Class => write!(f, "Class declaration"),
            Keywords::Const => write!(f, "Constant declaration"),
            Keywords::If => write!(f, "If statement"),
            Keywords::Else => write!(f, "Else statement"),
            Keywords::While => write!(f, "While statement"),
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