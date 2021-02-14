use crate::tokens;

pub fn get_keywords(text: &String) -> Vec<tokens::TokenTypes> {
    let mut words = Vec::new();
    let mut min_index = 0;
    let mut index = 0;
    let mut element = 0;
    let mut token_vector = Vec::new();
    let mut vector_index = 0;
    for chr in text.chars() {
        index += 1;
        if chr.is_whitespace() || chr == '\n' {
            if chr != '\n' {
                words.insert(element, &text[min_index..index-1]);
                min_index = index;
                element += 1;
            }
        }
    }

    for element in words {
        match element {
            "if" => {
                token_vector.insert(vector_index, tokens::TokenTypes::Keywords(tokens::Keywords::If));
            }

            "let" => {
                token_vector.insert(vector_index, tokens::TokenTypes::Keywords(tokens::Keywords::Let));
            }

            "+" => {
                token_vector.insert(vector_index, tokens::TokenTypes::Operator('+'));
            }
            "-" => {
                token_vector.insert(vector_index, tokens::TokenTypes::Operator('-'));
            }
            "=" => {
                token_vector.insert(vector_index, tokens::TokenTypes::Operator('='));
            }
            "*" => {
                token_vector.insert(vector_index, tokens::TokenTypes::Operator('*'));
            },
            "/" => {
                token_vector.insert(vector_index, tokens::TokenTypes::Operator('/'));
            },
            "//" => {
                token_vector.insert(vector_index, tokens::TokenTypes::Comment);
            },

            "\n" => {
                token_vector.insert(vector_index, tokens::TokenTypes::EndOfLine);
            },

            _ => {
                let text_data = element.chars().nth(0).unwrap();
                if text_data.is_digit(10) {
                    let int_converted: i32 = element.parse().unwrap();
                    token_vector.insert(vector_index, tokens::TokenTypes::Numbers(int_converted));
                }else if text_data.is_ascii_alphabetic() || text_data == '_' {
                    token_vector.insert(vector_index, tokens::TokenTypes::Identifier(element.to_string()));
                }else {
                    token_vector.insert(vector_index, tokens::TokenTypes::Illegal);
                }
            }
        }
        vector_index += 1;
    }
    token_vector
}
