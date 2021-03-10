use object::Objects;

use crate::{object, tokens};
use crate::parser;

pub fn eval_expression<'a>(expression: parser::Expression) -> Result<object::Objects, &'a str> {
    match expression {
        parser::Expression::NumberLit { number } => return Ok(object::Objects::Integer(number)),
        parser::Expression::BoolExp { value } => return Ok(object::Objects::Boolean(value)),
        parser::Expression::Prefix { operator, right} => {
            let evaluate_right = eval_expression(*right);
            match evaluate_right {
                Ok(s) => return Ok(eval_prefix_expression(operator, s)),
                Err(e) => return Err(e),
            }
        }
        parser::Expression::InfixOp {left, right, operator} => {
            let evaluate_left = eval_expression(*left)?;
            let evaluate_right = eval_expression(*right)?;
            return Ok(eval_infix_expression(operator, evaluate_left, evaluate_right));
        }
        _ => return Err("expected an expression to eval"),
    }
}

pub fn eval_statement<'a>(statement: parser::Statement) -> Result<object::Objects, &'a str> {
    match statement {
        parser::Statement::VarStatement { name, value } => {
            let e = *value.clone();
            return eval_expression(e);
        },
        parser::Statement::ReturnStatement { value } => {
            let e = *value.clone();
            return eval_expression(e);
        },
        parser::Statement::ExpressionStatement { value } => {
            let e = *value.clone();
            return eval_expression(e);
        },
        _ => return Err("expected an statement for eval"),
    }
}

fn eval_prefix_expression(operator: tokens::TokenTypes, obj: object::Objects) -> object::Objects {
    match operator {
        tokens::TokenTypes::Bang => return eval_bang_operator(obj),
        tokens::TokenTypes::Operator('-') => {
            let min = eval_minus_operator(obj);
            match min {
                Ok(s) => return s,
                Err(e) => {
                    println!("{}", e);
                    return object::Objects::Boolean(false);
                },
            }
        }
        _ => return object::Objects::Boolean(false),
    }
}

fn eval_bang_operator(obj: object::Objects) -> object::Objects {
    match obj {
        object::Objects::Boolean(true) => return object::Objects::Boolean(true),
        object::Objects::Boolean(false) => return object::Objects::Boolean(false),
        _ => return object::Objects::Boolean(false),
    }
}

fn eval_minus_operator<'a>(obj: object::Objects) -> Result<object::Objects, &'a str> {
    match obj {
        object::Objects::Integer(s) => return Ok(object::Objects::Integer(-s)),
        _ => return Err("expected an integer to the right of - sign"),
    }
}

fn eval_infix_expression(operator: tokens::TokenTypes, left: object::Objects, right: object::Objects) -> object::Objects {
    match (left, right) {
        (Objects::Integer(s), Objects::Integer(r)) => {
            match operator {
                tokens::TokenTypes::Operator('+') => return Objects::Integer(s + r),
                tokens::TokenTypes::Operator('-') => return Objects::Integer(s - r),
                tokens::TokenTypes::Operator('*') => return Objects::Integer(s * r),
                tokens::TokenTypes::Operator('/') => return Objects::Integer(s / r),
                _ => return Objects::Boolean(false),
            }
        },
        _ => return Objects::Boolean(false),
    }
}
