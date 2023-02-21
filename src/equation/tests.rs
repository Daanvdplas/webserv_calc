#[cfg(test)]
mod test {

    use crate::equation::{self, Element, Equation};
    use crate::tokenizer::operator::Token;

    #[test]
    fn simple_tests() {
        let tokens = vec![Token::Number(32), Token::Operator('+'), Token::Number(2)];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(32),
                Element::Number(2),
                Element::Operator('+'),
            ])
        );

        let tokens = vec![
            Token::Number(32),
            Token::Operator('-'),
            Token::Number(2),
            Token::Operator('+'),
            Token::Number(2),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(32),
                Element::Number(2),
                Element::Operator('-'),
                Element::Number(2),
                Element::Operator('+'),
            ])
        );

        let tokens = vec![Token::Number(32), Token::Operator('*'), Token::Number(2)];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(32),
                Element::Number(2),
                Element::Operator('*'),
            ])
        );

        let tokens = vec![
            Token::Number(32),
            Token::Operator('*'),
            Token::Number(2),
            Token::Operator('*'),
            Token::Number(2),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(32),
                Element::Number(2),
                Element::Operator('*'),
                Element::Number(2),
                Element::Operator('*'),
            ])
        );

        let tokens = vec![
            Token::Number(5),
            Token::Operator('+'),
            Token::Number(32),
            Token::Operator('*'),
            Token::Number(2),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(5),
                Element::Number(32),
                Element::Number(2),
                Element::Operator('*'),
                Element::Operator('+'),
            ])
        );

        let tokens = vec![
            Token::Number(5),
            Token::Operator('+'),
            Token::Number(32),
            Token::Operator('*'),
            Token::Number(2),
            Token::Operator('^'),
            Token::Number(5),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(5),
                Element::Number(32),
                Element::Number(2),
                Element::Number(5),
                Element::Operator('^'),
                Element::Operator('*'),
                Element::Operator('+'),
            ])
        );

        let tokens = vec![
            Token::Number(5),
            Token::Operator('+'),
            Token::Number(32),
            Token::Operator('^'),
            Token::Number(14),
            Token::Operator('*'),
            Token::Number(2),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(5),
                Element::Number(32),
                Element::Number(14),
                Element::Operator('^'),
                Element::Number(2),
                Element::Operator('*'),
                Element::Operator('+'),
            ])
        );

        let tokens = vec![
            Token::Number(5),
            Token::Operator('^'),
            Token::Number(32),
            Token::Operator('^'),
            Token::Number(14),
            Token::Operator('+'),
            Token::Number(2),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(5),
                Element::Number(32),
                Element::Operator('^'),
                Element::Number(14),
                Element::Operator('^'),
                Element::Number(2),
                Element::Operator('+'),
            ])
        );

        let tokens = vec![
            Token::Number(5),
            Token::Operator('^'),
            Token::Number(32),
            Token::Operator('+'),
            Token::Number(14),
            Token::Operator('*'),
            Token::Number(2),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(5),
                Element::Number(32),
                Element::Operator('^'),
                Element::Number(14),
                Element::Number(2),
                Element::Operator('*'),
                Element::Operator('+'),
            ])
        );

        let tokens = vec![
            Token::Number(5),
            Token::Operator('*'),
            Token::Number(32),
            Token::Operator('+'),
            Token::Number(14),
            Token::Operator('^'),
            Token::Number(2),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(5),
                Element::Number(32),
                Element::Operator('*'),
                Element::Number(14),
                Element::Number(2),
                Element::Operator('^'),
                Element::Operator('+'),
            ])
        );
    }

    #[test]
    fn bracket_tests() {
        let tokens = vec![
            Token::Number(32),
            Token::Operator('+'),
            Token::Operator('('),
            Token::Number(2),
            Token::Operator(')'),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(32),
                Element::Equation(Equation(vec![Element::Number(2)])),
                Element::Operator('+'),
            ])
        );

        let tokens = vec![
            Token::Number(32),
            Token::Operator('-'),
            Token::Operator('('),
            Token::Number(2),
            Token::Operator('+'),
            Token::Number(2),
            Token::Operator(')'),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(32),
                Element::Equation(Equation(vec![
                    Element::Number(2),
                    Element::Number(2),
                    Element::Operator('+'),
                ])),
                Element::Operator('-'),
            ])
        );

        let tokens = vec![
            Token::Number(32),
            Token::Operator('*'),
            Token::Operator('('),
            Token::Number(2),
            Token::Operator(')'),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(32),
                Element::Equation(Equation(vec![Element::Number(2)])),
                Element::Operator('*'),
            ])
        );

        let tokens = vec![
            Token::Number(32),
            Token::Operator('*'),
            Token::Operator('('),
            Token::Number(2),
            Token::Operator('*'),
            Token::Number(2),
            Token::Operator(')'),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Number(32),
                Element::Equation(Equation(vec![
                    Element::Number(2),
                    Element::Number(2),
                    Element::Operator('*'),
                ])),
                Element::Operator('*'),
            ])
        );

        let tokens = vec![
            Token::Operator('('),
            Token::Number(5),
            Token::Operator('+'),
            Token::Number(32),
            Token::Operator(')'),
            Token::Operator('*'),
            Token::Number(2),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Equation(Equation(vec![
                    Element::Number(5),
                    Element::Number(32),
                    Element::Operator('+'),
                ])),
                Element::Number(2),
                Element::Operator('*'),
            ])
        );

        let tokens = vec![
            Token::Operator('('),
            Token::Operator('('),
            Token::Number(5),
            Token::Operator('+'),
            Token::Number(32),
            Token::Operator(')'),
            Token::Operator('*'),
            Token::Number(2),
            Token::Operator(')'),
            Token::Operator('^'),
            Token::Number(5),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Equation(Equation(vec![
                    Element::Equation(Equation(vec![
                        Element::Number(5),
                        Element::Number(32),
                        Element::Operator('+'),
                    ])),
                    Element::Number(2),
                    Element::Operator('*'),
                ])),
                Element::Number(5),
                Element::Operator('^'),
            ])
        );

        let tokens = vec![
            Token::Operator('('),
            Token::Number(5),
            Token::Operator('+'),
            Token::Number(32),
            Token::Operator('^'),
            Token::Number(14),
            Token::Operator(')'),
            Token::Operator('*'),
            Token::Number(2),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![
                Element::Equation(Equation(vec![
                    Element::Number(5),
                    Element::Number(32),
                    Element::Number(14),
                    Element::Operator('^'),
                    Element::Operator('+'),
                ])),
                Element::Number(2),
                Element::Operator('*'),
            ])
        );

        let tokens = vec![
            Token::Operator('('),
            Token::Number(5),
            Token::Operator('^'),
            Token::Number(32),
            Token::Operator('^'),
            Token::Operator('('),
            Token::Number(14),
            Token::Operator('+'),
            Token::Number(2),
            Token::Operator(')'),
            Token::Operator(')'),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![Element::Equation(Equation(vec![
                Element::Number(5),
                Element::Number(32),
                Element::Operator('^'),
                Element::Equation(Equation(vec![
                    Element::Number(14),
                    Element::Number(2),
                    Element::Operator('+'),
                ])),
                Element::Operator('^'),
            ]))]),
        );

        let tokens = vec![
            Token::Operator('('),
            Token::Number(5),
            Token::Operator('^'),
            Token::Operator('('),
            Token::Number(32),
            Token::Operator('+'),
            Token::Operator('('),
            Token::Number(14),
            Token::Operator('*'),
            Token::Number(2),
            Token::Operator(')'),
            Token::Operator(')'),
            Token::Operator(')'),
        ];
        let mut token_iter = tokens.into_iter();
        assert_eq!(
            equation::make(&mut token_iter),
            Equation(vec![Element::Equation(Equation(vec![
                Element::Number(5),
                Element::Equation(Equation(vec![
                    Element::Number(32),
                    Element::Equation(Equation(vec![
                        Element::Number(14),
                        Element::Number(2),
                        Element::Operator('*'),
                    ])),
                    Element::Operator('+'),
                ])),
                Element::Operator('^'),
            ]))])
        );
    }
}
