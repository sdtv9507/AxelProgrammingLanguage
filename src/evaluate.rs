use object::{Environment, Function, Objects};

use crate::parser;
use crate::{object, tokens};

pub struct Evaluator {
    environment: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            environment: Environment::new(),
        }
    }

    pub fn eval_statement<'a>(
        &mut self,
        statement: parser::Statement,
    ) -> Result<object::Objects, &'a str> {
        match statement {
            parser::Statement::VarStatement { name, value } => {
                let e = *value.clone();
                let eval = self.eval_expression(e)?;
                self.environment.add(name, eval.clone());
                return Ok(eval);
            }
            parser::Statement::ConstStatement { name, value } => {
                let e = *value.clone();
                let eval = self.eval_expression(e)?;
                self.environment.add(name, eval.clone());
                return Ok(eval);
            }
            parser::Statement::ReturnStatement { value } => {
                let e = *value.clone();
                return self.eval_expression(e);
            }
            parser::Statement::ExpressionStatement { value } => {
                let e = *value.clone();
                return self.eval_expression(e);
            }
        }
    }

    pub fn eval_expression<'a>(
        &mut self,
        expression: parser::Expression,
    ) -> Result<object::Objects, &'a str> {
        match expression {
            parser::Expression::NumberLit { number } => {
                return Ok(object::Objects::Integer(number));
            }
            parser::Expression::StringLit { string } => {
                return Ok(object::Objects::String(string));
            }
            parser::Expression::BoolExp { value } => return Ok(object::Objects::Boolean(value)),
            parser::Expression::IdentifierLit { name } => {
                let val = self.environment.search(name);
                match val {
                    Some(s) => return Ok(s.clone()),
                    None => return Err("identifier not found"),
                }
            }

            parser::Expression::Prefix { operator, right } => {
                let evaluate_right = self.eval_expression(*right);
                match evaluate_right {
                    Ok(s) => {
                        let right_eval = self.eval_prefix_expression(operator, s);
                        match right_eval {
                            Ok(s) => return Ok(s),
                            Err(e) => return Err(e),
                        }
                    }
                    Err(e) => return Err(e),
                }
            }

            parser::Expression::InfixOp {
                left,
                right,
                operator,
            } => {
                let evaluate_left = self.eval_expression(*left)?;
                let evaluate_right = self.eval_expression(*right)?;
                let infix = self.eval_infix_expression(operator, evaluate_left, evaluate_right);
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
                return Ok(self.evaluate_if_condition(*condition, *then, other)?);
            }
            parser::Expression::FunctionExpr {
                identifier,
                parameters,
                body,
            } => {
                let function = Function::new(parameters, body);
                self.environment
                    .add(identifier, Objects::Function(function.clone()));
                return Ok(Objects::Function(function));
            }

            parser::Expression::CallExpr {
                identifier,
                parameters,
            } => {
                let val = self.environment.search(identifier.clone());
                let call_identifier;
                match val {
                    Some(s) => call_identifier = s.clone(),
                    None => {
                        let builtin = object::BuiltinFunction::new(identifier);
                        call_identifier = Objects::BuiltIn(builtin);
                    }
                };
                let param_values = self.eval_call_params(parameters)?;

                let mut func_names: Vec<String> = Vec::new();
                //let mut inner_environment;
                let mut evaluated: Option<Result<Objects, &'a str>> = None;
                match call_identifier {
                    Objects::Function(func) => {
                        for i in func.parameters {
                            func_names.push(i);
                        }
                        //inner_environment = func.environment;
                        for i in 0..func_names.len() {
                            self.environment
                                .add(func_names[i].clone(), param_values[i].clone());
                        }

                        for statements in *func.body {
                            evaluated = Some(self.eval_statement(statements));
                        }

                        for i in 0..func_names.len() {
                            self.environment.remove(func_names[i].clone());
                        }
                    }
                    Objects::BuiltIn(mut func) => {
                        evaluated = Some(func.call(param_values));
                    }
                    _ => return Err("object isn't a function"),
                }

                match evaluated {
                    Some(s) => match s {
                        Ok(t) => return Ok(t),
                        Err(e) => return Err(e),
                    },
                    None => return Err("evaluating function"),
                }
            }
            parser::Expression::ArrayLit { elements } => {
                let mut elements_object: Vec<Objects> = Vec::new();
                for element in elements {
                    elements_object.push(self.eval_expression(element)?);
                }
                return Ok(Objects::Array(elements_object));
            }
            parser::Expression::IndexExpression { left, right } => {
                let left_obj = self.eval_expression(*left)?;
                let right_obj = self.eval_expression(*right)?;
                return self.eval_index_expression(left_obj, right_obj);
            }
        }
    }

    fn eval_index_expression<'a>(&mut self, left: Objects, right: Objects) -> Result<Objects, &'a str> {
        match (left, right) {
            (Objects::Array(s), Objects::Integer(t)) => {
                return Ok(s[t as usize].clone());
            }
            _ => return Err("Array call expressions unsupported"),
        }
    }

    fn eval_call_params<'a>(
        &mut self,
        parameters: Vec<parser::Expression>,
    ) -> Result<Vec<Objects>, &'a str> {
        let mut result = Vec::new();
        for arg in parameters {
            let arg_eval = self.eval_expression(arg);
            match arg_eval {
                Ok(s) => result.push(s),
                Err(e) => return Err(e),
            }
        }
        return Ok(result);
    }

    fn eval_bang_operator<'a>(&mut self, obj: Objects) -> Result<Objects, &'a str> {
        match obj {
            Objects::Boolean(true) => return Ok(Objects::Boolean(true)),
            Objects::Boolean(false) => return Ok(Objects::Boolean(false)),
            _ => return Err("expected boolean object"),
        }
    }

    fn eval_minus_operator<'a>(&mut self, obj: Objects) -> Result<Objects, &'a str> {
        match obj {
            Objects::Integer(s) => return Ok(Objects::Integer(-s)),
            _ => return Err("expected an integer to the right of - sign"),
        }
    }

    fn eval_prefix_expression<'a>(
        &mut self,
        operator: tokens::TokenTypes,
        obj: Objects,
    ) -> Result<Objects, &'a str> {
        match operator {
            tokens::TokenTypes::Bang => {
                let bang_op = self.eval_bang_operator(obj);
                match bang_op {
                    Ok(s) => return Ok(s),
                    Err(e) => return Err(e),
                }
            }
            tokens::TokenTypes::Operator('-') => {
                let min = self.eval_minus_operator(obj);
                match min {
                    Ok(s) => return Ok(s),
                    Err(e) => return Err(e),
                }
            }
            _ => return Err("operator prefix mismatch"),
        }
    }

    fn eval_infix_expression<'a>(
        &mut self,
        operator: tokens::TokenTypes,
        left: Objects,
        right: Objects,
    ) -> Result<Objects, &'a str> {
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
            (Objects::String(s), Objects::String(t)) => match operator {
                tokens::TokenTypes::Operator('+') => return Ok(Objects::String(s + &t)),
                _ => return Err("unknown operator"),
            },
            _ => return Err("operand type mismatch"),
        }
    }

    fn evaluate_if_condition<'a>(
        &mut self,
        condition: parser::Expression,
        then: Vec<parser::Statement>,
        other: Option<Box<Vec<parser::Statement>>>,
    ) -> Result<Objects, &'a str> {
        let obj_condition = self.eval_expression(condition.clone())?;
        match obj_condition {
            Objects::Boolean(true) => {
                let mut evaluated_statement = None;
                for statement in then {
                    evaluated_statement = Some(self.return_if_condition(statement.clone()));
                    match statement {
                        parser::Statement::ReturnStatement { value: _ } => {
                            match evaluated_statement {
                                Some(s) => return Ok(s),
                                None => return Err("error evaluating if expressions"),
                            }
                        }
                        _ => continue,
                    }
                }
                match evaluated_statement {
                    Some(s) => return Ok(s),
                    None => return Err("error evaluating if expressions"),
                }
            }
            Objects::Boolean(false) => match other {
                Some(other_statement) => {
                    let mut evaluated_statement = None;
                    for statement in *other_statement {
                        evaluated_statement = Some(self.return_if_condition(statement.clone()));
                        match statement {
                            parser::Statement::ReturnStatement { value: _ } => {
                                match evaluated_statement {
                                    Some(t) => return Ok(t),
                                    None => return Err("error evaluating if expressions"),
                                }
                            }
                            _ => continue,
                        }
                    }
                    match evaluated_statement {
                        Some(t) => return Ok(t),
                        None => return Err("error evaluating if expressions"),
                    }
                }
                _ => return Ok(Objects::Boolean(false)),
            },
            _ => return Err("evaluating if expression"),
        }
    }

    fn return_if_condition(&mut self, ifexpression: parser::Statement) -> Objects {
        let st = self.eval_statement(ifexpression);
        match st {
            Ok(s) => return s,
            _ => return Objects::Boolean(false),
        }
    }
}
