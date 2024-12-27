#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Literal,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Semicolon,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Token{{ tt: {:?}, lexeme: {}, literal: {} }}",
            self.token_type,
            self.lexeme,
            self.literal.as_ref().unwrap_or(&"".to_string())
        )
    }
}
