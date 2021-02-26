use tokens::TokenTypes;

use crate::tokens;

pub struct Parser {
    token_vector: Vec<TokenTypes>,
    current_token: usize,
    next_token: usize,
}

struct VarStatement {
    token: tokens::TokenTypes,
    name: Identifier,
    value: String,
}

struct Identifier {
    identifier: tokens::TokenTypes,
    value: String,
}

struct ReturnStatement {
    value: String
}

#[derive(Clone)]
pub enum Expresion {
    NumberLit {
        number: i32,
    },

    BoolExp {
        value: bool,
    },
    
    InfixOp {
        left: Box<Expresion>,
        operator: tokens::TokenTypes,
        right: Box<Expresion>,
    },
    Prefix {
        operator: tokens::TokenTypes,
        right: Box<Expresion>,
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

    pub fn parse_line(&mut self) {
        match &self.token_vector[self.current_token] {
            tokens::TokenTypes::Keywords(tokens::Keywords::If) => {
                println!("if parsed");
            }

            tokens::TokenTypes::Keywords(tokens::Keywords::Var) => {
                let variable_statement = self.parse_variable();
                println!("token: {0}, name: {1}, value: {1}", variable_statement.token, variable_statement.value);
            }

            tokens::TokenTypes::Keywords(tokens::Keywords::Const) => {
                let const_statement = self.parse_constant();
            }

            tokens::TokenTypes::Keywords(tokens::Keywords::Return) => {
                let return_statement = self.parse_return();
                println!("{}", return_statement.value);
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

    fn expression_parser(&mut self) -> Expresion {
        let number;
        let mut precedence = 0;
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::NumbersInt(s) => number = s,
            _ => number = 0,
        }
        let mut left_op = Expresion::NumberLit { number: number };
        while &self.token_vector[self.next_token] != &tokens::TokenTypes::EndOfLine {
            left_op = self.infix_expression_parser(precedence, left_op);
            precedence = Parser::get_precedence(&self.token_vector[self.current_token]);
        }

        left_op
    }

    fn advance_tokens(&mut self) {
        self.current_token = self.next_token;
        self.next_token += 1;
    }

    fn parse_return(&mut self) -> ReturnStatement {
        if &self.token_vector.len() <= &self.next_token {
            println!("error, expected an expression");
            return 
            ReturnStatement {
                value: "error".to_string(),
            }
        }
        while &self.token_vector[self.next_token] != &tokens::TokenTypes::Semicolon {
            self.advance_tokens();
            if self.token_vector.len() == self.next_token {
                println!("error, expected an expression");
                return 
                ReturnStatement {
                    value: "error".to_string(),
                }
            }
            if &tokens::TokenTypes::EndOfLine == &self.token_vector[self.next_token] {
                println!("error, expected an expression");
                return ReturnStatement {
                    value: "error".to_string(),
                }
            }
        }
        ReturnStatement {
            value: "string".to_string(),
        }
    }

    fn parse_variable(&mut self) -> VarStatement {
        let token: tokens::TokenTypes;
        let identifier: Identifier;
        match &self.token_vector[self.next_token] {
            tokens::TokenTypes::Identifier(s) => {
                token = tokens::TokenTypes::Identifier(s.to_string());
            }
            tokens::TokenTypes::EndOfLine => {
                println!("error, expected an identifier");
                return VarStatement {
                    token: tokens::TokenTypes::Illegal,
                    name: Identifier {
                        identifier: tokens::TokenTypes::Illegal,
                        value: "<error>".to_string(),
                    },
                    value: "error".to_string(),
                };
            }
            _ => {
                println!("error, expected an identifier");
                return VarStatement {
                    token: tokens::TokenTypes::Illegal,
                    name: Identifier {
                        identifier: tokens::TokenTypes::Illegal,
                        value: "<error>".to_string(),
                    },
                    value: "error".to_string(),
                };
            }
        }
        self.advance_tokens();
        match &self.token_vector[self.next_token] {
            tokens::TokenTypes::Operator('=') => {
                identifier = Identifier {
                    identifier: tokens::TokenTypes::Operator('='),
                    value: "=".to_string(),
                }
            }
            tokens::TokenTypes::EndOfLine => {
                println!("error, expected = sign");
                return VarStatement {
                    token: tokens::TokenTypes::Illegal,
                    name: Identifier {
                        identifier: tokens::TokenTypes::Illegal,
                        value: "<error>".to_string(),
                    },
                    value: "error".to_string(),
                };
            }
            _ => {
                println!("error, expected = sign");
                return VarStatement {
                    token: tokens::TokenTypes::Illegal,
                    name: Identifier {
                        identifier: tokens::TokenTypes::Illegal,
                        value: "<error>".to_string(),
                    },
                    value: "error".to_string(),
                };
            }
        }
        self.advance_tokens();
        if &self.token_vector.len() <= &self.next_token {
            println!("error, expected an expression");
            return VarStatement {
                token: tokens::TokenTypes::Illegal,
                name: Identifier {
                    identifier: tokens::TokenTypes::Illegal,
                    value: "<error>".to_string(),
                },
                value: "error".to_string(),
            };
        }
        while &self.token_vector[self.next_token] != &tokens::TokenTypes::Semicolon {
            self.advance_tokens();
            if self.token_vector.len() == self.next_token {
                println!("error, expected an expression");
                return VarStatement {
                    token: tokens::TokenTypes::Illegal,
                    name: Identifier {
                        identifier: tokens::TokenTypes::Illegal,
                        value: "<error>".to_string(),
                    },
                    value: "error".to_string(),
                };
            }
            if &tokens::TokenTypes::EndOfLine == &self.token_vector[self.next_token] {
                println!("error, expected an expression");
                return VarStatement {
                    token: tokens::TokenTypes::Illegal,
                    name: Identifier {
                        identifier: tokens::TokenTypes::Illegal,
                        value: "<error>".to_string(),
                    },
                    value: "error".to_string(),
                };
            }
        }
        VarStatement {
            token: token,
            name: identifier,
            value: "string".to_string(),
        }
    }

    fn parse_constant(&mut self) {
        self.advance_tokens();
    }

    fn infix_expression_parser(&mut self, precedence: usize, left_op: Expresion) -> Expresion {
        let op;
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::Operator(s) => op = tokens::TokenTypes::Operator(s),
            _ => op = tokens::TokenTypes::Operator('-'),
        }
        self.advance_tokens();
        let next_precedence: usize = Parser::get_precedence(&self.token_vector[self.current_token]);
        let mut right_op;
        let num;
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::NumbersInt(s) => num = s,
            _ => num = 0,
        };
        right_op = Expresion::NumberLit { number: num };
        self.advance_tokens();
        if precedence < next_precedence {
            right_op = self.infix_expression_parser(next_precedence, right_op);
        }
        Expresion::InfixOp {
            left: Box::new(left_op.clone()),
            operator: op,
            right: Box::new(right_op.clone()),
        }
    }

    fn parse_boolean(&mut self) -> Expresion {
        let boolean = self.token_vector[self.current_token] == tokens::TokenTypes::Keywords(tokens::Keywords::True);
        Expresion::BoolExp {
            value: boolean,
        }
    }

    fn parse_grouped_expression(&mut self) -> Expresion {
        self.advance_tokens();
        let number;
        let mut precedence = 0;
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::NumbersInt(s) => number = s,
            _ => number = 0,
        }
        let mut left_op = Expresion::NumberLit { number: number };
        while &self.token_vector[self.next_token] != &tokens::TokenTypes::Operator(')') {
            left_op = self.infix_expression_parser(precedence, left_op);
            precedence = Parser::get_precedence(&self.token_vector[self.current_token]);
        }

        left_op
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
}
