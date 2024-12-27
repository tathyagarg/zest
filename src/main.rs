use std::env;

mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_name>", args[0]);
        return;
    }

    let file_name = &args[1];
    let contents = std::fs::read_to_string(file_name).expect("Failed to read file");

    let mut parser = parser::Parser::new(lexer::Lexer::new(contents).lex());
    println!("{:?}", parser.parse());
    println!("{:?}", parser.parse());
}
