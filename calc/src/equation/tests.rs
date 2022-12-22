#[cfg(test)]
mod test {

    use crate::equation::{Element, Equation};
    use crate::tokenizer::operator::Token;

    #[test]
    fn simple_tests() {
        assert_eq!(
            Equation::make(vec![
                Token::Number(32),
                Token::Operator('+'),
                Token::Number(2)
            ]),
            Equation(vec![
                Element::Number(32),
                Element::Operator('+'),
                Element::Number(2)
            ])
        );

        assert_eq!(
            Equation::make(vec![
                Token::Number(32),
                Token::Operator('*'),
                Token::Number(2)
            ]),
            Equation(vec![
                Element::Number(32),
                Element::Operator('*'),
                Element::Number(2)
            ])
        );

        assert_eq!(
            Equation::make(vec![
                Token::Number(5),
                Token::Operator('+'),
                Token::Number(32),
                Token::Operator('*'),
                Token::Number(2),
            ]),
            Equation(vec![
                Element::Number(5),
                Element::Operator('+'),
                Element::Number(32),
                Element::Operator('*'),
                Element::Number(2),
            ])
        );
    }
}
