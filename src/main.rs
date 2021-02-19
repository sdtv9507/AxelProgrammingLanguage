use std::{io, path::PathBuf};
use std::io::Write;
use std::env;
mod lexer;
mod tokens;
mod parser;

use crate::parser::Parser;

fn main() {
    println!("Axel version 0.1.0");
    println!("OS: {}", env::consts::OS);
    println!("Write exit to stop the program");
    let mut input = String::new();

    loop {
        input.clear();
        let path = get_path();
        let path = match path {
            Ok(path) => path,
            Err(error) => panic!("Problem getting path: {:?}", error),
        };
        println!("Path: {}", path.display());
        print!("axel>>");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let compare_input: String = input.trim_end_matches("\r\n").to_string();
        if compare_input == "exit" {
            println!("Goodbye!");
            break;
        }else{
            let token = lexer::get_keywords(&input);
            let mut parser = Parser::new(token);
            parser.parse_line();
        }
    }
}

fn get_path() -> std::io::Result<PathBuf>{
    let current_path = env::current_dir()?;
    Ok(current_path)
}