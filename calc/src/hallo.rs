
match token {
    Token::Begin => { 
        match x {
            '-' => token = Token::Minus(1),
            '(' => token = Operator::make(x),
            _ => return Err("Error: can't start with this operator".to_string()),
        };
    },
    Token::Minus(amount) => {
        if x == '-' { token = Token::Minus(amount + 1); }
        else { return Err("Error: minus".to_string()); }
    },
    Token::Number(nbr) => {
        tokens.push(token);
        token = Operator::make(x);
    },
    Token::Operator(ref operator) => match operator {
        Operator::Substract | Operator::Add | Operator::Multiply | Operator::Divide | Operator::Square => {
            if x == '-' { 
                tokens.push(token);
                token = Token::Minus(1);
            } else if x == '(' {
                tokens.push(token);
                token = Operator::make(x)
            } else { return Err(format!("Error: double operator 1")); }
        },
        Operator::OpenBracket => {
            if x == '(' {
                tokens.push(token);
                token = Operator::make(x)
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
            token = Operator::make(x);
        },
    },
};
