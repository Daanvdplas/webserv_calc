use std::io;
use crate::tokenizer::*;

mod tokenizer;

fn main() {

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input);
    input.pop();
    match tokenizer(input) {
        Ok(tokens) => {
            //let parsed_input = parser(tokens);
            //let result = executor(parsed_input);
            //println!("{result}");
            println!("succes");

        },
        Err(message) => println!("{message}"),
    }
}
