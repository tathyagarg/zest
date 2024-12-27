#[derive(Debug, PartialEq)]
pub enum TokenType {
    Integer,
    Float,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Semicolon,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Token{{ tt: {:?}, value: {} }}",
            self.token_type, self.value
        )
    }
}
