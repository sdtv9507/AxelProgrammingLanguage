use crate::tokens;
use std::fmt;
use tokens::TokenTypes;

pub struct Parser {
    token_vector: Vec<TokenTypes>,
    current_token: usize,
    next_token: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    VarStatement {
        name: String,
        value: Box<Expression>,
    },

    ConstStatement {
        name: String,
        value: Box<Expression>,
    },

    ReturnStatement {
        value: Box<Expression>,
    },

    ExpressionStatement {
        value: Box<Expression>,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::VarStatement { name, value } => {
                write!(f, "Var Statement name: {0}, value: {1}", name, *value)
            }
            Statement::ConstStatement { name, value } => {
                write!(f, "Const Statement name: {0}, value: {1}", name, *value)
            }
            Statement::ReturnStatement { value } => {
                write!(f, "Return Statement value: {0}", *value)
            }
            Statement::ExpressionStatement { value } => {
                write!(f, "Expression Statement value: {0}", *value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    NumberLit {
        number: i32,
    },

    FloatLit {
        number: f32,
    },

    StringLit {
        string: String,
    },

    ArrayLit {
        elements: Vec<Expression>,
    },

    IndexExpression {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    HashMap {
        keys: Vec<Expression>,
        values: Vec<Expression>,
    },

    IdentifierLit {
        name: String,
    },

    BoolExp {
        value: bool,
    },

    IfExpr {
        condition: Box<Expression>,
        then: Box<Vec<Statement>>,
        other: Option<Box<Vec<Statement>>>,
    },

    FunctionExpr {
        identifier: String,
        parameters: Vec<String>,
        body: Box<Vec<Statement>>,
    },

    InfixOp {
        left: Box<Expression>,
        operator: tokens::TokenTypes,
        right: Box<Expression>,
    },

    CallExpr {
        identifier: String,
        parameters: Vec<Expression>,
    },

    Prefix {
        operator: tokens::TokenTypes,
        right: Box<Expression>,
    },

    VarChange {
        identifier: String,
        right: Box<Expression>,
    },

    CompoundOperation {
        identifier: String,
        operator: tokens::TokenTypes,
        right: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::NumberLit { number } => write!(f, "Number Literal: {0}", number),
            Expression::FloatLit { number } => write!(f, "Float Literal: {0}", number),
            Expression::StringLit { string } => write!(f, "String Literal: {0}", string),
            Expression::ArrayLit { elements } => {
                for i in elements {
                    write!(f, "Array Element: {0}", i)?;
                }
                return Ok(());
            }
            Expression::IndexExpression { left, right } => {
                write!(f, "Index Expression: left: {0}, right: {1}", left, right)
            }
            Expression::HashMap { keys, values } => {
                for (i, j) in keys.iter().zip(values) {
                    write!(f, "Hash Key Element: {0}", i)?;
                    write!(f, "Hash Value Element: {0}", j)?;
                }
                return Ok(());
            }
            Expression::IdentifierLit { name } => write!(f, "Identifier Literal: {0}", name),
            Expression::BoolExp { value } => write!(f, "Boolean Expression: {0}", value),
            Expression::IfExpr {
                condition,
                then: _,
                other: _,
            } => write!(f, "If Expression: condition: {0}", condition),

            Expression::FunctionExpr {
                identifier,
                parameters: _,
                body: _,
            } => write!(f, "Function Expression: identifier: {0}", identifier),
            Expression::InfixOp {
                left,
                operator,
                right,
            } => write!(
                f,
                "Infix Operation: left: {0}, operator: {1}, right: {2}",
                *left, operator, *right
            ),
            Expression::CallExpr {
                identifier,
                parameters,
            } => {
                write!(f, "Call Expression identifier: {0}", identifier)?;
                for i in parameters {
                    write!(f, "Call Expression Parameters: {0}", i)?;
                }
                return Ok(());
            }
            Expression::Prefix { operator, right } => write!(
                f,
                "Infix Operation: operator: {0}, right: {1}",
                operator, *right
            ),
            Expression::CompoundOperation {
                identifier,
                operator,
                right,
            } => write!(
                f,
                "Compound Operation: identifier: {0}, operator: {1}, right: {2}",
                identifier, operator, *right
            ),
            Expression::VarChange { identifier, right } => write!(
                f,
                "Change Variable: identifier: {0}, right: {1}",
                identifier, *right
            ),
        }
    }
}

impl Eq for Expression {}

impl Parser {
    pub fn new(line: Vec<tokens::TokenTypes>) -> Self {
        Parser {
            token_vector: line,
            current_token: 0,
            next_token: 1,
        }
    }

    pub fn parse_token_line<'a>(&mut self) -> Result<Vec<Statement>, &'a str> {
        let size = self.token_vector.len();
        let mut final_vector = Vec::new();
        while self.current_token < size {
            let statement = self.check_statement();
            match statement {
                Ok(s) => final_vector.push(s),
                Err(e) => return Err(e),
            }
            self.advance_tokens();
            if &self.token_vector[self.current_token] == &tokens::TokenTypes::EndOfLine {
                break;
            }
        }
        return Ok(final_vector);
    }

    pub fn check_statement<'a>(&mut self) -> Result<Statement, &'a str> {
        match &self.token_vector[self.current_token] {
            tokens::TokenTypes::Keywords(tokens::Keywords::Var) => {
                let variable_statement = self.parse_variable();
                match variable_statement {
                    Ok(s) => return Ok(s),
                    Err(e) => return Err(e),
                }
            }
            tokens::TokenTypes::Keywords(tokens::Keywords::Const) => {
                let constant_statement = self.parse_constant();
                match constant_statement {
                    Ok(s) => return Ok(s),
                    Err(e) => return Err(e),
                }
            }
            tokens::TokenTypes::Keywords(tokens::Keywords::Return) => {
                let return_statement = self.parse_return();
                match return_statement {
                    Ok(s) => return Ok(s),
                    Err(e) => return Err(e),
                }
            }

            _ => {
                let expression_statement = self.parse_expressions();
                match expression_statement {
                    Ok(s) => {
                        let statement = Statement::ExpressionStatement {
                            value: Box::from(s),
                        };
                        return Ok(statement);
                    }
                    Err(e) => return Err(e),
                }
            }
        }
    }

    fn parse_expressions<'a>(&mut self) -> Result<Expression, &'a str> {
        match &self.token_vector[self.current_token] {
            tokens::TokenTypes::Keywords(tokens::Keywords::If) => {
                let if_expression = self.parse_if();
                match if_expression {
                    Ok(s) => return Ok(s),
                    Err(e) => return Err(e),
                }
            }

            tokens::TokenTypes::Keywords(tokens::Keywords::Function) => {
                let function_expression = self.parse_function();
                match function_expression {
                    Ok(s) => return Ok(s),
                    Err(e) => return Err(e),
                }
            }

            tokens::TokenTypes::Keywords(tokens::Keywords::True) => {
                let true_expression = self.parse_boolean();
                Ok(true_expression)
            }

            tokens::TokenTypes::Keywords(tokens::Keywords::False) => {
                let false_expression = self.parse_boolean();
                Ok(false_expression)
            }

            tokens::TokenTypes::Identifier(s) => {
                let name = s.clone();
                match self.token_vector[self.next_token] {
                    tokens::TokenTypes::Delim('[') => {
                        self.advance_tokens();
                        self.advance_tokens();
                        let right = self.expression_parser(&tokens::TokenTypes::Delim(']'));
                        match right {
                            Ok(t) => {
                                return Ok(Expression::IndexExpression {
                                    left: Box::new(Expression::IdentifierLit { name }),
                                    right: Box::new(t),
                                })
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    tokens::TokenTypes::Operator('(') => {
                        return self.parse_call();
                    }
                    tokens::TokenTypes::CompoundOperator(t) => {
                        self.advance_tokens();
                        let rt = self.parse_loop_expressions()?;
                        let compound_op = Expression::CompoundOperation {
                            identifier: name.clone(),
                            operator: tokens::TokenTypes::CompoundOperator(t),
                            right: Box::from(rt),
                        };
                        return Ok(Expression::VarChange {
                            identifier: name.clone(),
                            right: Box::from(compound_op),
                        });
                    }
                    _ => return Err("wrong identifier statement"),
                }
            }
            //tokens::TokenTypes::Comment => {
            //    println!("This is a comment: Line ignored");
            //}
            _ => {
                let expression = self.expression_parser(&tokens::TokenTypes::Semicolon);
                match expression {
                    Ok(s) => return Ok(s),
                    Err(e) => return Err(e),
                }
            }
        }
    }

    fn advance_tokens(&mut self) {
        self.current_token = self.next_token;
        self.next_token += 1;
    }

    fn parse_variable<'a>(&mut self) -> Result<Statement, &'a str> {
        let identifier: String;
        self.advance_tokens();
        match &self.token_vector[self.current_token] {
            tokens::TokenTypes::Identifier(s) => identifier = s.clone(),
            _ => {
                return Err("expected an identifier");
            }
        }
        self.advance_tokens();
        if self.match_current_operator('=') == false {
            return Err("expected a = sign, variable must be initialized");
        }
        if &self.token_vector.len() <= &self.next_token {
            return Err("expected an expression (parser.rs, line 221)");
        }

        let result_op: Expression = self.parse_loop_expressions()?;
        Ok(Statement::VarStatement {
            name: identifier,
            value: Box::from(result_op),
        })
    }

    fn parse_constant<'a>(&mut self) -> Result<Statement, &'a str> {
        let identifier: String;
        self.advance_tokens();
        match &self.token_vector[self.next_token] {
            tokens::TokenTypes::Operator('=') => identifier = "=".to_string(),
            tokens::TokenTypes::EndOfLine => {
                return Err("error, expected = sign");
            }
            _ => {
                return Err("error, expected an identifier");
            }
        }
        self.advance_tokens();
        if &self.token_vector.len() <= &self.next_token {
            return Err("expected an expression (parser.rs, line 272)");
        }

        let result_op: Expression = self.parse_loop_expressions()?;
        Ok(Statement::ConstStatement {
            name: identifier,
            value: Box::from(result_op),
        })
    }

    fn parse_return<'a>(&mut self) -> Result<Statement, &'a str> {
        if &self.token_vector.len() <= &self.next_token {
            return Err("expected an expression (parser.rs, line 311)");
        }
        let result_op: Expression = self.parse_loop_expressions()?;
        Ok(Statement::ReturnStatement {
            value: Box::from(result_op),
        })
    }

    fn parse_if<'a>(&mut self) -> Result<Expression, &'a str> {
        self.advance_tokens();
        if self.match_current_operator('(') == false {
            return Err("Error, expected (");
        }
        self.advance_tokens();

        let condition_result = self.expression_parser(&tokens::TokenTypes::Operator(')'));
        let condition;
        match condition_result {
            Ok(v) => condition = Box::new(v),
            Err(e) => return Err(e),
        }

        if self.match_current_operator(')') == false {
            return Err("Error, expected )");
        }

        self.advance_tokens();

        if self.match_current_delim('{') == false {
            return Err("Error, expected {");
        }

        self.advance_tokens();

        let consequence = Box::new(self.parse_statement(&tokens::TokenTypes::Delim('}'))?);

        self.advance_tokens();
        if self.match_current_delim('}') == false {
            return Err("Error, expected }");
        }

        let then: Option<Box<Vec<Statement>>>;
        match self.token_vector[self.next_token] {
            tokens::TokenTypes::Keywords(tokens::Keywords::Else) => {
                self.advance_tokens();
                self.advance_tokens();
                self.advance_tokens();
                let then_box = Box::new(self.parse_statement(&tokens::TokenTypes::Delim('}'))?);
                self.advance_tokens();
                if self.match_current_delim('}') == false {
                    return Err("Error, expected }");
                }
                then = Some(then_box);
            }
            _ => {
                then = None;
            }
        }
        Ok(Expression::IfExpr {
            condition: condition,
            then: consequence,
            other: then,
        })
    }

    fn parse_prefix_expressions<'a>(&mut self) -> Result<Expression, &'a str> {
        match &self.token_vector[self.current_token] {
            tokens::TokenTypes::NumbersInt(s) => {
                return Ok(Expression::NumberLit { number: s.clone() })
            }
            tokens::TokenTypes::NumbersFloat(s) => {
                return Ok(Expression::FloatLit { number: s.clone() })
            }
            tokens::TokenTypes::Strings(s) => {
                return Ok(Expression::StringLit { string: s.clone() })
            }
            tokens::TokenTypes::Identifier(s) => {
                let name = s.clone();

                match self.token_vector[self.next_token] {
                    tokens::TokenTypes::Delim('[') => {
                        self.advance_tokens();
                        self.advance_tokens();
                        let right = self.expression_parser(&tokens::TokenTypes::Delim(']'));
                        match right {
                            Ok(t) => {
                                return Ok(Expression::IndexExpression {
                                    left: Box::new(Expression::IdentifierLit { name }),
                                    right: Box::new(t),
                                })
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    tokens::TokenTypes::Operator('(') => {
                        return self.parse_call();
                    }
                    _ => return Ok(Expression::IdentifierLit { name }),
                }
            }
            tokens::TokenTypes::Operator('(') => {
                self.advance_tokens();
                return self.expression_parser(&tokens::TokenTypes::Operator(')'));
            }
            tokens::TokenTypes::Delim('[') => {
                let elements: Vec<Expression> =
                    self.parse_comma_separation(&tokens::TokenTypes::Delim(']'))?;
                return Ok(Expression::ArrayLit { elements });
            }
            tokens::TokenTypes::Delim('{') => {
                let mut keys: Vec<Expression> = Vec::new();
                let mut values: Vec<Expression> = Vec::new();
                while self.match_current_delim('}') == false {
                    self.advance_tokens();
                    keys.push(self.parse_prefix_expressions()?);
                    self.advance_tokens();
                    match self.token_vector[self.current_token] {
                        tokens::TokenTypes::Colon => self.advance_tokens(),
                        _ => return Err("expected a : (colon)"),
                    }
                    values.push(self.parse_prefix_expressions()?);
                    match self.token_vector[self.next_token] {
                        tokens::TokenTypes::Comma => self.advance_tokens(),
                        tokens::TokenTypes::Delim('}') => break,
                        _ => return Err("expected a , (comma) or } (right brace"),
                    }
                }
                self.advance_tokens();
                return Ok(Expression::HashMap { keys, values });
            }
            tokens::TokenTypes::Operator('-') => {
                self.advance_tokens();
                let parse_exp = self.parse_prefix_expressions();
                let expression;
                match parse_exp {
                    Ok(s) => expression = s,
                    Err(e) => return Err(e),
                }
                return Ok(Expression::Prefix {
                    operator: tokens::TokenTypes::Operator('-'),
                    right: Box::new(expression),
                });
            }
            tokens::TokenTypes::Bang => {
                self.advance_tokens();
                let parse_exp = self.parse_prefix_expressions();
                let expression;
                match parse_exp {
                    Ok(s) => expression = s,
                    Err(e) => return Err(e),
                }
                return Ok(Expression::Prefix {
                    operator: tokens::TokenTypes::Bang,
                    right: Box::new(expression),
                });
            }
            tokens::TokenTypes::Keywords(tokens::Keywords::True) => {
                return Ok(Expression::BoolExp { value: true })
            }
            tokens::TokenTypes::Keywords(tokens::Keywords::False) => {
                return Ok(Expression::BoolExp { value: false })
            }
            _ => return Err("expected an expression (parser.rs, line 480)"),
        }
    }

    fn parse_function<'a>(&mut self) -> Result<Expression, &'a str> {
        let identifier: String;
        match &self.token_vector[self.next_token] {
            tokens::TokenTypes::Identifier(s) => identifier = s.clone(),
            _ => return Err("Error, expected an identifier"),
        }

        self.advance_tokens();
        self.advance_tokens();

        if self.match_current_operator('(') == false {
            return Err("Error, expected (");
        }
        self.advance_tokens();

        let mut parameters: Vec<String> = Vec::new();

        while self.match_current_operator(')') == false {
            match &self.token_vector[self.current_token] {
                tokens::TokenTypes::Identifier(s) => parameters.push(s.clone()),
                _ => return Err("expected an identifier"),
            }
            self.advance_tokens();

            if self.match_current_operator(')') == false {
                match &self.token_vector[self.current_token] {
                    tokens::TokenTypes::Comma => self.advance_tokens(),
                    _ => return Err("expected a comma"),
                }
            }
        }

        self.advance_tokens();

        if self.match_current_delim('{') == false {
            return Err("Error, expected {");
        }

        self.advance_tokens();

        let statement_result = self.parse_statement(&tokens::TokenTypes::Delim('}'));
        let statement;

        match statement_result {
            Ok(v) => statement = Box::new(v),
            _ => return Err("error parsing statement"),
        }

        self.advance_tokens();
        if self.match_current_delim('}') == false {
            return Err("Error, expected }");
        }

        Ok(Expression::FunctionExpr {
            identifier: identifier,
            parameters: parameters,
            body: statement,
        })
    }

    fn parse_boolean(&mut self) -> Expression {
        let boolean = self.token_vector[self.current_token]
            == tokens::TokenTypes::Keywords(tokens::Keywords::True);
        Expression::BoolExp { value: boolean }
    }

    fn parse_call<'a>(&mut self) -> Result<Expression, &'a str> {
        let identifier;
        match &self.token_vector[self.current_token] {
            tokens::TokenTypes::Identifier(s) => identifier = s.clone(),
            _ => return Err("expected an identifier"),
        }

        self.advance_tokens();
        if self.match_current_operator('(') == false {
            return Err("expected (");
        }

        let parameters = self.parse_comma_separation(&tokens::TokenTypes::Operator(')'))?;

        Ok(Expression::CallExpr {
            identifier,
            parameters,
        })
    }

    fn parse_comma_separation<'a>(
        &mut self,
        delimiter: &tokens::TokenTypes,
    ) -> Result<Vec<Expression>, &'a str> {
        let mut parameters: Vec<Expression> = Vec::new();
        while &self.token_vector[self.current_token] != delimiter {
            self.advance_tokens();
            let left_op;
            let parsed_prefix = self.parse_prefix_expressions();
            match parsed_prefix {
                Ok(s) => left_op = s,
                Err(e) => return Err(e),
            }

            self.advance_tokens();
            if &self.token_vector[self.current_token] != &tokens::TokenTypes::Comma {
                let result_op = self.infix_expression_parser(0, left_op);
                match result_op {
                    Ok(s) => parameters.push(s.clone()),
                    Err(e) => return Err(e),
                }
            } else {
                parameters.push(left_op);
            }
        }
        return Ok(parameters);
    }

    fn parse_statement<'a>(
        &mut self,
        delimiter: &tokens::TokenTypes,
    ) -> Result<Vec<Statement>, &'a str> {
        let mut statement: Vec<Statement> = Vec::new();

        loop {
            let check_statement = self.check_statement();
            match check_statement {
                Ok(s) => statement.push(s),
                Err(e) => return Err(e),
            }
            if &self.token_vector[self.next_token] == delimiter {
                break;
            }
            self.advance_tokens();
        }

        Ok(statement)
    }

    fn expression_parser<'a>(
        &mut self,
        delimiter: &tokens::TokenTypes,
    ) -> Result<Expression, &'a str> {
        let mut precedence = 0;
        let mut left_op;
        let op = self.parse_prefix_expressions();
        match op {
            Ok(s) => left_op = s,
            Err(e) => return Err(e),
        }
        loop {
            self.advance_tokens();
            println!("delimiter {}", delimiter);
            println!("left {}", left_op);
            println!("token {}", self.token_vector[self.current_token]);
            let result_op = self.infix_expression_parser(precedence, left_op);
            match result_op {
                Ok(v) => left_op = v,
                Err(e) => return Err(e),
            };
            if &self.token_vector[self.current_token] == delimiter {
                break;
            }
            precedence = Parser::get_precedence(&self.token_vector[self.current_token]);
        }

        Ok(left_op)
    }

    fn infix_expression_parser<'a>(
        &mut self,
        precedence: usize,
        left_op: Expression,
    ) -> Result<Expression, &'a str> {
        let op;
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::Operator(s) => op = tokens::TokenTypes::Operator(s),
            tokens::TokenTypes::CompoundOperator(s) => op = tokens::TokenTypes::CompoundOperator(s),
            tokens::TokenTypes::Compare(tokens::Comparison::Equal) => {
                op = tokens::TokenTypes::Compare(tokens::Comparison::Equal)
            }
            tokens::TokenTypes::Compare(tokens::Comparison::NotEqual) => {
                op = tokens::TokenTypes::Compare(tokens::Comparison::NotEqual)
            }
            tokens::TokenTypes::Compare(tokens::Comparison::Less) => {
                op = tokens::TokenTypes::Compare(tokens::Comparison::Less)
            }
            tokens::TokenTypes::Compare(tokens::Comparison::LessE) => {
                op = tokens::TokenTypes::Compare(tokens::Comparison::LessE)
            }
            tokens::TokenTypes::Compare(tokens::Comparison::Greater) => {
                op = tokens::TokenTypes::Compare(tokens::Comparison::Greater)
            }
            tokens::TokenTypes::Compare(tokens::Comparison::GreaterE) => {
                op = tokens::TokenTypes::Compare(tokens::Comparison::GreaterE)
            }
            _ => return Ok(left_op),
        }

        let next_precedence: usize = Parser::get_precedence(&op);
        if next_precedence > precedence
            && &self.token_vector[self.next_token] != &tokens::TokenTypes::Semicolon
        {
            self.advance_tokens();
            let mut right_op;
            let prefix_op = self.parse_prefix_expressions();
            match prefix_op {
                Ok(s) => right_op = s,
                Err(e) => return Err(e),
            }

            self.advance_tokens();
            let result_op = self.infix_expression_parser(next_precedence, right_op);
            match result_op {
                Ok(v) => right_op = v,
                Err(e) => return Err(e),
            };
            return Ok(Expression::InfixOp {
                left: Box::new(left_op.clone()),
                operator: op,
                right: Box::new(right_op.clone()),
            });
        } else {
            return Ok(left_op);
        }
    }

