mod ast;

use crate::lexer::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ast::AST {
        let res = self.expression();
        self.match_token(TokenType::Semicolon);
        res
    }

    fn expression(&mut self) -> ast::AST {
        self.addition()
    }

    fn addition(&mut self) -> ast::AST {
        let mut node = self.multiplication();

        while self.match_token(TokenType::Plus) || self.match_token(TokenType::Minus) {
            let operator = self.previous();
            let right = self.multiplication();
            node = ast::AST::Binary {
                left: Box::new(node),
                operator,
                right: Box::new(right),
            };
        }

        node
    }

    fn multiplication(&mut self) -> ast::AST {
        let mut node = self.unary();

        while self.match_token(TokenType::Star) || self.match_token(TokenType::Slash) {
            let operator = self.previous();
            let right = self.unary();
            node = ast::AST::Binary {
                left: Box::new(node),
                operator,
                right: Box::new(right),
            };
        }

        node
    }

    fn unary(&mut self) -> ast::AST {
        if self.match_token(TokenType::Minus) {
            let operator = self.previous();
            let right = self.unary();
            return ast::AST::Binary {
                left: Box::new(ast::AST::Constant {
                    token: Token {
                        token_type: TokenType::Literal,
                        lexeme: "0".to_string(),
                        literal: Some("0".to_string()),
                    },
                }),
                operator,
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> ast::AST {
        if self.match_token(TokenType::Literal) {
            return ast::AST::Constant {
                token: self.previous(),
            };
        }

        panic!("Unexpected token: {:?}", self.peek());
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
