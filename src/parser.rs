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
        value: Box<Expression>,
    },

    ConstStatement {
        token: tokens::TokenTypes,
        name: String,
        value: String,
    },

    ReturnStatement {
        value: Box<Expression>,
    },

    ExpressionStatement {
        value: Box<Expression>,
    },
}

#[derive(Clone)]
pub enum Expression {
    NumberLit {
        number: i32,
    },

    StringLit {
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

    CallExpr {
        name: String,
        expressions: Vec<Expression>,
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
                let call_expression = self.parse_call();
                match call_expression {
                    Ok(s) => return Ok(s),
                    Err(e) => return Err(e),
                }
            }

            //tokens::TokenTypes::Comment => {
            //    println!("This is a comment: Line ignored");
            //}
            _ => {
                let expression = self.expression_parser();
                match expression {
                    Ok(s) => return Ok(s),
                    Err(e) => return Err(e),
                }
            }
        }
    }

    fn parse_prefix_expressions<'a>(&mut self) -> Result<Expression, &'a str> {
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::NumbersInt(s) => return Ok(Expression::NumberLit { number: s }),
            tokens::TokenTypes::Operator('-') => {
                let parse_exp = self.parse_expressions();
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
            _ => return Err("expected an expression"),
        }
    }

    fn parse_call<'a>(&mut self) -> Result<Expression, &'a str> {
        let name;
        match &self.token_vector[self.current_token] {
            tokens::TokenTypes::Identifier(s) => name = s.clone(),
            _ => return Err("Error, expected an identifier"),
        }

        if self.match_operator('(') == false {
            return Err("Error, expected (");
        }

        self.advance_tokens();
        let mut expressions: Vec<Expression> = Vec::new();
        while &self.token_vector[self.current_token] != &tokens::TokenTypes::Operator(')') {
            let left_op;
            let parsed_prefix = self.parse_prefix_expressions();
            match parsed_prefix {
                Ok(s) => left_op = s,
                Err(e) => return Err(e),
            }
            let result_op = self.infix_expression_parser(0, left_op);
            match result_op {
                Ok(s) => expressions.push(s.clone()),
                Err(e) => return Err(e),
            }

            if &self.token_vector[self.current_token] == &tokens::TokenTypes::Comma {
                self.advance_tokens();
                self.advance_tokens();
            }
        }

        Ok(Expression::CallExpr { name, expressions })
    }

    fn parse_function<'a>(&mut self) -> Result<Expression, &'a str> {
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
        let mut statement: Statement;

        loop {
            let check_statement = self.check_statement();
            match check_statement {
                Ok(s) => statement = s,
                Err(e) => return Err(e),
            }
            if &self.token_vector[self.next_token] == &tokens::TokenTypes::Delim('}') {
                break;
            }
            self.advance_tokens();
        }

        Ok(statement)
    }

    fn expression_parser<'a>(&mut self) -> Result<Expression, &'a str> {
        let mut precedence = 0;
        let mut left_op;
        let op = self.parse_prefix_expressions();
        match op {
            Ok(s) => left_op = s,
            Err(e) => return Err(e),
        }
        while &self.token_vector[self.next_token] != &tokens::TokenTypes::EndOfLine {
            let result_op = self.infix_expression_parser(precedence, left_op);
            match result_op {
                Ok(v) => left_op = v,
                Err(e) => return Err(e),
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
        let mut result_op: Expression;
        loop {
            self.advance_tokens();
            if self.token_vector.len() == self.next_token {
                return Err("error, expected an expression");
            }
            if &tokens::TokenTypes::EndOfLine == &self.token_vector[self.next_token] {
                return Err("error, expected an expression");
            }
            let left_op;
            let op = self.parse_prefix_expressions();
            match op {
                Ok(s) => left_op = s,
                Err(e) => return Err(e),
            }
            result_op = self.infix_expression_parser(0, left_op)?;
            if &self.token_vector[self.next_token] == &tokens::TokenTypes::Semicolon {
                break;
            }
        }
        Ok(Statement::ReturnStatement {
            value: Box::from(result_op),
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

        let mut result_op;
        loop {
            self.advance_tokens();
            if self.token_vector.len() <= self.next_token {
                return Err("error, token overflow");
            }
            if &tokens::TokenTypes::EndOfLine == &self.token_vector[self.next_token] {
                return Err("error, reached end of line without semicolon");
            }
            let left_op;
            let op = self.parse_prefix_expressions();
            match op {
                Ok(s) => left_op = s,
                Err(e) => return Err(e),
            }
            result_op = self.infix_expression_parser(0, left_op)?;
            if &self.token_vector[self.current_token] == &tokens::TokenTypes::Semicolon {
                break;
            }
        }
        Ok(Statement::VarStatement {
            token: token,
            name: identifier,
            value: Box::from(result_op),
        })
    }

    fn parse_constant<'a>(&mut self) -> Result<Statement, &'a str> {
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
                return Err("error, token overflow");
            }
            if &tokens::TokenTypes::EndOfLine == &self.token_vector[self.next_token] {
                return Err("error, reached end of line without semicolon");
            }
        }
        Ok(Statement::ConstStatement {
            token: token,
            name: identifier,
            value: "string".to_string(),
        })
    }

    fn infix_expression_parser<'a>(
        &mut self,
        precedence: usize,
        left_op: Expression,
    ) -> Result<Expression, &'a str> {
        self.advance_tokens();
        let op;
        match self.token_vector[self.current_token] {
            tokens::TokenTypes::Operator(s) => op = tokens::TokenTypes::Operator(s),
            _ => return Ok(left_op),
        }

        self.advance_tokens();
        let next_precedence: usize = Parser::get_precedence(&op);
        let mut right_op;
        let prefix_op = self.parse_prefix_expressions();
        match prefix_op {
            Ok(s) => right_op = s,
            Err(e) => return Err(e),
        }

        if next_precedence > precedence {
            let result_op = self.infix_expression_parser(next_precedence, right_op);
            match result_op {
                Ok(v) => right_op = v,
                Err(e) => return Err(e),
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
        let mut precedence = 0;
        let mut left_op;
        let op = self.parse_prefix_expressions();
        match op {
            Ok(s) => left_op = s,
            Err(e) => return Err(e),
        }
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