    fn get_precedence(token: &tokens::TokenTypes) -> usize {
        match token {
            tokens::TokenTypes::Operator('+') => 2,
            tokens::TokenTypes::Operator('-') => 2,
            tokens::TokenTypes::Operator('*') => 3,
            tokens::TokenTypes::Operator('/') => 3,
            tokens::TokenTypes::Compare(_s) => 1,
            _ => 0,
        }
    }

    fn match_current_operator(&mut self, token: char) -> bool {
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::Operator(s) if s == token => true,
            _ => false,
        }
    }

    fn match_current_delim(&mut self, token: char) -> bool {
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::Delim(s) if s == token => true,
            _ => false,
        }
    }

    fn parse_loop_expressions<'a>(&mut self) -> Result<Expression, &'a str> {
        let mut result_op: Expression;
        self.advance_tokens();
        let mut left_op;
        let op = self.parse_prefix_expressions();
        match op {
            Ok(s) => left_op = s,
            Err(e) => return Err(e),
        }
        self.advance_tokens();
        if &self.token_vector[self.current_token] == &tokens::TokenTypes::Semicolon {
            return Ok(left_op);
        }
        loop {
            if self.token_vector.len() <= self.next_token {
                return Err("token overflow on return parse");
            }
            if &tokens::TokenTypes::EndOfLine == &self.token_vector[self.next_token] {
                return Err("reached end of line without semicolon on return parse");
            }
            result_op = self.infix_expression_parser(0, left_op)?;
            if &self.token_vector[self.current_token] == &tokens::TokenTypes::Semicolon {
                break;
            }
            left_op = result_op;
        }
        return Ok(result_op);
    }
}
