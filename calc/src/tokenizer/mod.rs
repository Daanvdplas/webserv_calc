#[derive(PartialEq, Debug)]
pub enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
    Square,
    OpenBracket,
    ClosingBracket,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Begin,
    Minus,
    Operator(Operator),
    Number(i64),
}

pub fn tokenizer(input: String) -> Result<Vec<Token>, &'static str> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut token: Token = Token::Begin;
    let mut bracket_count: u8 = 0;

    for x in input.chars() {
        // Check if character is allowed
        if !"0123456789-+/*^ ()".contains(x) {
            return Err("Wrong input; only digits and operators (+ - * / ^ ( ))");
        }

        if x == ' ' { continue }
        
        // For numeric characters, add to token
        if "0123456789".contains(x) {
            match token {
                Token::Begin => token = Token::Number(x as i64 - 48),
                Token::Minus => token = Token::Number(-1 * (x as i64 - 48)),
                Token::Number(nbr) => token = Token::Number(nbr * 10 + (x as i64 - 48)),
                Token::Operator(ref operator) => {
                    if operator == &Operator::ClosingBracket { return Err("Immediate nbr after closing bracket"); }
                    else { 
                        tokens.push(token);
                        token = Token::Number(x as i64 - 48);
                    }
                },
            };
        }

        // For operator character
        else if "+-*/^()".contains(x) {

            match token {
                Token::Begin => { 
                    match x {
                        '-' => token = Token::Minus,
                        '(' => token = Token::Operator(Operator::OpenBracket),
                        _ => return Err("Error: 5"),
                    };
                },
                Token::Minus => return Err("Error: minus"),
                Token::Number(nbr) => {
                    tokens.push(token);
                    token = make_operator_token(x);
                },
                Token::Operator(ref operator) => match operator {
                    Operator::Substract | Operator::Add | Operator::Multiply | Operator::Divide | Operator::Square => {
                        if x == '-' { 
                            tokens.push(token);
                            token = Token::Minus;
                        } else { return Err("Error: double operator 1"); }
                    },
                    Operator::OpenBracket => {
                        if x == '(' {
                            tokens.push(token);
                            token = Token::Operator(Operator::OpenBracket);
                            bracket_count += 1;
                        } else { return Err("Error: double operator 2"); }
                    },
                    Operator::ClosingBracket => {
                        if x == ')' {
                            if !bracket_check(x, &mut bracket_count) { return Err("Error: bracket"); }
                            tokens.push(token);
                            token = Token::Operator(Operator::ClosingBracket);
                        } else { return Err("Error: double operator 3"); }
                    },
                },
            };
        }
    }
    if bracket_count != 0 {
        return Err("Error: bracket count");
    }
    println!("{tokens:?}");
    Ok(tokens)
}

fn make_operator_token(x: char) -> Token {
    match x {
        '-' => Token::Operator(Operator::Substract),
        '+' => Token::Operator(Operator::Add),
        '/' => Token::Operator(Operator::Divide),
        '*' => Token::Operator(Operator::Multiply),
        '^' => Token::Operator(Operator::Square),
        '(' => Token::Operator(Operator::OpenBracket),
        ')' => Token::Operator(Operator::ClosingBracket),
        _ => todo!(),
    }
}
fn bracket_check(x: char, bracket_count: &mut u8) -> bool {
    if x == '(' {
        *bracket_count += 1;
    } 
    if x == ')' {
        if *bracket_count == 0 {
            return false;
        }
        *bracket_count -= 1;
    }
    true
}

#cfg[test
