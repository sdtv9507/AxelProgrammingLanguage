use crate::tokens;

pub fn get_keywords(read_text: &String) -> Vec<tokens::TokenTypes> {
    let text_vec: Vec<char> = read_text.chars().collect();
    /*for i in text_vec {
        println!("{}", i);
    }*/
    let final_vector = read_token(&text_vec);
    final_vector
}

fn read_token(text_vec: &Vec<char>) -> Vec<tokens::TokenTypes> {
    let mut index = 0;
    let mut token_vector = Vec::new();
    loop {
        let mut chr = text_vec[index];
        while is_ignored(chr) {
            index += 1;
            chr = text_vec[index];
        }
        match chr {
            '+' => {
                println!("+");
                token_vector.push(tokens::TokenTypes::Operator('+'));
            }
            '-' => {
                println!("-");
                token_vector.push(tokens::TokenTypes::Operator('-'));
            }
            '=' => {
                println!("=");
                token_vector.push(tokens::TokenTypes::Operator('='));
            }
            '*' => {
                println!("*");
                token_vector.push(tokens::TokenTypes::Operator('*'));
            }
            '/' => {
                println!("/");
                token_vector.push(tokens::TokenTypes::Operator('/'));
            }
            '(' => {
                println!("(");
                token_vector.push(tokens::TokenTypes::Operator('('));
            }
            ')' => {
                println!(")");
                token_vector.push(tokens::TokenTypes::Operator(')'));
            }
            '#' => {
                println!("comment");
                token_vector.push(tokens::TokenTypes::Comment);
            }

            '\n' => {
                break;
            }

            _ => {
                if is_valid_number(chr) == true {
                    let mut final_index = index;
                    while is_valid_number(chr) == true {
                        final_index += 1;
                        chr = text_vec[final_index];
                    }
                    let identifier: &str = &text_vec[index..final_index].iter().collect::<String>();
                    let int_identifier = identifier.parse().unwrap();
                    token_vector.push(tokens::TokenTypes::NumbersInt(int_identifier));
                    index = final_index - 1;
                }else if is_valid_identifier(chr) == true {
                    let mut final_index = index;
                    while is_valid_identifier(chr) == true {
                        final_index += 1;
                        chr = text_vec[final_index];
                    }
                    let identifier: &str = &text_vec[index..final_index].iter().collect::<String>();
                    match identifier {
                        "if" => {
                            token_vector.push(tokens::TokenTypes::Keywords(tokens::Keywords::If));
                        }
                        
                        "var" => {
                            token_vector.push(tokens::TokenTypes::Keywords(tokens::Keywords::Var));
                        }
                        
                        "const" => {
                            token_vector.push(tokens::TokenTypes::Keywords(tokens::Keywords::Const));
                        }
                        _ => {
                            token_vector.push(tokens::TokenTypes::Identifier(identifier.to_string()));
                        }
                    }
                    index = final_index - 1;
                } else {
                    println!("{}", chr);
                    token_vector.push(tokens::TokenTypes::Illegal);
                }
            }
        }
        index += 1;
    }
    token_vector
}

fn is_ignored(chr: char) -> bool {
    chr == ' ' || chr == '\r'
}

fn is_valid_number(chr: char) -> bool {
    '0' <= chr && chr <= '9'
}

fn is_valid_identifier(chr: char) -> bool {
    'a' <= chr && chr <= 'z' || 'A' <= chr && chr <= 'Z' || chr == '_'
}
