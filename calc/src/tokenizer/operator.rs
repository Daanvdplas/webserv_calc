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
            Token::Operator(o) => {
                if o == Operator::CloseBracket {
                    Ok(())
                } else {
                    Err("Error: double operator 1")
                }
            }
            Token::Number(_) => Ok(()),
        }
    }

    pub fn bracket_check(x: char, info: &mut Info) -> Result<(), &'static str> {
        if x == '(' {
            info.bracket_count += 1;
        } else if x == ')' {
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
                if o == Operator::CloseBracket {
                    Err("Error: open bracket after close bracket")
                } else {
                    Self::bracket_check(x, info)
                }
            }
            _ => Self::bracket_check(x, info),
        }
    }

    pub fn close_bracket_check(token: Token, x: char, info: &mut Info) -> Result<(), &'static str> {
        match token {
            Token::Operator(o) => {
                if o == Operator::Add
                    || o == Operator::Substract
                    || o == Operator::Multiply
                    || o == Operator::Square
                    || o == Operator::Divide
                {
                    Err("Error: closing bracket after operator")
                } else {
                    Self::bracket_check(x, info)
                }
            }
            _ => Self::bracket_check(x, info),
        }
    }

    pub fn minus_check(token: Token, info: &mut Info) -> Result<(), &'static str> {
        match token {
            Token::Operator(o) => {
                if o == Operator::OpenBracket
                    || o == Operator::Add
                    || o == Operator::Substract
                    || o == Operator::Multiply
                    || o == Operator::Divide
                    || o == Operator::Square
                {
                    if info.minus_amount > 0 {
                        return Err("Error: too many minusses");
                    }
                    info.minus_amount += 1;
                }
                Ok(())
            }
            Token::Begin => {
                info.minus_amount += 1;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
