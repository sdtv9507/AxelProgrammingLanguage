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
        if index >= text_vec.len() {
            token_vector.push(tokens::TokenTypes::EndOfLine);
            break;
        }
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
                token_vector.push(tokens::TokenTypes::Operator('+'));
            }
            '-' => {
                token_vector.push(tokens::TokenTypes::Operator('-'));
            }
            '=' => {
                let final_index = index + 1;
                if text_vec[final_index] == '=' {
                    index += 1;
                    token_vector.push(tokens::TokenTypes::Compare(tokens::Comparison::Equal));
                } else {
                    token_vector.push(tokens::TokenTypes::Operator('='));
                }
            }
            '<' => {
                let final_index = index + 1;
                if text_vec[final_index] == '=' {
                    index += 1;
                    token_vector.push(tokens::TokenTypes::Compare(tokens::Comparison::LessE));
                } else {
                    token_vector.push(tokens::TokenTypes::Compare(tokens::Comparison::Less));
                }
            }
            '>' => {
                let final_index = index + 1;
                if text_vec[final_index] == '=' {
                    index += 1;
                    token_vector.push(tokens::TokenTypes::Compare(tokens::Comparison::GreaterE));
                } else {
                    token_vector.push(tokens::TokenTypes::Compare(tokens::Comparison::Greater));
                }
            }
            '!' => {
                let final_index = index + 1;
                if text_vec[final_index] == '=' {
                    index += 1;
                    token_vector.push(tokens::TokenTypes::Compare(tokens::Comparison::NotEqual));
                } else {
                    token_vector.push(tokens::TokenTypes::Bang);
                }
            }
            '*' => {
                token_vector.push(tokens::TokenTypes::Operator('*'));
            }
            '/' => {
                token_vector.push(tokens::TokenTypes::Operator('/'));
            }
            '(' => {
                token_vector.push(tokens::TokenTypes::Operator('('));
            }
            ')' => {
                token_vector.push(tokens::TokenTypes::Operator(')'));
            }
            '{' => {
                token_vector.push(tokens::TokenTypes::Delim('{'));
            }
            '}' => {
                token_vector.push(tokens::TokenTypes::Delim('}'));
            }
            '#' => {
                token_vector.push(tokens::TokenTypes::Comment);
            }
            ',' => {
                token_vector.push(tokens::TokenTypes::Comma);
            }
            ';' => {
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
                let string = &text_vec[index+1..final_index-1]
                    .iter()
                    .collect::<String>()
                    .to_string();
                let final_string = string.parse().unwrap();
                token_vector.push(tokens::TokenTypes::Strings(final_string));
                index = final_index - 1;
            }
            '\x00' => {
                //Ignore this
            }
            '\n' => {
                //Ignore this
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
                } else if is_valid_identifier(chr) == true {
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

                        "else" => {
                            token_vector.push(tokens::TokenTypes::Keywords(tokens::Keywords::Else));
                        }

                        "var" => {
                            token_vector.push(tokens::TokenTypes::Keywords(tokens::Keywords::Var));
                        }

                        "const" => {
                            token_vector
                                .push(tokens::TokenTypes::Keywords(tokens::Keywords::Const));
                        }

                        "return" => {
                            token_vector
                                .push(tokens::TokenTypes::Keywords(tokens::Keywords::Return));
                        }

                        "fn" => {
                            token_vector
                                .push(tokens::TokenTypes::Keywords(tokens::Keywords::Function));
                        }

                        "true" => {
                            token_vector.push(tokens::TokenTypes::Keywords(tokens::Keywords::True));
                        }

                        "false" => {
                            token_vector
                                .push(tokens::TokenTypes::Keywords(tokens::Keywords::False));
                        }
                        _ => {
                            token_vector
                                .push(tokens::TokenTypes::Identifier(identifier.to_string()));
                        }
                    }
                    index = final_index - 1;
                } else {
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
