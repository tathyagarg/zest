#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Number,
    String,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Semicolon,
    EoI, // End of Input
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
}

pub struct Lexer {
    contents: Vec<char>,
    position: usize,
    line: usize,
}

impl Lexer {
    pub fn new(contents: &str) -> Lexer {
        Lexer {
            contents: contents.chars().collect(),
            position: 0,
            line: 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.position >= self.contents.len() {
            return Token {
                token_type: TokenType::EoI,
                value: String::new(),
                line: self.line,
            };
        }

        let mut c = self.contents[self.position];
        while c.is_ascii_whitespace() && self.position < self.contents.len() {
            if c == '\n' {
                self.line += 1;
            }

            c = self.contents[self.position];
            self.position += 1;
        }
        if self.position == self.contents.len() {
            return Token {
                token_type: TokenType::EoI,
                value: String::new(),
                line: self.line,
            };
        }

        let token = match c {
            '+' => self.empty_token(TokenType::Plus),
            '-' => self.empty_token(TokenType::Minus),
            '*' => self.empty_token(TokenType::Star),
            '/' => self.empty_token(TokenType::Slash),
            ';' => self.empty_token(TokenType::Semicolon),
            '(' => self.empty_token(TokenType::LParen),
            ')' => self.empty_token(TokenType::RParen),
            _ => {
                let num_check = self.check_number();
                if num_check.token_type == TokenType::Number {
                    return num_check;
                } else {
                    let str_check = self.check_string();
                    if str_check.token_type == TokenType::String {
                        return str_check;
                    } else {
                        self.empty_token(TokenType::Unknown)
                    }
                }
            } // TODO: Implement this
        };
        self.position += 1;
        token
    }

    fn empty_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            value: String::new(),
            line: self.line,
        }
    }

    fn check_number(&mut self) -> Token {
        if self.position >= self.contents.len() {
            self.empty_token(TokenType::EoI);
        }

        let mut c = self.contents[self.position];
        if !c.is_ascii_digit() {
            return Token {
                token_type: TokenType::Unknown,
                value: c.to_string(),
                line: self.line,
            };
        }

        let mut result = String::new();
        while c.is_ascii_digit() {
            result.push(c);
            self.position += 1;
            c = self.contents[self.position];
        }

        Token {
            token_type: TokenType::Number,
            value: result,
            line: self.line,
        }
    }

    fn check_string(&mut self) -> Token {
        if self.position >= self.contents.len() {
            self.empty_token(TokenType::EoI);
        }

        if self.contents[self.position] != '"' {
            return Token {
                token_type: TokenType::Unknown,
                value: self.contents[self.position].to_string(),
                line: self.line,
            };
        }
        self.position += 1;
        let mut result = String::new();

        while self.contents[self.position] != '"' && self.contents[self.position - 1] != '\\' {
            result.push(self.contents[self.position]);
            self.position += 1;
        }
        self.position += 1;

        Token {
            token_type: TokenType::String,
            value: result,
            line: self.line,
        }
    }
}
