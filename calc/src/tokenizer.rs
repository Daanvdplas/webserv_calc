pub mod operator;

use operator::Token;

// To helpers for correct bracket use and negative numbers
pub struct Tokenizer {
    pub bracket_count: u8,
    pub neg_nbr: bool,
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {
            bracket_count: 0,
            neg_nbr: false,
        }
    }

    // Makes tokens of numbers and operators
    pub fn tokenizer(input: String) -> Result<Vec<Token>, &'static str> {
        let mut tokens = Vec::new();
        let mut token = Token::Begin;
        let mut tokenizer = Tokenizer::new();

        // Looping over input
        for x in input.chars() {
            if x == ' ' {
                continue;
            }
            let check = match x {
                '0'..='9' => Self::numbers(&mut tokens, token, x, &mut tokenizer),
                '+' | '-' | '*' | '/' | '^' | '(' | ')' => {
                    Self::operators(&mut tokens, token, x, &mut tokenizer)
                }
                // Found character that is not allowed
                _ => Err("Wrong input; only digits and operators (+ - * / ^ ( ))"),
            };
            match check {
                Ok(new_token) => token = new_token,
                Err(error) => return Err(error),
            }
        }
        match token {
            Token::Number(_) => tokens.push(token),
            Token::Operator(')') => {}
            _ => return Err("Error: can't end with operator"),
        }
        if tokenizer.bracket_count != 0 {
            return Err("Error: bracket count");
        }
        Ok(tokens)
    }

    // Found a nunber, therefor:
    //    - create a token
    //    - add it to an existing number token
    //    - error
    fn numbers(
        tokens: &mut Vec<Token>,
        token: Token,
        x: char,
        tokenizer: &mut Tokenizer,
    ) -> Result<Token, &'static str> {
        match token {
            Token::Begin => Ok(Token::Number(x as i64 - 48)),
            Token::Number(nbr) => {
                if nbr < 0 {
                    Ok(Token::Number(nbr * 10 - (x as i64 - 48)))
                } else {
                    Ok(Token::Number(nbr * 10 + (x as i64 - 48)))
                }
            }
            Token::Operator(operator) => {
                if operator == ')' {
                    Err("Error: immediate nbr after close bracket")
                } else if operator == '-' {
                    if tokenizer.neg_nbr == false {
                        tokens.push(token);
                        Ok(Token::Number(x as i64 - 48))
                    } else {
                        tokenizer.neg_nbr = false;
                        Ok(Token::Number(-1 * (x as i64 - 48)))
                    }
                } else {
                    tokens.push(token);
                    Ok(Token::Number(x as i64 - 48))
                }
            }
        }
    }

    // Found an operator, therefore:
    //    - push the current token and create a new one
    //    - error
    fn operators(
        tokens: &mut Vec<Token>,
        token: Token,
        x: char,
        tokenizer: &mut Tokenizer,
    ) -> Result<Token, &'static str> {
        let check = match x {
            '-' => operator::minus_check(token, &mut tokenizer.neg_nbr),
            '+' | '/' | '*' | '^' => operator::easy_operators_check(token),
            '(' => operator::open_bracket_check(
                token,
                x,
                &mut tokenizer.bracket_count,
                &tokenizer.neg_nbr,
            ),
            ')' => operator::close_bracket_check(token, x, &mut tokenizer.bracket_count),
            _ => panic!("dev fault"),
        };

        match check {
            Ok(()) => {
                if token != Token::Begin {
                    tokens.push(token);
                }
                Ok(Token::Operator(x))
            }
            Err(error) => Err(error),
        }
    }
}

mod tests;
