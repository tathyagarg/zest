mod token;

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
                        value: "+".to_string(),
                    });
                    self.advance();
                }
                '-' => {
                    tokens.push(Token {
                        token_type: TokenType::Minus,
                        value: "-".to_string(),
                    });
                    self.advance();
                }
                '*' => {
                    tokens.push(Token {
                        token_type: TokenType::Star,
                        value: "*".to_string(),
                    });
                    self.advance();
                }
                '/' => {
                    tokens.push(Token {
                        token_type: TokenType::Slash,
                        value: "/".to_string(),
                    });
                    self.advance();
                }
                '(' => {
                    tokens.push(Token {
                        token_type: TokenType::LParen,
                        value: "(".to_string(),
                    });
                    self.advance();
                }
                ')' => {
                    tokens.push(Token {
                        token_type: TokenType::RParen,
                        value: ")".to_string(),
                    });
                    self.advance();
                }
                ';' => {
                    tokens.push(Token {
                        token_type: TokenType::Semicolon,
                        value: ";".to_string(),
                    });
                    self.advance();
                }
                '0'..='9' | '.' => {
                    tokens.push(self.make_number());
                }
                _ => {
                    panic!("Invalid character: {}", self.current);
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

        if is_float {
            return Token {
                token_type: TokenType::Float,
                value,
            };
        }
        Token {
            token_type: TokenType::Integer,
            value,
        }
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
                    token_type: TokenType::Integer,
                    value: "1".to_string(),
                },
                Token {
                    token_type: TokenType::Plus,
                    value: "+".to_string(),
                },
                Token {
                    token_type: TokenType::Integer,
                    value: "2".to_string(),
                },
            ]
        );
    }
}
