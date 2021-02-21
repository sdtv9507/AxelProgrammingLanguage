use crate::tokens;

pub fn get_keywords(read_text: &String) -> Vec<tokens::TokenTypes> {
    let text_vec: Vec<char> = read_text.chars().collect();
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
            if index > text_vec.len() {
				token_vector.push(tokens::TokenTypes::EndOfLine);
                break;
            }
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
            ';' => {
                println!("semicolon");
                token_vector.push(tokens::TokenTypes::Semicolon);
            }
            '"' => {
                let mut final_index = index;
                final_index += 1;
                loop {
                    final_index += 1;
                    chr = text_vec[final_index];
                    if chr == '"' {
                        final_index += 1;
                        break;
                    }
                }
                let string = &text_vec[index..final_index].iter().collect::<String>().to_string();
                let final_string = string.parse().unwrap();
                println!("string: {0}, {1}, {2}", final_string, index, final_index);
                token_vector.push(tokens::TokenTypes::Strings(final_string));
                index = final_index - 1;
            }
			'\x00' => {
                println!("end of line");
				token_vector.push(tokens::TokenTypes::EndOfLine);
				break;
			}
            '\n' => {
				token_vector.push(tokens::TokenTypes::EndOfLine);
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
                    println!("{}", int_identifier);
                    token_vector.push(tokens::TokenTypes::NumbersInt(int_identifier));
                    index = final_index - 1;
                }else if is_valid_identifier(chr) == true {
                    let mut final_index = index;
                    while is_valid_identifier(chr) == true {
                        final_index += 1;
                        chr = text_vec[final_index];
                    }
                    let identifier: &str = &text_vec[index..final_index].iter().collect::<String>();
                    println!("{}", identifier);
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
