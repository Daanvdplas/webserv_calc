pub mod parser;

use crate::equation::parser::Parser;
use crate::tokenizer::operator::Token;

//pub struct MiniEquation {}
#[derive(PartialEq, Debug, Clone)]
pub enum Element {
    Number(i64),
    Operator(char),
    //MiniEquation(MiniEquation),
    // Equation(Equation),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Equation(Vec<Element>);

impl Equation {
    pub fn new() -> Self {
        Equation(Vec::new())
    }
}

pub fn make(tokens: Vec<Token>) -> Equation {
    let mut eq: Equation = Equation::new();
    let mut parser: Parser = Parser::new();
    let mut token_iter = tokens.into_iter();
    loop {
        if let Some(token) = token_iter.next() {
            parser.next(&mut eq, token)
        } else {
            break;
        }
    }
    eq
}

mod tests;
