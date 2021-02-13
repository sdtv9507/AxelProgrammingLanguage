use std::io;
use std::io::Write;

fn main() {
    println!("Axel version 0.1.0");
    println!("Write exit to stop the program");

    loop {
        print!("axel>>");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("{}", input);
    }
}

fn add(args: &[&str]) {
    for arg in args {
        println!("{}", arg);
    }
}