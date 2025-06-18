mod token;
mod lexer;
mod utils;
mod parser;

use std::fs::read_to_string;
use std::time::Instant;

use crate::lexer::tokenise;
use crate::parser::parse;

fn main() {
    let input = match read_to_string("massive-test.json") {
        Ok(x) => x,
        Err(e) => panic!("Could not read file: {}", e)
    };

    let start = Instant::now();

    let tokens = tokenise(input.as_str());

    let after_tokens = start.elapsed();

    println!("Time taken to tokenise: {:?}", after_tokens);

    let _result = parse(&tokens);

    let duration = start.elapsed() - after_tokens;

    println!("Time taken to parse tokens: {:?}", duration);
}