use std::fs::read_to_string;

use crate::lexer::tokenise;

mod token;
mod lexer;
mod utils;
mod parser;

fn main() {
    let input = match read_to_string("test.json") {
        Ok(x) => x,
        Err(e) => panic!("Could not open file: {}", e)
    };

    let tokens = tokenise(input.replace("\r\n", "\n").as_str());

    println!("Produced {} tokens!", tokens.len());
}