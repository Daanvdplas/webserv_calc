#[cfg(test)]
mod test {

    use crate::tokenizer::*;

    #[test]
    fn simple_tests() {
        assert_eq!(
            Tokenizer::tokenizer(String::from("234 + 4 ^ 3 / 2")),
            Ok(vec![
                Token::Number(234),
                Token::Operator(Operator::Add),
                Token::Number(4),
                Token::Operator(Operator::Square),
                Token::Number(3),
                Token::Operator(Operator::Divide),
                Token::Number(2)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("234 + 4 ^ 3  / 2     ")),
            Ok(vec![
                Token::Number(234),
                Token::Operator(Operator::Add),
                Token::Number(4),
                Token::Operator(Operator::Square),
                Token::Number(3),
                Token::Operator(Operator::Divide),
                Token::Number(2)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("234+4^3/2")),
            Ok(vec![
                Token::Number(234),
                Token::Operator(Operator::Add),
                Token::Number(4),
                Token::Operator(Operator::Square),
                Token::Number(3),
                Token::Operator(Operator::Divide),
                Token::Number(2)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("- 234+-4^3/2")),
            Ok(vec![
                Token::Number(-234),
                Token::Operator(Operator::Add),
                Token::Number(-4),
                Token::Operator(Operator::Square),
                Token::Number(3),
                Token::Operator(Operator::Divide),
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
                Token::Operator(Operator::Add),
                Token::Operator(Operator::OpenBracket),
                Token::Number(323),
                Token::Operator(Operator::Substract),
                Token::Number(1),
                Token::Operator(Operator::Multiply),
                Token::Number(4),
                Token::Operator(Operator::CloseBracket),
                Token::Operator(Operator::Divide),
                Token::Number(2)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("234 + (323- (1 * 4)) / 2")),
            Ok(vec![
                Token::Number(234),
                Token::Operator(Operator::Add),
                Token::Operator(Operator::OpenBracket),
                Token::Number(323),
                Token::Operator(Operator::Substract),
                Token::Operator(Operator::OpenBracket),
                Token::Number(1),
                Token::Operator(Operator::Multiply),
                Token::Number(4),
                Token::Operator(Operator::CloseBracket),
                Token::Operator(Operator::CloseBracket),
                Token::Operator(Operator::Divide),
                Token::Number(2)
            ])
        );

        assert_eq!(
            Tokenizer::tokenizer(String::from("((234 + (323)- 1) * 4) / 2")),
            Ok(vec![
                Token::Operator(Operator::OpenBracket),
                Token::Operator(Operator::OpenBracket),
                Token::Number(234),
                Token::Operator(Operator::Add),
                Token::Operator(Operator::OpenBracket),
                Token::Number(323),
                Token::Operator(Operator::CloseBracket),
                Token::Operator(Operator::Substract),
                Token::Number(1),
                Token::Operator(Operator::CloseBracket),
                Token::Operator(Operator::Multiply),
                Token::Number(4),
                Token::Operator(Operator::CloseBracket),
                Token::Operator(Operator::Divide),
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
                Token::Operator(Operator::OpenBracket),
                Token::Number(-234),
                Token::Operator(Operator::Add),
                Token::Operator(Operator::OpenBracket),
                Token::Number(323),
                Token::Operator(Operator::Square),
                Token::Operator(Operator::OpenBracket),
                Token::Number(4),
                Token::Operator(Operator::Substract),
                Token::Number(1),
                Token::Operator(Operator::CloseBracket),
                Token::Operator(Operator::Add),
                Token::Number(2),
                Token::Operator(Operator::CloseBracket),
                Token::Operator(Operator::CloseBracket),
                Token::Operator(Operator::Multiply),
                Token::Number(4)
            ])
        );
    }
}
