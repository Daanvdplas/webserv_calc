mod equation;
mod executor;
mod tokenizer;

use crate::tokenizer::Tokenizer;

pub fn run(input: String) -> Result<i64, &'static str> {
    match Tokenizer::tokenizer(input) {
        Ok(tokens) => {
            let mut token_iter = tokens.into_iter();
            let eq = equation::make(&mut token_iter);
            if let Ok(res) = executor::run(eq.0) {
                Ok(res)
            } else {
                Err("Error: Overflow")
            }
        }
        Err(message) => Err(message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lets_test() {
        let result = run(String::from("3 + 3 ^ 2"));
        assert_eq!(result, Ok(12));
    }
}
