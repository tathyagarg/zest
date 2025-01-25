#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    RBrace,
    LBrace,
    RParen,
    LParen,
    Dot,
    Comma,
    Equal,
    Number(String),
    EoF,
    Unknown,
    Ignore,
}

pub struct Tokeniser {
    pub text: String,
    current: usize,
    skip: usize,
}

impl Tokeniser {
    pub fn new(text: String) -> Self {
        Self {
            text,
            current: 0,
            skip: 0,
        }
    }

    pub fn tokenise(&mut self) -> Token {
        if self.current >= self.text.len() {
            return Token::EoF;
        }

        let curr = self.text.chars().nth(self.current).unwrap();

        let (token, adv) = match curr {
            '{' => (Token::LBrace, 1),
            '}' => (Token::RBrace, 1),
            '(' => (Token::LParen, 1),
            ')' => (Token::RParen, 1),
            '.' => (Token::Dot, 1),
            ',' => (Token::Comma, 1),
            '=' => (Token::Equal, 1),
            _ => (self.make_token(), self.skip),
        };

        self.current += adv;
        self.skip = 0;

        self.skip_whitespace();
        self.current += self.skip;
        self.skip = 0;

        token
    }

    pub fn make_token(&mut self) -> Token {
        let curr = self.text.chars().nth(self.current).unwrap();
        if curr.is_ascii_digit() || curr == '-' {
            self.number()
        } else if curr.is_ascii_alphabetic() {
            self.identifier()
        } else {
            Token::Unknown
        }
    }

    pub fn skip_whitespace(&mut self) -> Token {
        let mut curr = self.text.chars().nth(self.current).unwrap();
        while curr.is_whitespace() || curr == '\n' {
            self.skip += 1;
            if (self.current + self.skip) >= self.text.len() {
                break;
            }

            curr = self.text.chars().nth(self.current + self.skip).unwrap();
        }

        Token::Ignore
    }

    pub fn number(&mut self) -> Token {
        let mut number = String::new();
        let mut curr = self.text.chars().nth(self.current).unwrap();
        if curr == '-' {
            number.push(curr);
            self.skip += 1;
            curr = self.text.chars().nth(self.current + self.skip).unwrap();
        }
        let mut period_seen = false;

        while curr.is_ascii_digit() || (curr == '.' && !period_seen) {
            if curr == '.' {
                period_seen = true;
            }
            number.push(curr);
            self.skip += 1;
            curr = self.text.chars().nth(self.current + self.skip).unwrap();
        }

        Token::Number(number)
    }

    pub fn identifier(&mut self) -> Token {
        let mut identifier = String::new();
        let mut curr = self.text.chars().nth(self.current).unwrap();
        while curr.is_ascii_alphabetic() {
            identifier.push(curr);
            self.skip += 1;
            curr = self.text.chars().nth(self.current + self.skip).unwrap();
        }

        Token::Identifier(identifier)
    }
}
