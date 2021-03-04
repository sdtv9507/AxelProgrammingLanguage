use std::process;
use tokens::TokenTypes;

use crate::tokens;

pub struct Parser {
    token_vector: Vec<TokenTypes>,
    current_token: usize,
    next_token: usize,
}

#[derive(Clone)]
pub enum Statement {
    VarStatement {
        token: tokens::TokenTypes,
        name: String,
        value: String,
    },

    ReturnStatement {
        value: String,
    },
}

#[derive(Clone)]
pub enum Expression {
    NumberLit {
        number: i32,
    },

    BoolExp {
        value: bool,
    },

    IfExpr {
        condition: Box<Expression>,
        then: Box<Statement>,
        other: Option<Box<Statement>>,
    },

    FunctionExpr {
        identifier: String,
        parameters: Vec<String>,
        body: Box<Statement>,
    },

    InfixOp {
        left: Box<Expression>,
        operator: tokens::TokenTypes,
        right: Box<Expression>,
    },
    Prefix {
        operator: tokens::TokenTypes,
        right: Box<Expression>,
    },
}

impl Parser {
    pub fn new(line: Vec<tokens::TokenTypes>) -> Self {
        Parser {
            token_vector: line,
            current_token: 0,
            next_token: 1,
        }
    }

    pub fn check_statement<'a>(&mut self) -> Result<Statement, &'a str> {
        match &self.token_vector[self.next_token] {
            tokens::TokenTypes::Keywords(tokens::Keywords::Var) => {
                let variable_statement = self.parse_variable();
                match variable_statement {
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

            _ => return Err("error, expected a statement"),
        }
    }

    pub fn parse_line(&mut self) {
        match &self.token_vector[self.current_token] {
            tokens::TokenTypes::Keywords(tokens::Keywords::If) => {
                let if_statement = self.parse_if();
            }

            tokens::TokenTypes::Keywords(tokens::Keywords::Var) => {
                let variable_statement = self.parse_variable();
                //println!("token: {0}, name: {1}, value: {1}", variable_statement.token, variable_statement.value);
            }

            tokens::TokenTypes::Keywords(tokens::Keywords::Const) => {
                let const_statement = self.parse_constant();
            }

            tokens::TokenTypes::Keywords(tokens::Keywords::Function) => {
                let function_statement = self.parse_function().unwrap_or_else(|err| {
                    println!("Error: {}", err);
                    process::exit(1);
                });
                //println!("{}", return_statement.value);
            }

            tokens::TokenTypes::Keywords(tokens::Keywords::Return) => {
                let return_statement = self.parse_return();
                //println!("{}", return_statement.value);
            }

            tokens::TokenTypes::Identifier(s) => {
                println!("identifier: {}", s);
            }

            tokens::TokenTypes::Comment => {
                println!("This is a comment: Line ignored");
            }
            _ => {
                self.expression_parser();
                //Parser::parse_expression(&self.token_vector);
            }
        }
    }

    fn parse_function<'a>(& mut self) -> Result<Expression, &'a str> {
        let identifier: String;
        match &self.token_vector[self.next_token] {
            tokens::TokenTypes::Identifier(s) => identifier = s.clone(),
            _ => return Err("Error, expected an identifier"),
        }

        self.advance_tokens();

        if self.match_operator('(') == false {
            return Err("Error, expected (");
        }
        self.advance_tokens();

        let mut parameters: Vec<String> = Vec::new();

        while self.match_operator(')') == false {
            match &self.token_vector[self.next_token] {
                tokens::TokenTypes::Identifier(s) => parameters.push(s.clone()),
                _ => return Err("Error, expected an identifier"),
            }
            self.advance_tokens();
    
            if self.match_operator(')') == false {
                match &self.token_vector[self.next_token] {
                    tokens::TokenTypes::Comma => self.advance_tokens(),
                    _ => return Err("Error, expected an identifier"),
                }
            }
        }

        self.advance_tokens();
        
        if self.match_delim('{') == false {
            return Err("Error, expected {");
        }

        let statement_result = self.parse_statement();
        let statement;

        match statement_result {
            Ok(v) => statement = Box::new(v),
            _ => return Err("error parsing statement"),
        }

        if self.match_delim('}') == false {
            return Err("Error, expected }");
        }

        Ok(Expression::FunctionExpr {
            identifier: identifier,
            parameters: parameters,
            body: statement,
        })
    }

    fn parse_if<'a>(&mut self) -> Result<Expression, &'a str> {
        if self.match_operator('(') == false {
            return Err("Error, expected (");
        }
        self.advance_tokens();

        let condition_result = self.expression_parser();
        let condition;
        match condition_result {
            Ok(v) => condition = Box::new(v),
            _ => return Err("error parsing condition statement"),
        }

        if self.match_operator(')') == false {
            return Err("Error, expected )");
        }

        self.advance_tokens();

        if self.match_delim('{') == false {
            return Err("Error, expected {");
        }

        self.advance_tokens();
        self.advance_tokens();

        let consequence_result = self.parse_statement();
        let consequence;

        match consequence_result {
            Ok(v) => consequence = Box::new(v),
            _ => return Err("error parsing else"),
        }

        if self.match_delim('}') == false {
            return Err("Error, expected }");
        }

        let then: Option<Box<Statement>>;
        match self.token_vector[self.next_token] {
            tokens::TokenTypes::Keywords(tokens::Keywords::Else) => {
                let then_result = self.parse_statement();
                let then_box;
                match then_result {
                    Ok(v) => then_box = Box::new(v),
                    _ => return Err("error parsing else statement"),
                }

                if self.match_delim('}') == false {
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

    fn parse_statement<'a>(&mut self) -> Result<Statement, &'a str> {
        self.advance_tokens();
        let mut statement = Statement::ReturnStatement {
            value: "".to_string(),
        };

        while &self.token_vector[self.next_token] != &tokens::TokenTypes::Delim('}') {
            let check_statement = self.check_statement();
            match check_statement {
                Ok(s) => statement = s,
                Err(e) => return Err(e),
            }
            self.advance_tokens();
        }

        Ok(statement)
    }

    fn expression_parser<'a>(&mut self) -> Result<Expression, &'a str> {
        let number;
        let mut precedence = 0;
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::NumbersInt(s) => number = s,
            _ => number = 0,
        }
        let mut left_op = Expression::NumberLit { number: number };
        while &self.token_vector[self.next_token] != &tokens::TokenTypes::EndOfLine {
            let result_op = self.infix_expression_parser(precedence, left_op);
            match result_op {
                Ok(v) => left_op = v,
                _ => return Err("error, expected a number"),
            };
            precedence = Parser::get_precedence(&self.token_vector[self.current_token]);
        }

        Ok(left_op)
    }

    fn advance_tokens(&mut self) {
        self.current_token = self.next_token;
        self.next_token += 1;
    }

    fn parse_return<'a>(&mut self) -> Result<Statement, &'a str> {
        if &self.token_vector.len() <= &self.next_token {
            return Err("error, expected an expression");
        }
        while &self.token_vector[self.next_token] != &tokens::TokenTypes::Semicolon {
            self.advance_tokens();
            if self.token_vector.len() == self.next_token {
                return Err("error, expected an expression");
            }
            if &tokens::TokenTypes::EndOfLine == &self.token_vector[self.next_token] {
                return Err("error, expected an expression");
            }
        }
        Ok(Statement::ReturnStatement {
            value: "string".to_string(),
        })
    }

    fn parse_variable<'a>(&mut self) -> Result<Statement, &'a str> {
        let token: tokens::TokenTypes;
        let identifier: String;
        match &self.token_vector[self.next_token] {
            tokens::TokenTypes::Identifier(s) => {
                token = tokens::TokenTypes::Identifier(s.to_string());
            }
            tokens::TokenTypes::EndOfLine => {
                return Err("error, expected an identifier");
            }
            _ => {
                return Err("error, expected an identifier");
            }
        }
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
            return Err("error, expected an expression");
        }
        while &self.token_vector[self.next_token] != &tokens::TokenTypes::Semicolon {
            self.advance_tokens();
            if self.token_vector.len() == self.next_token {
                return Err("error, expected an expression");
            }
            if &tokens::TokenTypes::EndOfLine == &self.token_vector[self.next_token] {
                return Err("error, expected an expression");
            }
        }
        Ok(Statement::VarStatement {
            token: token,
            name: identifier,
            value: "string".to_string(),
        })
    }

    fn parse_constant(&mut self) {
        self.advance_tokens();
    }

    fn infix_expression_parser<'a>(
        &mut self,
        precedence: usize,
        left_op: Expression,
    ) -> Result<Expression, &'a str> {
        let op;
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::Operator(s) => op = tokens::TokenTypes::Operator(s),
            _ => return Err("error, expected an operator"),
        }
        self.advance_tokens();
        let next_precedence: usize = Parser::get_precedence(&self.token_vector[self.current_token]);
        let mut right_op;
        let num;
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::NumbersInt(s) => num = s,
            _ => return Err("error, expected a number"),
        };
        right_op = Expression::NumberLit { number: num };
        self.advance_tokens();
        if precedence < next_precedence {
            let result_op = self.infix_expression_parser(next_precedence, right_op);
            match result_op {
                Ok(v) => right_op = v,
                _ => return Err("error, expected a number"),
            };
        }
        Ok(Expression::InfixOp {
            left: Box::new(left_op.clone()),
            operator: op,
            right: Box::new(right_op.clone()),
        })
    }

