mod token;
mod lexer;
mod utils;
mod parser;

use std::fs::read_to_string;

use crate::lexer::tokenise;
use crate::parser::parse;

fn main() {
    let input = match read_to_string("test.json") {
        Ok(x) => x,
        Err(e) => panic!("Could not open file: {}", e)
    }.replace("\r\n", "\n");

    let tokens = tokenise(input.as_str());

    parse(&tokens);

    println!("Worked flawlessly!");
    
}