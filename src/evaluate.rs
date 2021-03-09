use crate::object;
use crate::parser;

pub fn eval_expression<'a>(expression: parser::Expression) -> Result<object::Objects, &'a str> {
    match expression {
        parser::Expression::NumberLit { number } => return Ok(object::Objects::Integer(number)),
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
        _ => return Err("expected an statement for eval"),
    }
}