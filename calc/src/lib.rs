mod equation;
mod tokenizer;

use crate::equation::Equation;
use crate::tokenizer::Tokenizer;

pub fn run(input: String) -> () {
    match Tokenizer::tokenizer(input.clone()) {
        Ok(tokens) => {
            // Equation::make(tokens);
            //let result = executor(parsed_input);
            //println!("{result}");
        }
        Err(message) => println!("{message}"),
    }
}
