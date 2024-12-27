use crate::lexer::token::{Token, TokenType};

#[derive(Debug)]
pub enum AST {
    Constant {
        token: Token,
    },
    Binary {
        left: Box<AST>,
        operator: Token,
        right: Box<AST>,
    },
}
