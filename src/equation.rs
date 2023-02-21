pub mod parser;

use crate::equation::parser::Parser;
use crate::tokenizer::operator::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum Element {
    Number(i64),
    Operator(char),
    Equation(Equation),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Equation(pub Vec<Element>);

impl Equation {
    pub fn new() -> Self {
        Equation(Vec::new())
    }
}

pub fn make<I>(token_iter: &mut I) -> Equation
where
    I: Iterator<Item = Token>,
{
    let mut eq: Equation = Equation::new();
    let mut parser: Parser = Parser::new();
    while let Some(token) = token_iter.next() {
        if parser.next(&mut eq, token, token_iter) {
            break;
        }
    }
    parser.clean(&mut eq);
    eq
}

mod tests;
