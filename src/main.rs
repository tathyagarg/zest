mod tokeniser;

pub fn main() {
    let file = std::env::args().nth(1).expect("No file provided");
    let content = std::fs::read_to_string(&file).expect("Could not read file");

    let mut tokeniser = tokeniser::Tokeniser::new(content);
    let mut token = tokeniser.tokenise();

    let mut tokens = Vec::<tokeniser::Token>::new();

    loop {
        token = tokeniser.tokenise();
        match token {
            tokeniser::Token::EoF | tokeniser::Token::Unknown => break,
            _ => tokens.push(token.clone()),
        }
    }
    println!("{:?}", tokens);
}