    fn parse_boolean(&mut self) -> Expression {
        let boolean = self.token_vector[self.current_token]
            == tokens::TokenTypes::Keywords(tokens::Keywords::True);
        Expression::BoolExp { value: boolean }
    }

    fn parse_grouped_expression<'a>(&mut self) -> Result<Expression, &'a str> {
        self.advance_tokens();
        let number;
        let mut precedence = 0;
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::NumbersInt(s) => number = s,
            _ => number = 0,
        }
        let mut left_op = Expression::NumberLit { number: number };
        while &self.token_vector[self.next_token] != &tokens::TokenTypes::Operator(')') {
            let result_op = self.infix_expression_parser(precedence, left_op);
            match result_op {
                Ok(v) => left_op = v,
                _ => return Err("error, expected a number"),
            };
            precedence = Parser::get_precedence(&self.token_vector[self.current_token]);
        }

        Ok(left_op)
    }

    fn parse_expression(line: &Vec<tokens::TokenTypes>) {
        let mut index = 0;
        let mut operation: Vec<&char> = Vec::new();
        let mut num_vector = Vec::new();
        let mut operation_index = 0;
        let mut num_vector_index = 0;
        loop {
            match &line[index] {
                tokens::TokenTypes::NumbersInt(s) => {
                    num_vector.insert(num_vector_index, s);
                    num_vector_index += 1;
                }

                tokens::TokenTypes::Operator(s) => {
                    operation.insert(operation_index, s);
                    operation_index += 1;
                }

                _ => {
                    println!("illegal expression");
                }
            }
            index += 1;
            if index > line.len() - 1 {
                break;
            }
        }
        Parser::operate(&num_vector, &operation);
    }

    fn operate(numbers_vector: &Vec<&i32>, operators: &Vec<&char>) {
        let mut operations: usize = 0;
        let mut final_value = 0;
        let mut n = 0;
        if operators.len() == 0 {
            final_value = 0;
        } else if operators.len() == 1 {
            let number1 = numbers_vector[operations];
            let number2 = &numbers_vector[&operations + 1];
            match operators[operations] {
                '+' => {
                    n = Parser::add(number1, number2);
                }
                '-' => {
                    n = Parser::minus(number1, number2);
                }
                _ => println!("Error"),
            }
            final_value = n;
        } else {
            while operations <= operators.len() - 1 {
                if operations == 0 {
                    final_value += numbers_vector[operations];
                }
                let number2 = &numbers_vector[&operations + 1];
                match operators[operations] {
                    '+' => {
                        n = Parser::add(&final_value, number2);
                    }
                    '-' => {
                        n = Parser::minus(&final_value, number2);
                    }
                    '*' => {
                        n = Parser::mult(&final_value, number2);
                    }
                    '/' => {
                        n = Parser::divide(&final_value, number2);
                    }
                    _ => break,
                }
                final_value = n;
                operations += 1;
            }
        }
        println!("Result: {}", final_value);
    }

    fn add(num1: &i32, num2: &i32) -> i32 {
        num1 + num2
    }

    fn minus(num1: &i32, num2: &i32) -> i32 {
        num1 - num2
    }

    fn mult(num1: &i32, num2: &i32) -> i32 {
        num1 * num2
    }

    fn divide(num1: &i32, num2: &i32) -> i32 {
        num1 / num2
    }

    fn get_precedence(token: &tokens::TokenTypes) -> usize {
        match token {
            tokens::TokenTypes::Operator('+') => 1,
            tokens::TokenTypes::Operator('-') => 1,
            tokens::TokenTypes::Operator('*') => 2,
            tokens::TokenTypes::Operator('/') => 2,
            _ => 0,
        }
    }

    fn match_operator(&mut self, token: char) -> bool {
        match self.token_vector[self.next_token] {
            tokens::TokenTypes::Operator(s) if s == token => true,
            _ => false,
        }
    }

    fn match_delim(&mut self, token: char) -> bool {
        match self.token_vector[self.next_token] {
            tokens::TokenTypes::Delim(s) if s == token => true,
            _ => false,
        }
    }
}
