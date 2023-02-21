pub mod stack;

use crate::equation::Element;
use crate::executor::{self, stack::Stack};

pub fn run(eq: Vec<Element>) -> Result<i64, &'static str> {
    let mut stack: Stack = Stack::new();
    for elem in eq.into_iter() {
        match elem {
            Element::Number(nbr) => {
                stack.0.push(nbr);
                Ok(())
            }
            Element::Operator(operater) => stack.operations(operater),
            Element::Equation(eq) => match executor::run(eq.0) {
                Ok(res) => {
                    stack.0.push(res);
                    Ok(())
                }
                Err(error) => Err(error),
            },
        }?
    }
    Ok(stack.0.pop().unwrap())
}
