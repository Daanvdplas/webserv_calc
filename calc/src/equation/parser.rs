use crate::equation::{Element, Equation};
use crate::tokenizer::operator::Token;

pub struct Parser {
    sub_add: Vec<Element>,
    stack2: Vec<Element>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            sub_add: Vec::new(),
            stack2: Vec::new(),
        }
    }

    pub fn next(self, eq: &mut Equation, token: Token) -> () {
        match token {
            Token::Number(nbr) => eq.0.push(Element::Number(nbr)),
            Token::Operator(operator) => self.operators(eq, operator),
            _ => todo!(),
        }
    }

    pub fn operators(self, eq: &mut Equation, operator: char) -> () {
        match operator {
            '+' | '-' => {
                self.push_sub_add(eq, operator);
            }
            // Operator::Multiply | Operator::Divide => {
            //     self.push_mul_div(operator);
            // },
            _ => todo!(),
        }
    }

    pub fn push_sub_add(self, eq: &mut Equation, operator: char) -> () {
        if self.sub_add.len() > 0 {
            eq.0.push(self.sub_add.pop().unwrap());
        }
        match operator {
            '+' => self.sub_add = vec![Element::Operator('+')],
            '-' => self.sub_add = vec![Element::Operator('-')],
            _ => panic!("dev fault"),
        }
    }

    //     pub fn mul_div<I>(self, mut token_iter: I, number: i64, operator: char) -> Option<I>
    //     where
    //         I: Iterator<Item = Token>,
    //     {
    //         match token_iter.next() {
    //             Some(Token::Number(nbr)) => match operator {
    //                 '/' => self.parse(token_iter, number / nbr),
    //                 '*' => self.parse(token_iter, number * nbr),
    //             },
    //             _ => todo!(),
    //         }
    //     }
}
