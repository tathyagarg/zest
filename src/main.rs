mod constructor;
mod tokeniser;
mod transpiler;

use std::{collections::VecDeque, io::Write};

const USAGE: &str = "\nUsage: zest <input file> [output file]\n";

pub fn main() {
    let in_file = std::env::args().nth(1).expect(USAGE);
    let content = std::fs::read_to_string(&in_file).expect("Could not read file");

    let out_file = std::env::args().nth(2).unwrap_or("out.zig".to_string());

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
    //constructor.print();

    let transpiler = transpiler::Transpiler::new(constructor.engine);

    let mut f = std::fs::File::create(out_file).expect("Could not create file");
    f.write_all(transpiler.transpile().as_bytes())
        .expect("Could not write to file");
}
