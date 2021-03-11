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
        parser::Expression::IfExpr {condition, then, other} => {
            return Ok(evaluate_if_condition(*condition, *then, other)?);
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

fn eval_infix_expression(operator: tokens::TokenTypes, left: Objects, right: Objects) -> Objects {
    match (left, right) {
        (Objects::Integer(s), Objects::Integer(r)) => {
            match operator {
                tokens::TokenTypes::Operator('+') => return Objects::Integer(s + r),
                tokens::TokenTypes::Operator('-') => return Objects::Integer(s - r),
                tokens::TokenTypes::Operator('*') => return Objects::Integer(s * r),
                tokens::TokenTypes::Operator('/') => return Objects::Integer(s / r),
                tokens::TokenTypes::Compare(tokens::Comparison::Equal) => return Objects::Boolean(s == r),
                tokens::TokenTypes::Compare(tokens::Comparison::NotEqual) => return Objects::Boolean(s != r),
                tokens::TokenTypes::Compare(tokens::Comparison::Less) => return Objects::Boolean(s < r),
                tokens::TokenTypes::Compare(tokens::Comparison::LessE) => return Objects::Boolean(s <= r),
                tokens::TokenTypes::Compare(tokens::Comparison::Greater) => return Objects::Boolean(s > r),
                tokens::TokenTypes::Compare(tokens::Comparison::GreaterE) => return Objects::Boolean(s >= r),
                _ => return Objects::Boolean(false),
            }
        },
        _ => return Objects::Boolean(false),
    }
}

fn evaluate_if_condition<'a>(condition: parser::Expression, then: parser::Statement, other: Option<Box<parser::Statement>>) -> Result<Objects, &'a str> {
    let obj_condition = eval_expression(condition.clone())?;
    match obj_condition {
        Objects::Boolean(true) => return Ok(return_if_condition(then)),
        Objects::Boolean(false) => {
            match other {
                Some(s) => return Ok(return_if_condition(*s)),
                _ => return Ok(Objects::Boolean(false)),
            }
        }
        _ => return Err("evaluating if expression"),
    }
}

fn return_if_condition(ifexpression: parser::Statement) -> Objects {
    let st = eval_statement(ifexpression);
    match st {
        Ok(s) => return s,
        _ => return Objects::Boolean(false),
    }
}