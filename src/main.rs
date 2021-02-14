use std::io;
use std::io::Write;
use std::env;
mod lexer;
mod tokens;
mod parser;

fn main() {
    println!("Axel version 0.1.0");
    println!("OS: {}", env::consts::OS);
    println!("Write exit to stop the program");
    let mut input = String::new();

    loop {
        input.clear();
        print!("axel>>>>");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let compare_input: String = input.trim_end_matches("\r\n").to_string();
        if compare_input == "exit" {
            println!("Goodbye!");
            break;
        }else{
            let token = lexer::get_keywords(&input);
            parser::parse_line(&token);
        }
    }
}
