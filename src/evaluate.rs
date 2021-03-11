use object::Objects;

use crate::parser;
use crate::{object, tokens};

pub fn eval_expression<'a>(expression: parser::Expression) -> Result<object::Objects, &'a str> {
    match expression {
        parser::Expression::NumberLit { number } => return Ok(object::Objects::Integer(number)),
        parser::Expression::BoolExp { value } => return Ok(object::Objects::Boolean(value)),
        parser::Expression::Prefix { operator, right } => {
            let evaluate_right = eval_expression(*right);
            match evaluate_right {
                Ok(s) => {
                    let right_eval = eval_prefix_expression(operator, s);
                    match right_eval {
                        Ok(s) => return Ok(s),
                        Err(e) => return Err(e),
                    }
                },
                Err(e) => return Err(e),
            }
        }
        parser::Expression::InfixOp {
            left,
            right,
            operator,
        } => {
            let evaluate_left = eval_expression(*left)?;
            let evaluate_right = eval_expression(*right)?;
            let infix = eval_infix_expression(
                operator,
                evaluate_left,
                evaluate_right,
            );
            match infix {
                Ok(s) => return Ok(s),
                Err(e) => return Err(e),
            }
        }
        parser::Expression::IfExpr {
            condition,
            then,
            other,
        } => {
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
        }
        parser::Statement::ReturnStatement { value } => {
            let e = *value.clone();
            return eval_expression(e);
        }
        parser::Statement::ExpressionStatement { value } => {
            let e = *value.clone();
            return eval_expression(e);
        }
        _ => return Err("expected an statement for eval"),
    }
}

fn eval_prefix_expression<'a>(operator: tokens::TokenTypes, obj: Objects) -> Result<Objects, &'a str> {
    match operator {
        tokens::TokenTypes::Bang => {
            let bang_op = eval_bang_operator(obj);
            match bang_op {
                Ok(s) => return Ok(s),
                Err(e) => return Err(e),
            }
        },
        tokens::TokenTypes::Operator('-') => {
            let min = eval_minus_operator(obj);
            match min {
                Ok(s) => return Ok(s),
                Err(e) => return Err(e),
            }
        }
        _ => return Err("operator prefix mismatch"),
    }
}

fn eval_bang_operator<'a>(obj: Objects) -> Result<Objects, &'a str> {
    match obj {
        Objects::Boolean(true) => return Ok(Objects::Boolean(true)),
        Objects::Boolean(false) => return Ok(Objects::Boolean(false)),
        _ => return Err("expected boolean object"),
    }
}

fn eval_minus_operator<'a>(obj: Objects) -> Result<Objects, &'a str> {
    match obj {
        Objects::Integer(s) => return Ok(Objects::Integer(-s)),
        _ => return Err("expected an integer to the right of - sign"),
    }
}

fn eval_infix_expression<'a>(operator: tokens::TokenTypes, left: Objects, right: Objects) -> Result<Objects, &'a str> {
    match (left, right) {
        (Objects::Integer(s), Objects::Integer(r)) => match operator {
            tokens::TokenTypes::Operator('+') => return Ok(Objects::Integer(s + r)),
            tokens::TokenTypes::Operator('-') => return Ok(Objects::Integer(s - r)),
            tokens::TokenTypes::Operator('*') => return Ok(Objects::Integer(s * r)),
            tokens::TokenTypes::Operator('/') => return Ok(Objects::Integer(s / r)),
            tokens::TokenTypes::Compare(tokens::Comparison::Equal) => {
                return Ok(Objects::Boolean(s == r));
            }
            tokens::TokenTypes::Compare(tokens::Comparison::NotEqual) => {
                return Ok(Objects::Boolean(s != r));
            }
            tokens::TokenTypes::Compare(tokens::Comparison::Less) => {
                return Ok(Objects::Boolean(s < r));
            }
            tokens::TokenTypes::Compare(tokens::Comparison::LessE) => {
                return Ok(Objects::Boolean(s <= r));
            }
            tokens::TokenTypes::Compare(tokens::Comparison::Greater) => {
                return Ok(Objects::Boolean(s > r));
            }
            tokens::TokenTypes::Compare(tokens::Comparison::GreaterE) => {
                return Ok(Objects::Boolean(s >= r));
            }
            _ => return Err("unknown operator"),
        },
        _ => return Err("operand type mismatch"),
    }
}

fn evaluate_if_condition<'a>(
    condition: parser::Expression,
    then: parser::Statement,
    other: Option<Box<parser::Statement>>,
) -> Result<Objects, &'a str> {
    let obj_condition = eval_expression(condition.clone())?;
    match obj_condition {
        Objects::Boolean(true) => return Ok(return_if_condition(then)),
        Objects::Boolean(false) => match other {
            Some(s) => return Ok(return_if_condition(*s)),
            _ => return Ok(Objects::Boolean(false)),
        },
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
