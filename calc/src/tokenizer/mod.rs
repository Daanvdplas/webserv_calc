#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
    Square,
    OpenBracket,
    ClosingBracket,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Begin,
    Minus(u8),
    Operator(Operator),
    Number(i64),
}

pub fn tokenizer(input: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut token: Token = Token::Begin;
    let mut bracket_count: u8 = 0;

    for x in input.chars() {
        // Check if character is allowed
        if !"0123456789-+/*^ ()".contains(x) {
            return Err("Wrong input; only digits and operators (+ - * / ^ ( ))".to_string());
        }

        if x == ' ' { continue }
        
        // For numeric characters, add to token
        if "0123456789".contains(x) {
            match token {
                Token::Begin => token = Token::Number(x as i64 - 48),
                Token::Minus(amount) => {
                    if amount % 2 == 0 { token = Token::Number(x as i64 - 48); }
                    else { token = Token::Number(-1 * (x as i64 - 48)); }
                },
                Token::Number(nbr) => {
                    if nbr < 0 {
                        token = Token::Number(nbr * 10 - (x as i64 - 48));
                    } else {
                        token = Token::Number(nbr * 10 + (x as i64 - 48));
                    }
                }
                Token::Operator(ref operator) => {
                    if operator == &Operator::ClosingBracket { return Err("Immediate nbr after closing bracket".to_string()); }
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
                        '-' => token = Token::Minus(1),
                        '(' => token = Token::Operator(Operator::OpenBracket),
                        _ => return Err("Error: can't start with this operator".to_string()),
                    };
                },
                Token::Minus(amount) => {
                    if x == '-' { token = Token::Minus(amount + 1); }
                    else { return Err("Error: minus".to_string()); }
                },
                Token::Number(nbr) => {
                    tokens.push(token);
                    token = make_operator_token(x);
                },
                Token::Operator(ref operator) => match operator {
                    Operator::Substract | Operator::Add | Operator::Multiply | Operator::Divide | Operator::Square => {
                        if x == '-' { 
                            tokens.push(token);
                            token = Token::Minus(1);
                        } else if x == '(' {
                            tokens.push(token);
                            token = Token::Operator(Operator::OpenBracket);
                        } else { return Err(format!("Error: double operator 1")); }
                    },
                    Operator::OpenBracket => {
                        if x == '(' {
                            tokens.push(token);
                            token = Token::Operator(Operator::OpenBracket);
                            bracket_count += 1;
                            println!("count: {bracket_count}");
                        } else { return Err("Error: double operator 2".to_string()); }
                    },
                    Operator::ClosingBracket => {
                        println!("{bracket_count}");
                        if x == ')' {
                            if !bracket_check(x, &mut bracket_count) { return Err("Error: inaccurate bracket usage".to_string()); }
                        } 
                        tokens.push(token);
                        token = make_operator_token(x);
                    },
                },
            };
        }
    }
    match token {
        Token::Number(_) => tokens.push(token),
        Token::Operator(Operator::ClosingBracket) => {},
        _ => return Err("Error: can't end with operator".to_string()),
    }
    if bracket_count != 0 {
        return Err("Error: bracket count".to_string());
    }
    //println!("{tokens:?}");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tests() {
        assert_eq!(tokenizer(String::from("234 + 4 ^ 3 / 2")), 
                   Ok(vec![Token::Number(234), Token::Operator(Operator::Add), Token::Number(4), Token::Operator(Operator::Square),
                    Token::Number(3), Token::Operator(Operator::Divide), Token::Number(2)]));

        assert_eq!(tokenizer(String::from("234 + 4 ^ 3  / 2     ")), 
                   Ok(vec![Token::Number(234), Token::Operator(Operator::Add), Token::Number(4), Token::Operator(Operator::Square),
                    Token::Number(3), Token::Operator(Operator::Divide), Token::Number(2)]));
        
        assert_eq!(tokenizer(String::from("234+4^3/2")), 
                   Ok(vec![Token::Number(234), Token::Operator(Operator::Add), Token::Number(4), Token::Operator(Operator::Square),
                    Token::Number(3), Token::Operator(Operator::Divide), Token::Number(2)]));

        assert_eq!(tokenizer(String::from("- 234+-4^3/2")), 
                   Ok(vec![Token::Number(-234), Token::Operator(Operator::Add), Token::Number(-4), Token::Operator(Operator::Square),
                    Token::Number(3), Token::Operator(Operator::Divide), Token::Number(2)]));

        assert_eq!(tokenizer(String::from("-- 234+-4^3/2")), 
                   Ok(vec![Token::Number(234), Token::Operator(Operator::Add), Token::Number(-4), Token::Operator(Operator::Square),
                    Token::Number(3), Token::Operator(Operator::Divide), Token::Number(2)]));

        assert_eq!(tokenizer(String::from("234+--4^3/2")), 
                   Ok(vec![Token::Number(234), Token::Operator(Operator::Add), Token::Number(4), Token::Operator(Operator::Square),
                    Token::Number(3), Token::Operator(Operator::Divide), Token::Number(2)]));

        assert_eq!(tokenizer(String::from("234+-+4^3/2")), Err(format!("Error: minus")));

        assert_eq!(tokenizer(String::from("234++4^3/2")), Err(format!("Error: double operator 1")));

        assert_eq!(tokenizer(String::from("234+4^/3/2")), Err(format!("Error: double operator 1")));

        assert_eq!(tokenizer(String::from("*234++4^3/2")), Err(format!("Error: can't start with this operator")));

        assert_eq!(tokenizer(String::from("234+--4^3/2 +")), Err(format!("Error: can't end with operator")));
    }

    #[test]
    fn bracket_tests() {
        assert_eq!(tokenizer(String::from("234 + (323- 1 * 4) / 2")), 
                   Ok(vec![Token::Number(234), Token::Operator(Operator::Add), Token::Operator(Operator::OpenBracket),
                    Token::Number(323), Token::Operator(Operator::Substract), Token::Number(1), Token::Operator(Operator::Multiply),
                    Token::Number(4), Token::Operator(Operator::ClosingBracket), Token::Operator(Operator::Divide), Token::Number(2)]));

        assert_eq!(tokenizer(String::from("234 + (323- (1 * 4)) / 2")), 
                   Ok(vec![Token::Number(234), Token::Operator(Operator::Add), Token::Operator(Operator::OpenBracket),
                    Token::Number(323), Token::Operator(Operator::Substract), Token::Operator(Operator::OpenBracket), Token::Number(1), Token::Operator(Operator::Multiply),
                    Token::Number(4), Token::Operator(Operator::ClosingBracket), Token::Operator(Operator::ClosingBracket), Token::Operator(Operator::Divide), Token::Number(2)]));

        assert_eq!(tokenizer(String::from("((234 + (323)- 1) * 4) / 2")), 
                   Ok(vec![Token::Operator(Operator::OpenBracket), Token::Operator(Operator::OpenBracket), Token::Number(234), 
                      Token::Operator(Operator::Add), Token::Operator(Operator::OpenBracket), Token::Number(323), Token::Operator(Operator::ClosingBracket), 
                      Token::Operator(Operator::Substract), Token::Number(1), Token::Operator(Operator::ClosingBracket), Token::Operator(Operator::Multiply), 
                      Token::Number(4), Token::Operator(Operator::ClosingBracket), Token::Operator(Operator::Divide), Token::Number(2)]));

        assert_eq!(tokenizer(String::from("(234 + (323- 1 * 4) / 2")), Err(format!("Error: inaccurate bracket use")));

        assert_eq!(tokenizer(String::from("(234 + (323- 1 * 4)  2")), Err(format!("Error: inaccurate bracket use")));
    }
}
