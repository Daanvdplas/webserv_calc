#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {
    Begin,
    Operator(Operator),
    Number(i64),
}

pub struct Info {
    pub bracket_count: u8,
    pub minus_amount: u8,
}

impl Info {
    pub fn new() -> Info {
        Info {
            bracket_count: 0,
            minus_amount: 0,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
    Square,
    OpenBracket,
    CloseBracket,
}

impl Operator {
    pub fn create(x: char) -> Token {
        match x {
            '-' => Token::Operator(Operator::Substract),
            '+' => Token::Operator(Operator::Add),
            '/' => Token::Operator(Operator::Divide),
            '*' => Token::Operator(Operator::Multiply),
            '^' => Token::Operator(Operator::Square),
            '(' => Token::Operator(Operator::OpenBracket),
            ')' => Token::Operator(Operator::CloseBracket),
            _ => panic!("dev fault"),
        }
    }

    pub fn easy_operators_check(token: Token) -> Result<(), &'static str> {
        match token {
            Token::Begin => Err("Error: can't start with this operator"),
            Token::Operator(_) => Err("Error: double operator 1"),
            Token::Number(_) => Ok(()),
        }
    }

    pub fn bracket_check(x: char, info: &mut Info) -> Result<(), &'static str> {
        if x == '(' {
            info.bracket_count += 1;
        } 
        if x == ')' {
            if info.bracket_count == 0 {
                return Err("Error: bracket count");
            }
            info.bracket_count -= 1;
        }
        Ok(())
    }

    pub fn open_bracket_check(token: Token, x: char, info: &mut Info) -> Result<(), &'static str> {
        match token {
            Token::Number(_) => Err("Error: open bracket after number"),
            Token::Operator(o) => {
                if o == Operator::OpenBracket || o == Operator::CloseBracket {
                    Self::bracket_check(x, info)
                } else {
                    Ok(())
                }
            },
            _ => Ok(())
        }
    }

    pub fn close_bracket_check(token: Token, x: char, info: &mut Info) -> Result<(), &'static str> {
        match token {
            Token::Operator(o) => {
                if o == Operator::OpenBracket || o == Operator::CloseBracket {
                    Self::bracket_check(x, info)
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }


    pub fn minus_check(token: Token, info: &mut Info) -> Result<(), &'static str> {
        match token {
            Token::Begin => {
                info.minus_amount += 1;
                Ok(())
            },
            Token::Operator(o) => {
                if o == Operator::OpenBracket {
                    Err("Error: double operator 2")
                } else if o == Operator::CloseBracket {
                    Ok(())
                } else {
                    info.minus_amount += 1;
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }
}









