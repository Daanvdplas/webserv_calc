mod operator;

use operator::{Token, Operator, Info};

pub fn tokenizer(input: String) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();
    let mut token = Token::Begin;
    let mut info = Info::new();

    for x in input.chars() {
        // Check if character is allowed
        if !"0123456789-+/*^ ()".contains(x) {
            return Err("Wrong input; only digits and operators (+ - * / ^ ( ))");
        }

        if x == ' ' { continue }
        
        // For numeric characters, add to token
        if "0123456789".contains(x) {
            match number(&mut tokens, token, x, &mut info) {
                Ok(new_token) => token = new_token,
                Err(error) => return Err( error),
            }
        }

        // For operator character
        else if "+-*/^()".contains(x) {
            match operator(&mut tokens, token, x, &mut info) {
                Ok(new_token) => token = new_token,
                Err(error) => return Err(error),
            }
        }
    }
    match token {
        Token::Number(_) => tokens.push(token),
        Token::Operator(Operator::CloseBracket) => {},
        _ => return Err("Error: can't end with operator"),
    }
    if info.bracket_count != 0 {
        return Err("Error: bracket count");
    }
    Ok(tokens)
}

fn number(tokens: &mut Vec<Token>, token: Token, x: char, info: &mut Info) -> Result<Token, &'static str> {
    match token {
        Token::Begin => Ok(Token::Number(x as i64 - 48)),
        Token::Number(nbr) => {
            if nbr < 0 {
                Ok(Token::Number(nbr * 10 - (x as i64 - 48)))
            } else {
                Ok(Token::Number(nbr * 10 + (x as i64 - 48)))
            }
        }
        Token::Operator(ref operator) => {
            if operator == &Operator::CloseBracket {
                Err("Immediate nbr after closing bracket")
            } else if operator == &Operator::Substract {
                if info.minus_amount % 2 == 0 {
                    Ok(Token::Number(x as i64 - 48))
                } else {
                    Ok(Token::Number(-1 * (x as i64 - 48)))
                }
            } else { 
                tokens.push(token);
                Ok(Token::Number(x as i64 - 48))
            }
        },
    }
}

fn operator(tokens: &mut Vec<Token>, token: Token, x: char, info: &mut Info) -> Result<Token, &'static str> {
    let check = match x {
        '-' => Operator::minus_check(token, info),
        '+' | '/' | '*' | '^' => Operator::easy_operators_check(token),
        '(' => Operator::open_bracket_check(token, x, info),
        ')' => Operator::close_bracket_check(token, x, info),
        _ => panic!("dev fault"),
    };

    match check {
        Ok(()) => {
            tokens.push(token);
            Ok(Operator::create(x))
        },
        Err(error) => Err(error),
    }
}

mod tests;
