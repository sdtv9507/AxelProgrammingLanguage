use std::env;
use std::fs;
use std::io::Write;
use std::{io, path::PathBuf};
mod lexer;
mod parser;
mod tokens;

use crate::parser::Parser;

fn main() {
    println!("Axel version 0.1.0");
    println!("OS: {}", env::consts::OS);
    println!("Write exit to stop the program");
    println!("Write read to parse a file");
    let args: Vec<String> = env::args().collect();
    let mut input = String::new();

    if args.len() > 1 {
        let contents = fs::read_to_string(&args[1]).expect("Couldn't read the file");

        let token = lexer::get_keywords(&contents);
        let mut parser = Parser::new(token);
        let result = parser.check_statement();
        match result {
            Ok(_s) => println!("Program success"),
            Err(e) => println!("Exit program with error: {}", e),
        }
    }

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
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let compare_input: String = input.trim_end_matches("\r\n").to_string();
        if compare_input == "exit" {
            println!("Goodbye!");
            break;
        } else if compare_input == "read" {
            let mut file_content = String::new();
            println!("Write the file's path: ");
            io::stdin()
                .read_line(&mut file_content)
                .expect("Failed to read file");
            let contents = fs::read_to_string(&args[1]).expect("Couldn't read the file");

            let token = lexer::get_keywords(&contents);
            let mut parser = Parser::new(token);
            let result = parser.check_statement();
            match result {
                Ok(_s) => println!("Program success"),
                Err(e) => println!("Exit program with error: {}", e),
            }
        } else {
            let token = lexer::get_keywords(&input);
            let mut parser = Parser::new(token);
            let result = parser.check_statement();
            match result {
                Ok(_s) => println!("Program success"),
                Err(e) => println!("Exit program with error: {}", e),
            }
        }
    }
}

fn get_path() -> std::io::Result<PathBuf> {
    let current_path = env::current_dir()?;
    Ok(current_path)
}
