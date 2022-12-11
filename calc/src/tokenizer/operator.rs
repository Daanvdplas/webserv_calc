#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {
    // All the tokens
    //
    // Begin token we need for the start
    Begin,
    Operator(Operator),
    Number(i64),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Operator {
    // All the operators
    Add,
    Substract,
    Multiply,
    Divide,
    Square,
    OpenBracket,
    CloseBracket,
}

impl Operator {
    // Create a new operator token
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

    // Do the checks for +, -, /, *, ^ operators
    pub fn easy_operators_check(token: Token) -> Result<(), &'static str> {
        match token {
            Token::Begin => Err("Error: can't start with this operator"),
            Token::Operator(o) => {
                if o == Operator::CloseBracket {
                    Ok(())
                } else {
                    Err("Error: found an operator combination which is not allowed")
                }
            }
            Token::Number(_) => Ok(()),
        }
    }

    // Accounting of opening and closing brackets; each '(' will increase the count
    // and each ')' will decrease the count. The count can't be lower than 0.
    pub fn bracket_check(x: char, bracket_count: &mut u8) -> Result<(), &'static str> {
        if x == '(' {
            *bracket_count += 1;
        } else if x == ')' {
            if *bracket_count == 0 {
                return Err("Error: too many closing brackets");
            }
            *bracket_count -= 1;
        }
        Ok(())
    }

    // Do the checks for an open bracket.
    pub fn open_bracket_check(
        token: Token,
        x: char,
        bracket_count: &mut u8,
    ) -> Result<(), &'static str> {
        match token {
            Token::Number(_) => Err("Error: open bracket after number"),
            Token::Operator(o) => {
                if o == Operator::CloseBracket {
                    Err("Error: open bracket after close bracket")
                } else {
                    Self::bracket_check(x, bracket_count)
                }
            }
            _ => Self::bracket_check(x, bracket_count),
        }
    }

    // Do the checks for a closing bracket.
    pub fn close_bracket_check(
        token: Token,
        x: char,
        bracket_count: &mut u8,
    ) -> Result<(), &'static str> {
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
                    Self::bracket_check(x, bracket_count)
                }
            }
            _ => Self::bracket_check(x, bracket_count),
        }
    }

    // Do the checks for the minus operator. Finding '-' after the operator
    // (except ')' ) and begin token will turn on the minus flag for a followed
    // negative number.
    pub fn minus_check(token: Token, minus_count: &mut bool) -> Result<(), &'static str> {
        match token {
            Token::Operator(o) => {
                if o == Operator::OpenBracket
                    || o == Operator::Add
                    || o == Operator::Substract
                    || o == Operator::Multiply
                    || o == Operator::Divide
                    || o == Operator::Square
                {
                    if *minus_count == true {
                        return Err("Error: too many minusses");
                    }
                    *minus_count = true;
                }
                Ok(())
            }
            Token::Begin => {
                *minus_count = true;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
