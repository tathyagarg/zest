pub mod token;

use token::{Token, TokenType};

pub struct Lexer {
    contents: String,
    position: usize,
    current: char,
}

impl Lexer {
    pub fn new(contents: String) -> Lexer {
        let mut lexer = Lexer {
            contents,
            position: 0,
            current: '\0',
        };
        lexer.current = lexer.contents.chars().nth(lexer.position).unwrap_or('\0');

        lexer
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current = self.contents.chars().nth(self.position).unwrap_or('\0');
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        while self.current != '\0' {
            match self.current {
                ' ' => self.advance(),
                '+' => {
                    tokens.push(Token {
                        token_type: TokenType::Plus,
                        lexeme: "+".to_string(),
                        literal: None,
                    });
                    self.advance();
                }
                '-' => {
                    tokens.push(Token {
                        token_type: TokenType::Minus,
                        lexeme: "-".to_string(),
                        literal: None,
                    });
                    self.advance();
                }
                '*' => {
                    tokens.push(Token {
                        token_type: TokenType::Star,
                        lexeme: "*".to_string(),
                        literal: None,
                    });
                    self.advance();
                }
                '/' => {
                    tokens.push(Token {
                        token_type: TokenType::Slash,
                        lexeme: "/".to_string(),
                        literal: None,
                    });
                    self.advance();
                }
                '(' => {
                    tokens.push(Token {
                        token_type: TokenType::LParen,
                        lexeme: "(".to_string(),
                        literal: None,
                    });
                    self.advance();
                }
                ')' => {
                    tokens.push(Token {
                        token_type: TokenType::RParen,
                        lexeme: ")".to_string(),
                        literal: None,
                    });
                    self.advance();
                }
                ';' => {
                    tokens.push(Token {
                        token_type: TokenType::Semicolon,
                        lexeme: ";".to_string(),
                        literal: None,
                    });
                    self.advance();
                }
                '0'..='9' | '.' => {
                    tokens.push(self.make_number());
                }
                '\n' => {
                    self.advance();
                }
                _ => {
                    panic!(
                        "Invalid character: {} at {}",
                        self.current as u32, self.position
                    );
                }
            }
        }

        tokens
    }

    fn make_number(&mut self) -> Token {
        let mut value = String::new();
        let mut is_float = false;

        while self.current.is_ascii_digit() || self.current == '.' {
            if self.current == '.' {
                if is_float {
                    // TODO: Error, we already have a float
                }
                is_float = true;
            }

            value.push(self.current);
            self.advance();
        }

        return Token {
            token_type: TokenType::Literal,
            lexeme: value.clone(),
            literal: Some(value),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("1 + 2".to_string());
        assert_eq!(
            lexer.lex(),
            vec![
                Token {
                    token_type: TokenType::Literal,
                    lexeme: "1".to_string(),
                    literal: Some("1".to_string()),
                },
                Token {
                    token_type: TokenType::Plus,
                    lexeme: "+".to_string(),
                    literal: None,
                },
                Token {
                    token_type: TokenType::Literal,
                    lexeme: "2".to_string(),
                    literal: Some("2".to_string()),
                },
            ]
        );
    }
}
