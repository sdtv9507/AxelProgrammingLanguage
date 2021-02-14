use crate::tokens;

pub fn get_keywords(text: &String) -> tokens::TokenTypes {
    let mut words = Vec::new();
    let mut min_index = 0;
    let mut index = 0;
    let mut element = 0;
    let mut token_type = tokens::TokenTypes::Unknown;
    for chr in text.chars() {
        index += 1;
        if chr.is_whitespace() {
            words.insert(element, &text[min_index..index-1]);
            min_index = index;
            element += 1;
        }
    }

    for element in words {
        println!("{}", element);
        match element {
            "if" => {
                println!(" its if!");
                token_type = tokens::TokenTypes::Keywords(tokens::Keywords::If("if".to_string()));
            }

            "let" => {
                println!("its let!");
                token_type = tokens::TokenTypes::Keywords(tokens::Keywords::Let("let".to_string()))
            }

            _ => {
                for chr in element.chars() {
                    token_type = test_next_token(&chr);
                }
            }
        }
    }

    token_type
}

pub fn test_next_token(chr: &char) -> tokens::TokenTypes {
    let mut token_type = tokens::TokenTypes::Illegal;
    match chr {
        '+' => {
            println!("plus");
            token_type = tokens::TokenTypes::Operator(String::from("+"));
        },
        '-' => token_type = tokens::TokenTypes::Operator(String::from("-")),
        '=' => token_type = tokens::TokenTypes::Operator(String::from("=")),
        '*' => token_type = tokens::TokenTypes::Operator(String::from("*")),
        '/' => token_type = tokens::TokenTypes::Operator(String::from("/")),
        _ => {
            if chr.is_digit(10) {
                token_type = tokens::TokenTypes::Numbers((chr.to_string()).parse().unwrap());
            }else if chr.is_alphanumeric() {
                token_type = tokens::TokenTypes::Strings(chr.to_string());
            }
        },
    }

    token_type
}
