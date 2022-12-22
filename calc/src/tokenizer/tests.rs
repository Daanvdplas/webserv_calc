#[cfg(test)]
mod test {

    use crate::tokenizer::*;

    #[test]
    fn simple_tests() {
        assert_eq!(
            Tokenizer::tokenizer(String::from("234 + 4 ^ 3 / 2")),
            Ok(vec![
                Token::Number(234),
                Token::Operator('+'),
                Token::Number(4),
                Token::Operator('^'),
                Token::Number(3),
                Token::Operator('/'),
                Token::Number(2)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("234 + 4 ^ 3  / 2     ")),
            Ok(vec![
                Token::Number(234),
                Token::Operator('+'),
                Token::Number(4),
                Token::Operator('^'),
                Token::Number(3),
                Token::Operator('/'),
                Token::Number(2)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("234+4^3/2")),
            Ok(vec![
                Token::Number(234),
                Token::Operator('+'),
                Token::Number(4),
                Token::Operator('^'),
                Token::Number(3),
                Token::Operator('/'),
                Token::Number(2)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("- 234+-4^3/2")),
            Ok(vec![
                Token::Number(-234),
                Token::Operator('+'),
                Token::Number(-4),
                Token::Operator('^'),
                Token::Number(3),
                Token::Operator('/'),
                Token::Number(2)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("-- 234+-4^3/2")),
            Err("Error: too many minusses")
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("234+--4^3/2")),
            Err("Error: too many minusses")
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("234+-+4^3/2")),
            Err("Error: found an operator combination which is not allowed")
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("234++4^3/2")),
            Err("Error: found an operator combination which is not allowed")
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("234+4^/3/2")),
            Err("Error: found an operator combination which is not allowed")
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("*234++4^3/2")),
            Err("Error: can't start with this operator")
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("234+-4^3/2 +")),
            Err("Error: can't end with operator")
        );
    }

    #[test]
    fn bracket_tests() {
        assert_eq!(
            Tokenizer::tokenizer(String::from("234 + (323- 1 * 4) / 2")),
            Ok(vec![
                Token::Number(234),
                Token::Operator('+'),
                Token::Operator('('),
                Token::Number(323),
                Token::Operator('-'),
                Token::Number(1),
                Token::Operator('*'),
                Token::Number(4),
                Token::Operator(')'),
                Token::Operator('/'),
                Token::Number(2)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("234 + (323- (1 * 4)) / 2")),
            Ok(vec![
                Token::Number(234),
                Token::Operator('+'),
                Token::Operator('('),
                Token::Number(323),
                Token::Operator('-'),
                Token::Operator('('),
                Token::Number(1),
                Token::Operator('*'),
                Token::Number(4),
                Token::Operator(')'),
                Token::Operator(')'),
                Token::Operator('/'),
                Token::Number(2)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("((234 + (323)- 1) * 4) / 2")),
            Ok(vec![
                Token::Operator('('),
                Token::Operator('('),
                Token::Number(234),
                Token::Operator('+'),
                Token::Operator('('),
                Token::Number(323),
                Token::Operator(')'),
                Token::Operator('-'),
                Token::Number(1),
                Token::Operator(')'),
                Token::Operator('*'),
                Token::Number(4),
                Token::Operator(')'),
                Token::Operator('/'),
                Token::Number(2)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("(234 + (323- 1 * 4) / 2")),
            Err("Error: bracket count")
        );
        assert_eq!(
            Tokenizer::tokenizer(String::from("234 + 323- 1 * 4) / 2")),
            Err("Error: too many closing brackets")
        );
        assert_eq!(
            Tokenizer::tokenizer(String::from("234 + (323- 1 * 4 / 2")),
            Err("Error: bracket count")
        );
        assert_eq!(
            Tokenizer::tokenizer(String::from("(234 + 323- 1 * 4)) / 2")),
            Err("Error: too many closing brackets")
        );
        assert_eq!(
            Tokenizer::tokenizer(String::from("(234 + (323- 1 * 4)  2")),
            Err("Error: immediate nbr after close bracket")
        );
    }

    #[test]
    fn new_tests() {
        assert_eq!(
            Tokenizer::tokenizer(String::from("(- 234 + (323 ^ (4 - 1) + 2)) * 4")),
            Ok(vec![
                Token::Operator('('),
                Token::Number(-234),
                Token::Operator('+'),
                Token::Operator('('),
                Token::Number(323),
                Token::Operator('^'),
                Token::Operator('('),
                Token::Number(4),
                Token::Operator('-'),
                Token::Number(1),
                Token::Operator(')'),
                Token::Operator('+'),
                Token::Number(2),
                Token::Operator(')'),
                Token::Operator(')'),
                Token::Operator('*'),
                Token::Number(4)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("500 + - (5)")),
            Err("Error: found two operators and an open bracket")
        );
    }
}
