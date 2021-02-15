use crate::tokens;

pub fn parse_line(line: &Vec<tokens::TokenTypes>) {
    match &line[0] {
        tokens::TokenTypes::Keywords(tokens::Keywords::If) => {
            println!("if parsed");
        }

        tokens::TokenTypes::Keywords(tokens::Keywords::Var) => {
            parse_variable();
        }
        
        tokens::TokenTypes::Keywords(tokens::Keywords::Const) => {
            parse_variable();
        }

        tokens::TokenTypes::Identifier(s) => {
            println!("identifier: {}", s);
        }

        tokens::TokenTypes::Comment => {
            println!("This is a comment: Line ignored");
        }
        _ => {
            parse_expression(&line);
        }
    }
}

pub fn parse_variable() {
    println!("Parsing a Variable!!!");
}

pub fn parse_expression(line: &Vec<tokens::TokenTypes>) {
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
    operate(&num_vector, &operation);
}


fn operate(numbers_vector: &Vec<&i32>, operators: &Vec<&char>) {
    let mut operations: usize = 0;
    let mut final_value = 0;
    let mut n = 0;
    if operators.len() == 0 {
        final_value = 0;
    }else if operators.len() == 1 {
        let number1 = numbers_vector[operations];
        let number2 = &numbers_vector[&operations + 1];
        match operators[operations] {
            '+' => {
                n = add(number1, number2);
            },
            '-' => {
                n = minus(number1, number2);
            },
            _ => println!("Error"),
        }
        final_value = n;
    }else{
        while operations <= operators.len() - 1 {
            if operations == 0 {
                final_value += numbers_vector[operations];
            }
            let number2 = &numbers_vector[&operations + 1];
            match operators[operations] {
                '+' => {
                    n = add(&final_value, number2);
                },
                '-' => {
                    n = minus(&final_value, number2);
                },
                _ => break,
            }
            final_value = n;
            operations += 1;
        }
    }
    println!("Result: {}", final_value);
}

fn add(num1: &i32, num2: &i32) -> i32{
    num1 + num2
}

fn minus(num1: &i32, num2: &i32) -> i32{
    num1 - num2
}