use crate::ast::AST;
use crate::lexer::{Lexer, TokenType};

pub fn parse(contents: &str) {
    let mut lexer = Lexer::new(contents);
    loop {
        let token = lexer.next_token();
        if token.token_type == TokenType::EoI {
            break;
        }
        println!("{:?}", token);
    }
}
