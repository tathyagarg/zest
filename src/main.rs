mod constructor;
mod tokeniser;

use std::collections::VecDeque;

pub fn main() {
    let file = std::env::args().nth(1).expect("No file provided");
    let content = std::fs::read_to_string(&file).expect("Could not read file");

    let mut tokeniser = tokeniser::Tokeniser::new(content);
    let mut token = tokeniser.tokenise();

    let mut tokens = VecDeque::new();

    loop {
        match token {
            tokeniser::Token::EoF | tokeniser::Token::Unknown => break,
            _ => tokens.push_back(token.clone()),
        }
        token = tokeniser.tokenise();
    }
    let mut constructor = constructor::Constructor::new(tokens);
    constructor.construct();
}
