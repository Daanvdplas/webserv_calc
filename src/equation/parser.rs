use crate::equation::{self, Element, Equation};
use crate::tokenizer::operator::Token;

pub struct Parser {
    sub_add: Vec<Element>,
    mul_div: Vec<Element>,
    square: Vec<Element>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            sub_add: Vec::new(),
            mul_div: Vec::new(),
            square: Vec::new(),
        }
    }

    pub fn clean(&mut self, eq: &mut Equation) {
        Self::clean_specific(&mut self.square, eq);
        Self::clean_specific(&mut self.mul_div, eq);
        Self::clean_specific(&mut self.sub_add, eq);
    }

    pub fn clean_specific(stack: &mut Vec<Element>, eq: &mut Equation) {
        while let Some(pop) = stack.pop() {
            eq.0.push(pop);
        }
    }

    pub fn next<I>(&mut self, eq: &mut Equation, token: Token, token_iter: &mut I) -> bool
    where
        I: Iterator<Item = Token>,
    {
        match token {
            Token::Number(nbr) => eq.0.push(Element::Number(nbr as i64)),
            Token::Operator('(') => eq.0.push(Element::Equation(equation::make(token_iter))),
            Token::Operator(')') => return true,
            Token::Operator(operator) => self.operators(eq, operator),
            _ => todo!(),
        }
        false
    }

    pub fn operators(&mut self, eq: &mut Equation, operator: char) {
        match operator {
            '+' | '-' => self.push_sub_add(eq, operator),
            '*' | '/' => self.push_mul_div(eq, operator),
            '^' => self.push_square(eq),
            _ => todo!(),
        }
    }

    pub fn push_sub_add(&mut self, eq: &mut Equation, operator: char) {
        if let Some(last) = self.square.pop() {
            eq.0.push(last);
        }
        if let Some(last) = self.mul_div.pop() {
            eq.0.push(last);
        }
        if let Some(last) = self.sub_add.pop() {
            eq.0.push(last);
        }
        if '+' == operator {
            self.sub_add = vec![Element::Operator('+')];
        } else {
            self.sub_add = vec![Element::Operator('-')];
        }
    }

    pub fn push_mul_div(&mut self, eq: &mut Equation, operator: char) {
        if let Some(last) = self.square.pop() {
            eq.0.push(last);
        }
        if let Some(last) = self.mul_div.pop() {
            eq.0.push(last);
        }
        if '*' == operator {
            self.mul_div = vec![Element::Operator('*')];
        } else {
            self.mul_div = vec![Element::Operator('/')];
        }
    }

    pub fn push_square(&mut self, eq: &mut Equation) {
        if let Some(last) = self.square.pop() {
            eq.0.push(last);
        }
        self.square = vec![Element::Operator('^')];
    }
}
